//! Hyperbolic Projections and Visual Renderers
//!
//! Provides Poincaré and Klein disk projections, ASCII terminal visualization,
//! and standalone SVG vector graphic rendering of the hyperbolic world.

use crate::content::*;
use crate::engine::*;
use crate::math::*;
use crate::tiling::*;
use std::collections::{HashMap, VecDeque};

/// Cell position and projected vertices in the player's camera view
#[derive(Clone, Debug)]
pub struct ProjectedCell {
    pub cell_id: CellId,
    pub center_hyper: HyperPoint,
    pub center_disk: [f64; 2],
    pub vertices_disk: Vec<[f64; 2]>,
}

/// Computes the camera transformation for cells within sight of the player
pub struct CameraView {
    pub cells: HashMap<CellId, ProjectedCell>,
}

impl CameraView {
    /// Compute projected coordinates for all cells within view_dist of player
    pub fn compute(tiling: &mut Tiling, player_cell: CellId, view_dist: i32, alpha: f64) -> Self {
        let mut rel_matrices: HashMap<CellId, TransMatrix> = HashMap::new();
        let mut q = VecDeque::new();
        let mut cells = HashMap::new();

        rel_matrices.insert(player_cell, ID);
        q.push_back(player_cell);

        while let Some(c) = q.pop_front() {
            let m_c = rel_matrices[&c];
            let dist = tiling.cells[c].cpdist;

            // Center of cell c in player frame
            let center_h = m_c * C0;
            let center_disk = hybrid_disk(center_h, alpha);

            // Vertices of cell c in player frame
            let deg = tiling.cells[c].degree();
            let mut vertices_disk = Vec::with_capacity(deg);
            for k in 0..deg {
                let v_local = tiling.get_corner(c, k);
                let v_h = m_c * v_local;
                vertices_disk.push(hybrid_disk(v_h, alpha));
            }

            cells.insert(
                c,
                ProjectedCell {
                    cell_id: c,
                    center_hyper: center_h,
                    center_disk,
                    vertices_disk,
                },
            );

            if dist < view_dist {
                for i in 0..deg {
                    let n = tiling.create_mov(c, i);
                    if !rel_matrices.contains_key(&n) {
                        // Matrix chaining: M_neighbor = M_c * adj(c, i)
                        let adj_mat = tiling.adj(c, i);
                        let m_n = m_c * adj_mat;
                        rel_matrices.insert(n, m_n);
                        q.push_back(n);
                    }
                }
            }
        }

        Self { cells }
    }
}

/// Renderer for ASCII terminal display and SVG export
pub struct Renderer;

impl Renderer {
    /// Render current game state to an ASCII string buffer
    pub fn render_ascii(engine: &mut GameEngine, width: usize, height: usize) -> String {
        let player_c = engine.player.walker.at;
        let view = CameraView::compute(&mut engine.world.tiling, player_c, 5, 1.0);

        let mut grid = vec![vec![' '; width]; height];

        // Draw Poincaré disk circular boundary
        let cx = (width as f64 - 1.0) / 2.0;
        let cy = (height as f64 - 1.0) / 2.0;
        let rx = cx * 0.95;
        let ry = cy * 0.95;

        for y in 0..height {
            for x in 0..width {
                let nx = (x as f64 - cx) / rx;
                let ny = (y as f64 - cy) / ry;
                let r2 = nx * nx + ny * ny;

                if (r2 - 1.0).abs() < 0.08 {
                    grid[y][x] = '#'; // Boundary circle
                }
            }
        }

        // Plot projected cell centers
        for (c, proj) in &view.cells {
            let px = cx + proj.center_disk[0] * rx;
            let py = cy + proj.center_disk[1] * ry;

            let col = px.round() as usize;
            let row = py.round() as usize;

            if col < width && row < height {
                let b = &engine.world.board[*c];
                let ch = if b.monster != Monster::None {
                    b.monster.info().glyph
                } else if b.item != Item::None {
                    b.item.info().glyph
                } else if b.wall != Wall::None {
                    b.wall.info().glyph
                } else if engine.world.tiling.cells[*c].cell_type == CellType::Heptagon {
                    '.'
                } else {
                    ','
                };
                grid[row][col] = ch;
            }
        }

        // Build string with HUD status bar
        let mut output = String::new();
        let cur_land = engine.world.board[player_c].land;
        output.push_str(&format!(
            "--- HyperRogue | Land: {} | Turn: {} | Treasures: {} | Kills: {} ---\n",
            cur_land.info().name,
            engine.turn_count,
            engine.player.total_treasures(),
            engine.player.total_kills()
        ));

        for row in grid {
            output.push_str(&row.into_iter().collect::<String>());
            output.push('\n');
        }

        if let Some(msg) = engine.message_log.last() {
            output.push_str(&format!("Msg: {}\n", msg));
        }

        output
    }

    /// Render current game state to an SVG vector graphic
    pub fn render_svg(engine: &mut GameEngine, size: u32) -> String {
        let player_c = engine.player.walker.at;
        let view = CameraView::compute(&mut engine.world.tiling, player_c, 5, 1.0);

        let center = (size as f64) / 2.0;
        let radius = center * 0.92;
        let mut svg = format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {size} {size}" width="{size}" height="{size}">
<rect width="100%" height="100%" fill="#101018"/>
<circle cx="{center}" cy="{center}" r="{radius}" fill="#1a1a24" stroke="#4a4a60" stroke-width="2"/>
"##
        );

        // Sort cells back-to-front by hyperbolic distance from player
        let mut sorted_cells: Vec<_> = view.cells.values().collect();
        sorted_cells.sort_by(|a, b| {
            hdist0(b.center_hyper)
                .partial_cmp(&hdist0(a.center_hyper))
                .unwrap()
        });

        // Draw polygon tiles
        for proj in &sorted_cells {
            let c = proj.cell_id;
            let b = &engine.world.board[c];

            let fill_color = match b.wall {
                Wall::Icewall => "#6080e0",
                Wall::Barrier => "#b05000",
                Wall::Cavewall => "#206040",
                Wall::Dune => "#c0a070",
                Wall::FloorA => "#801030",
                Wall::FloorB => "#203090",
                Wall::Fire => "#e03000",
                Wall::DeadTroll => "#a02020",
                _ => {
                    if b.land == Land::Ice {
                        "#182040"
                    } else if b.land == Land::Caves {
                        "#182820"
                    } else if b.land == Land::Desert {
                        "#383020"
                    } else {
                        "#242430"
                    }
                }
            };

            let points: Vec<String> = proj
                .vertices_disk
                .iter()
                .map(|p| format!("{:.1},{:.1}", center + p[0] * radius, center + p[1] * radius))
                .collect();

            svg.push_str(&format!(
                r##"<polygon points="{}" fill="{}" stroke="#303040" stroke-width="0.8"/>
"##,
                points.join(" "),
                fill_color
            ));

            // Draw glyphs for entities
            let (glyph, col) = if b.monster != Monster::None {
                (b.monster.info().glyph, "#ffffff")
            } else if b.item != Item::None {
                (b.item.info().glyph, "#ffd700")
            } else {
                continue;
            };

            let gx = center + proj.center_disk[0] * radius;
            let gy = center + proj.center_disk[1] * radius;
            svg.push_str(&format!(
                r#"<text x="{:.1}" y="{:.1}" font-size="14" font-weight="bold" fill="{}" text-anchor="middle" dominant-baseline="central">{}</text>
"#,
                gx, gy, col, glyph
            ));
        }

        svg.push_str("</svg>\n");
        svg
    }
}

// ---------------------------------------------------------------------------
// Tests and Validations
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_view_origin_centered() {
        let mut tiling = Tiling::new();
        let view = CameraView::compute(&mut tiling, 0, 3, 1.0);

        // Player cell at 0 is exactly at origin (0, 0)
        let player_proj = &view.cells[&0];
        assert!(player_proj.center_disk[0].abs() < 1e-10);
        assert!(player_proj.center_disk[1].abs() < 1e-10);

        // All projected disk coordinates must lie strictly inside unit circle r < 1.0
        for (_, p) in &view.cells {
            let r2 = p.center_disk[0].powi(2) + p.center_disk[1].powi(2);
            assert!(r2 < 1.0, "Point outside unit disk: {:?}", p.center_disk);
        }
    }

    #[test]
    fn test_ascii_render_contains_player() {
        let mut engine = GameEngine::new(42);
        let ascii = Renderer::render_ascii(&mut engine, 40, 20);
        assert!(ascii.contains('@'), "ASCII frame must contain player '@'");
        assert!(ascii.contains("Crossroads"), "HUD must show current land");
    }

    #[test]
    fn test_svg_render_valid() {
        let mut engine = GameEngine::new(42);
        let svg = Renderer::render_svg(&mut engine, 500);
        assert!(svg.starts_with("<svg"));
        assert!(svg.ends_with("</svg>\n"));
        assert!(svg.contains("<polygon"));
        assert!(svg.contains("<circle"));
    }
}
