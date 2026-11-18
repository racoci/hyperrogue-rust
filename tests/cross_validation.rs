//! Comprehensive Integration Tests and Cross-Validation against Original C++ HyperRogue

use hyperrogue_rs::content::*;
use hyperrogue_rs::engine::*;
use hyperrogue_rs::math::*;
use hyperrogue_rs::render::*;
use hyperrogue_rs::sim::*;
use hyperrogue_rs::tiling::*;
use hyperrogue_rs::worldgen::*;
use std::collections::{HashMap, VecDeque};
use std::f64::consts::PI;

#[test]
fn test_cpp_cross_val_hyperbolic_constants() {
    // Golden constants computed directly by C++ hyperrogue (geometry.cpp, hyperpoint.cpp)
    let alpha = TAU / 7.0;
    let beta = TAU / 3.0;

    let tessf = edge_of_triangle_with_angles(beta, PI / 7.0, PI / 7.0);
    let hcrossf = edge_of_triangle_with_angles(PI / 2.0, PI / 7.0, beta / 2.0);

    // C++ reference:
    // tessf = 1.0905496635070873
    // hcrossf = 0.6206717375563866
    assert!(
        (tessf - 1.0905496635070873).abs() < 1e-12,
        "tessf {} vs C++ golden 1.0905496635070873",
        tessf
    );
    assert!(
        (hcrossf - 0.6206717375563866).abs() < 1e-12,
        "hcrossf {} vs C++ golden 0.6206717375563866",
        hcrossf
    );

    let geom = GeometryParams::compute_standard_hyperbolic();
    // C++ reference:
    // hexf = 0.37807724532031972
    // rhexf = 0.37807724532031972
    // edgelen = 0.33439517286644771
    // hexvdist = 0.32993663988712651
    // hexhexdist = 0.56625630673531557
    assert!(
        (geom.hexf - 0.37807724532031972).abs() < 1e-12,
        "hexf mismatch: {}",
        geom.hexf
    );
    assert!(
        (geom.edgelen - 0.33439517286644771).abs() < 1e-12,
        "edgelen mismatch: {}",
        geom.edgelen
    );
    assert!(
        (geom.hexvdist - 0.32993663988712651).abs() < 1e-12,
        "hexvdist mismatch: {}",
        geom.hexvdist
    );
    assert!(
        (geom.hexhexdist - 0.56625630673531557).abs() < 1e-12,
        "hexhexdist mismatch: {}",
        geom.hexhexdist
    );
}

#[test]
fn test_cpp_cross_val_heptagonal_tree_counts() {
    // In C++ hyperrogue (heptagon.cpp), the regular {7, 3} tiling tree automaton produces:
    // Distance 0: 1
    // Distance 1: 7
    // Distance 2: 21
    // Distance 3: 56
    // Distance 4: 147
    // Total cumulative at distance 4: 232 heptagons
    let mut tiling = Tiling::new();

    let mut q = VecDeque::new();
    q.push_back(0);

    while let Some(h) = q.pop_front() {
        if tiling.hepts[h].distance < 4 {
            for d in 0..7 {
                let n = tiling.create_step(h, d);
                if tiling.hepts[n].distance == tiling.hepts[h].distance + 1 {
                    q.push_back(n);
                }
            }
        }
    }

    let mut counts = HashMap::new();
    for h in &tiling.hepts {
        *counts.entry(h.distance).or_insert(0) += 1;
    }

    assert_eq!(counts[&0], 1, "Distance 0 count");
    assert_eq!(counts[&1], 7, "Distance 1 count");
    assert_eq!(counts[&2], 21, "Distance 2 count");
    assert_eq!(counts[&3], 56, "Distance 3 count");
    assert_eq!(counts[&4], 147, "Distance 4 count");
    assert_eq!(tiling.hept_count(), 232, "Total heptagons at radius 4");
}

#[test]
fn test_cpp_cross_val_bitruncated_cell_counts() {
    // In C++ hyperrogue (cell.cpp), bitruncated {7, 3} grid BFS cumulative counts:
    // Dist 0: 1 (cumulative: 1)
    // Dist 1: 7 (cumulative: 8)
    // Dist 2: 14 (cumulative: 22)
    // Dist 3: 28 (cumulative: 50)
    // Dist 4: 49 (cumulative: 99)
    // Dist 5: 84 (cumulative: 183)
    let mut tiling = Tiling::new();
    tiling.expand_disk(5);

    let mut counts = HashMap::new();
    for c in &tiling.cells {
        if c.cpdist <= 5 {
            *counts.entry(c.cpdist).or_insert(0) += 1;
        }
    }

    assert_eq!(counts[&0], 1);
    assert_eq!(counts[&1], 7);
    assert_eq!(counts[&2], 14);
    assert_eq!(counts[&3], 28);
    assert_eq!(counts[&4], 49);
    assert_eq!(counts[&5], 84);

    let cum_5: i32 = (0..=5).map(|d| counts[&d]).sum();
    assert_eq!(cum_5, 183);
}

#[test]
fn test_cpp_cross_val_connection_symmetry() {
    // Every neighbor link in both heptagon tree and cell graph must be strictly symmetric:
    // A.moves[d] = B with spin s <=> B.moves[s] = A with spin d
    let mut tiling = Tiling::new();
    tiling.expand_disk(4);

    // Heptagons
    for h in 0..tiling.hept_count() {
        for d in 0..7 {
            if let Some(n) = tiling.hepts[h].moves[d] {
                let s = tiling.hepts[h].spins[d] as usize;
                assert_eq!(tiling.hepts[n].moves[s], Some(h));
                assert_eq!(tiling.hepts[n].spins[s], d as u8);
            }
        }
    }

    // Cells
    for c in 0..tiling.cell_count() {
        let deg = tiling.cells[c].degree();
        for d in 0..deg {
            if let Some(n) = tiling.cells[c].moves[d] {
                let s = tiling.cells[c].spins[d] as usize;
                assert_eq!(tiling.cells[n].moves[s], Some(c));
                assert_eq!(tiling.cells[n].spins[s], d as u8);
            }
        }
    }
}

#[test]
fn test_cpp_cross_val_simulation_gameplay_flow() {
    // End-to-end simulation of 50 turns
    let mut engine = GameEngine::new(999);

    for _ in 0..50 {
        if engine.player.is_dead || engine.player.won {
            break;
        }

        let curr = engine.player.walker.at;
        let deg = engine.world.tiling.cells[curr].degree();
        let mut acted = false;

        for d in 0..deg {
            let next_c = engine.world.tiling.create_mov(curr, d);
            if engine.is_passable(next_c, false) {
                acted = engine.process_player_action(PlayerAction::Step(d));
                if acted {
                    break;
                }
            }
        }

        if !acted {
            engine.process_player_action(PlayerAction::Wait);
        }
    }

    assert!(engine.turn_count > 0, "Simulation advanced turns");

    // ASCII render check
    let frame = Renderer::render_ascii(&mut engine, 50, 20);
    assert!(!frame.is_empty());

    // SVG render check
    let svg = Renderer::render_svg(&mut engine, 400);
    assert!(svg.starts_with("<svg"));
}
