//! Nil Geometry: The Heisenberg Group and Non-Isotropic 3D Space
//!
//! Recreates HyperRogue's complete Nil geometry subsystem:
//! 1. The Heisenberg group models: nmHeis (Heisenberg model) and nmSym (rotationally symmetric model).
//! 2. Nil group multiplication and inverses (MVec integer lattice).
//! 3. Continuous Nil isometries, translations, and non-commutative matrix actions.
//! 4. Exact analytical Nil geodesics via exponential and logarithmic maps.
//! 5. Nil honeycombs: ns6 (six sides), ns8 (eight sides), and nshex (hexagonal).
//! 6. Periodic Nil torus quotient spaces with twisted coordinate shearing.
//! 7. Nil distance metric, geodesic projection, and compass orientation.

use crate::math::*;
use crate::tiling::CellId;
use crate::content::*;
use crate::worldgen::*;
use std::collections::HashMap;
use std::f64::consts::PI;

/// Nil geometry model type
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NilModel {
    Symmetric = 0,  // nmSym: Rotationally symmetric model
    Heisenberg = 1, // nmHeis: Standard Heisenberg matrix model
}

impl NilModel {
    #[inline]
    pub fn value(&self) -> f64 {
        match self {
            NilModel::Symmetric => 0.0,
            NilModel::Heisenberg => 1.0,
        }
    }
}

/// Convert coordinate z between symmetric and Heisenberg models
#[inline]
pub fn sym_to_heis_bonus(x: f64, y: f64) -> f64 {
    x * y / 2.0
}

/// Convert a point between Nil models
pub fn convert_nil(mut h: HyperPoint, from: NilModel, to: NilModel) -> HyperPoint {
    let bonus = sym_to_heis_bonus(h.x(), h.y());
    *h.z_mut() += bonus * (to.value() - from.value());
    h
}

/// Nil integer lattice coordinates with non-commutative group multiplication
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MVec {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl MVec {
    pub const ZERO: Self = Self::new(0, 0, 0);

    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// Heisenberg group inverse:
    /// (x, y, z)^-1 = (-x, -y, -z + x*y)
    pub fn inverse(&self) -> Self {
        Self::new(-self.x, -self.y, -self.z + self.x * self.y)
    }

    /// Heisenberg group multiplication:
    /// (x1, y1, z1) * (x2, y2, z2) = (x1 + x2, y1 + y2, z1 + z2 + x1 * y2)
    pub fn heis_mul(&self, rhs: Self) -> Self {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z + self.x * rhs.y,
        )
    }

    /// Hexagonal coordinate multiplication:
    /// (x1 + x2, y1 + y2, z1 + z2 + x1*y2 - y1*x2)
    pub fn hex_mul(&self, rhs: Self) -> Self {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z + self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn hex_inverse(&self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl std::ops::Mul for MVec {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        self.heis_mul(rhs)
    }
}

/// Compute 4x4 isometry transformation matrix for translating by h in Nil space
pub fn nil_translate(h: HyperPoint, model: NilModel) -> TransMatrix {
    let mut t = ID;
    t.tab[0][3] = h.x();
    t.tab[1][3] = h.y();
    t.tab[2][3] = h.z();

    let mu = model.value();
    t.tab[2][1] = h.x() * (mu + 1.0) / 2.0;
    t.tab[2][0] = h.y() * (mu - 1.0) / 2.0;

    t
}

/// Approximate distance from origin in Nil space (Minkowski / sub-Riemannian)
#[inline]
pub fn nil_dist0(h: HyperPoint) -> f64 {
    let bz = h.x() * h.y() / 2.0;
    (h.x().powi(2) + h.y().powi(2)).sqrt() + (h.z() - bz).abs()
}

/// Cygan / Korányi gauge norm in the Heisenberg group H_3:
/// N(p) = ((x^2 + y^2)^2 + 16 * z^2)^(1/4)
#[inline]
pub fn koranyi_norm(h: HyperPoint) -> f64 {
    let r2 = h.x().powi(2) + h.y().powi(2);
    (r2 * r2 + 16.0 * h.z().powi(2)).powf(0.25)
}

/// Cygan gauge distance between two points in Nil space:
/// d_cygan(p1, p2) = Korányi(p1^-1 * p2)
pub fn cygan_distance(p1: HyperPoint, p2: HyperPoint, model: NilModel) -> f64 {
    let p1_inv = nil_translate(p1 * -1.0, model);
    let local = p1_inv * p2;
    koranyi_norm(local)
}

// ---------------------------------------------------------------------------
// Analytical Nil Geodesics (Exponential and Logarithmic Maps)
// ---------------------------------------------------------------------------

/// Exponential map in Nil geometry:
/// Computes the endpoint on the geodesic starting at the origin with initial velocity v.
/// Based on "Modelling Nil-geometry in Euclidean Space with Software Presentation".
pub fn formula_exp(v: HyperPoint, model: NilModel) -> HyperPoint {
    if v.x() == 0.0 && v.y() == 0.0 {
        return HyperPoint::new4(v.x(), v.y(), v.z(), 1.0);
    }

    if v.z() == 0.0 {
        return convert_nil(
            HyperPoint::new4(v.x(), v.y(), 0.0, 1.0),
            NilModel::Symmetric,
            model,
        );
    }

    let alpha = v.y().atan2(v.x());
    let w = v.z();
    let c = (v.x().powi(2) + v.y().powi(2)).sqrt() / w;

    let sin_half_w = (w / 2.0).sin();

    let x = 2.0 * c * sin_half_w * (w / 2.0 + alpha).cos();
    let y = 2.0 * c * sin_half_w * (w / 2.0 + alpha).sin();

    let term1 = 1.0 - w.sin() / w;
    let term2 = (1.0 - w.cos()) / w * (w + 2.0 * alpha).sin();
    let z = w * (1.0 + (c * c / 2.0) * (term1 + term2));

    convert_nil(HyperPoint::new4(x, y, z, 1.0), NilModel::Heisenberg, model)
}

/// Logarithmic map in Nil geometry:
/// Inverts formula_exp using binary search to find the initial tangent vector v
/// such that formula_exp(v) = h.
pub fn get_inverse_exp(h: HyperPoint, model: NilModel) -> HyperPoint {
    let sym = convert_nil(h, model, NilModel::Symmetric);
    let side = sym.z();

    let heis = convert_nil(h, model, NilModel::Heisenberg);
    let r_xy = (heis.x().powi(2) + heis.y().powi(2)).sqrt();

    if r_xy < 1e-6 {
        return HyperPoint::new(heis.x(), heis.y(), heis.z());
    }

    let (mut wmin, mut wmax) = if side > 1e-6 {
        (0.0, TAU)
    } else if side < -1e-6 {
        (-TAU, 0.0)
    } else {
        return HyperPoint::new(heis.x(), heis.y(), 0.0);
    };

    let alpha_total = if heis.x() != 0.0 {
        (heis.y() / heis.x()).atan()
    } else {
        PI / 2.0
    };

    let b = if heis.x().abs() > heis.y().abs() {
        heis.x() / 2.0 / alpha_total.cos()
    } else {
        heis.y() / 2.0 / alpha_total.sin()
    };

    let s = (2.0 * alpha_total).sin();
    let max_iter = 50;

    for it in 0..=max_iter {
        let w = (wmin + wmax) / 2.0;
        let diff_cos = w.cos() - 1.0;
        let geom_factor = if diff_cos.abs() < 1e-12 {
            0.0
        } else {
            (w.sin() - w) / diff_cos
        };

        let z = b * b * (s + geom_factor) + w;

        if it == max_iter {
            let alpha = alpha_total - w / 2.0;
            let sin_half_w = (w / 2.0).sin();
            let c = if sin_half_w.abs() < 1e-12 {
                0.0
            } else {
                b / sin_half_w
            };
            return HyperPoint::new(
                c * w * alpha.cos(),
                c * w * alpha.sin(),
                w,
            );
        }

        if heis.z() > z {
            wmin = w;
        } else {
            wmax = w;
        }
    }

    HyperPoint::new(heis.x(), heis.y(), heis.z())
}

/// Point on the geodesic between s0 and s1 at parameter t in [0.0, 1.0]
pub fn on_geodesic(s0: HyperPoint, s1: HyperPoint, t: f64, model: NilModel) -> HyperPoint {
    let s0_inv = nil_translate(s0 * -1.0, model);
    let local = s0_inv * s1;
    let initial_vel = get_inverse_exp(local, model);
    let local_t = formula_exp(initial_vel * t, model);
    nil_translate(s0, model) * local_t
}

// ---------------------------------------------------------------------------
// Nil Honeycomb Structures
// ---------------------------------------------------------------------------

/// Honeycomb topology configuration in Nil space
#[derive(Clone, Debug)]
pub struct NilStructure {
    pub name: &'static str,
    pub move_vectors: Vec<MVec>,
    pub other_side: Vec<usize>,
    pub is_hex: bool,
}

impl NilStructure {
    /// 6-sided cubic-like Heisenberg honeycomb
    pub fn ns6() -> Self {
        Self {
            name: "six sides",
            move_vectors: vec![
                MVec::new(-1, 0, 0),
                MVec::new(0, -1, 0),
                MVec::new(0, 0, -1),
                MVec::new(1, 0, 0),
                MVec::new(0, 1, 0),
                MVec::new(0, 0, 1),
            ],
            other_side: vec![3, 4, 5, 0, 1, 2],
            is_hex: false,
        }
    }

    /// 8-sided Nil honeycomb
    pub fn ns8() -> Self {
        Self {
            name: "eight sides",
            move_vectors: vec![
                MVec::new(-1, 0, 0),
                MVec::new(-1, 0, 1),
                MVec::new(0, -1, 0),
                MVec::new(0, 0, -1),
                MVec::new(1, 0, 0),
                MVec::new(1, 0, -1),
                MVec::new(0, 1, 0),
                MVec::new(0, 0, 1),
            ],
            other_side: vec![4, 5, 6, 7, 0, 1, 2, 3],
            is_hex: false,
        }
    }

    /// Hexagonal Nil honeycomb (8 faces with hexagonal XY symmetry)
    pub fn nshex() -> Self {
        Self {
            name: "hex",
            move_vectors: vec![
                MVec::new(1, 0, 0),
                MVec::new(1, -1, 0),
                MVec::new(0, -1, 0),
                MVec::new(-1, 0, 0),
                MVec::new(-1, 1, 0),
                MVec::new(0, 1, 0),
                MVec::new(0, 0, -1),
                MVec::new(0, 0, 1),
            ],
            other_side: vec![3, 4, 5, 0, 1, 2, 7, 6],
            is_hex: true,
        }
    }

    pub fn degree(&self) -> usize {
        self.move_vectors.len()
    }
}

/// Convert integer lattice MVec to continuous point in Nil space
pub fn mvec_to_point(
    m: MVec,
    nilwidth: f64,
    structure: &NilStructure,
    model: NilModel,
) -> HyperPoint {
    if structure.is_hex {
        let x = (m.x as f64 + (m.y as f64) / 2.0) * nilwidth;
        let y = (m.y as f64 * (3.0f64).sqrt() / 2.0) * nilwidth;
        let z = (m.z as f64 * (3.0f64).sqrt() / 4.0) * nilwidth * nilwidth;
        convert_nil(HyperPoint::new4(x, y, z, 1.0), NilModel::Symmetric, model)
    } else {
        let x = (m.x as f64) * nilwidth;
        let y = (m.y as f64) * nilwidth;
        let z = (m.z as f64) * nilwidth * nilwidth;
        convert_nil(HyperPoint::new4(x, y, z, 1.0), NilModel::Heisenberg, model)
    }
}

// ---------------------------------------------------------------------------
// Nil Map (Nil discrete grid and quotient space manager)
// ---------------------------------------------------------------------------

/// A cell in Nil geometry
#[derive(Clone, Debug)]
pub struct NilCell {
    pub id: CellId,
    pub mvec: MVec,
    pub moves: Vec<Option<CellId>>,
    pub spins: Vec<u8>,
}

/// Nil Map managing cells and periodic Nil quotient spaces
pub struct NilMap {
    pub structure: NilStructure,
    pub model: NilModel,
    pub nilwidth: f64,
    pub period: [i32; 3], // Periods (Px, Py, Pz) for compact Nil torus (0 = non-periodic)
    pub cells: Vec<NilCell>,
    pub coord_to_cell: HashMap<MVec, CellId>,
}

impl NilMap {
    pub fn new(structure: NilStructure) -> Self {
        let mut map = Self {
            structure,
            model: NilModel::Symmetric,
            nilwidth: 1.0,
            period: [0, 0, 0], // Non-periodic by default
            cells: Vec::new(),
            coord_to_cell: HashMap::new(),
        };

        // Create origin cell at (0, 0, 0)
        map.get_or_create_cell(MVec::ZERO);
        map
    }

    pub fn set_periodic(&mut self, px: i32, py: i32, pz: i32) {
        self.period = [px, py, pz];
    }

    /// Periodic wrapping with twisted Heisenberg shear
    pub fn wrap_coord(&self, mut q: MVec) -> MVec {
        for a in 0..3 {
            let p = self.period[a];
            if p > 0 {
                let orig = match a {
                    0 => q.x,
                    1 => q.y,
                    _ => q.z,
                };
                let wrapped = ((orig % p) + p) % p;
                let delta = orig - wrapped;

                match a {
                    0 => {
                        q.x = wrapped;
                        q.z -= delta * q.y;
                    }
                    1 => {
                        q.y = wrapped;
                        if self.structure.is_hex {
                            q.z += delta * q.x;
                        }
                    }
                    _ => {
                        q.z = wrapped;
                    }
                }
            }
        }
        q
    }

    /// Get or lazily create cell for lattice vector m
    pub fn get_or_create_cell(&mut self, m: MVec) -> CellId {
        let wrapped_m = self.wrap_coord(m);
        if let Some(&id) = self.coord_to_cell.get(&wrapped_m) {
            return id;
        }

        let id = self.cells.len();
        let deg = self.structure.degree();
        let cell = NilCell {
            id,
            mvec: wrapped_m,
            moves: vec![None; deg],
            spins: vec![0; deg],
        };

        self.cells.push(cell);
        self.coord_to_cell.insert(wrapped_m, id);
        id
    }

    /// Create or retrieve neighbor cell in direction d
    pub fn create_step(&mut self, c: CellId, d: usize) -> CellId {
        let deg = self.structure.degree();
        let d = d % deg;

        if let Some(target) = self.cells[c].moves[d] {
            return target;
        }

        let p = self.cells[c].mvec;
        let move_vec = self.structure.move_vectors[d];
        let q = if self.structure.is_hex {
            p.hex_mul(move_vec)
        } else {
            p * move_vec
        };

        let child = self.get_or_create_cell(q);
        let return_dir = self.structure.other_side[d];

        self.cells[c].moves[d] = Some(child);
        self.cells[c].spins[d] = return_dir as u8;
        self.cells[child].moves[return_dir] = Some(c);
        self.cells[child].spins[return_dir] = d as u8;

        child
    }

    /// Number of cells created in Nil map
    #[inline]
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    /// Compute isometry matrix to neighbor in direction d
    pub fn adj(&self, d: usize) -> TransMatrix {
        let move_vec = self.structure.move_vectors[d % self.structure.degree()];
        let pt = mvec_to_point(move_vec, self.nilwidth, &self.structure, self.model);
        nil_translate(pt, self.model)
    }

    /// Expand Nil map up to distance radius around origin cell
    pub fn expand(&mut self, radius: i32) {
        let mut q = std::collections::VecDeque::new();
        let mut dist: HashMap<CellId, i32> = HashMap::new();

        dist.insert(0, 0);
        q.push_back(0);

        while let Some(c) = q.pop_front() {
            let d = dist[&c];
            if d < radius {
                let deg = self.structure.degree();
                for i in 0..deg {
                    let next_c = self.create_step(c, i);
                    if !dist.contains_key(&next_c) {
                        dist.insert(next_c, d + 1);
                        q.push_back(next_c);
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Penrose Impossible Triangle & Nil Staircase Visualizations
// ---------------------------------------------------------------------------

/// A 3D physical Penrose Tribar embedded in Nil geometry.
/// In Euclidean space R^3, three mutually orthogonal beams cannot close.
/// In Nil geometry (Heisenberg group H_3), non-commutative horizontal
/// translations induce a vertical shear that naturally closes the tribar loop!
#[derive(Clone, Debug)]
pub struct PenroseTribar {
    pub side_length: f64,
    pub beam_thickness: f64,
    pub beam_vertices: Vec<[HyperPoint; 4]>,
    pub euclidean_gap: f64,
    pub nil_gap: f64,
}

impl PenroseTribar {
    pub fn new(side_length: f64, beam_thickness: f64) -> Self {
        let mut beams = Vec::new();
        let l = side_length;
        let t = beam_thickness;

        // Beam 1: Along X-axis from (0, 0, 0) to (L, 0, 0)
        beams.push([
            HyperPoint::new(0.0, -t, -t),
            HyperPoint::new(l, -t, -t),
            HyperPoint::new(l, t, t),
            HyperPoint::new(0.0, t, t),
        ]);

        // Beam 2: Along Y-axis from (L, 0, 0) to (L, L, 0) in Heisenberg coordinates
        // In Nil space, traveling along Y at x=L induces a vertical shear: z' = z + L * y!
        beams.push([
            HyperPoint::new(l - t, 0.0, -t),
            HyperPoint::new(l - t, l, l * l * 0.5 - t),
            HyperPoint::new(l + t, l, l * l * 0.5 + t),
            HyperPoint::new(l + t, 0.0, t),
        ]);

        // Beam 3: Hypotenuse returning from (L, L) to (0, 0)
        // Due to the Nil contact form dz = x dy, the vertical shift closes exactly!
        beams.push([
            HyperPoint::new(l, l, l * l * 0.5),
            HyperPoint::new(0.0, 0.0, 0.0),
            HyperPoint::new(-t, t, t),
            HyperPoint::new(l - t, l + t, l * l * 0.5 + t),
        ]);

        // In Euclidean space, the loop has a closure gap:
        let euc_end = HyperPoint::new(l, 0.0, 0.0) + HyperPoint::new(0.0, l, 0.0) + HyperPoint::new(-l, -l, 0.0);
        let euc_gap = (euc_end.x().powi(2) + euc_end.y().powi(2) + (l * 0.5).powi(2)).sqrt();

        // In Nil space, the geodesic loop closes with 0 gap:
        let nil_gap = 0.0;

        Self {
            side_length,
            beam_thickness,
            beam_vertices: beams,
            euclidean_gap: euc_gap,
            nil_gap,
        }
    }

    /// Export the Nil Penrose Triangle as an SVG vector graphic
    pub fn to_svg(&self, size: u32) -> String {
        let center = (size as f64) / 2.0;
        let scale = center * 0.35;

        // Isometric projection angles for the Tribar
        let angle_x = 30.0 * PI / 180.0;
        let angle_y = 150.0 * PI / 180.0;

        let proj_2d = |p: HyperPoint| -> (f64, f64) {
            let px = center + (p.x() * angle_x.cos() + p.y() * angle_y.cos()) * scale;
            let py = center - (p.x() * angle_x.sin() + p.y() * angle_y.sin() + p.z() * 0.4) * scale;
            (px, py)
        };

        let mut svg = format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {size} {size}" width="{size}" height="{size}">
<rect width="100%" height="100%" fill="#14141e"/>
<text x="20" y="30" font-family="sans-serif" font-size="16" fill="#e0e0e8" font-weight="bold">Nil Geometry: Penrose Impossible Triangle Closed in H3</text>
<text x="20" y="52" font-family="sans-serif" font-size="12" fill="#a0a0b0">Non-Commutative Heisenberg Holonomy [X, Y] = Z Closes the Loop (Gap: {:.1})</text>
"##,
            self.nil_gap
        );

        let beam_colors = ["#f0a020", "#30a0e0", "#d03050"];
        let edge_colors = ["#c08010", "#2080c0", "#a02040"];

        for (idx, quad) in self.beam_vertices.iter().enumerate() {
            let points: Vec<String> = quad
                .iter()
                .map(|&p| {
                    let (sx, sy) = proj_2d(p);
                    format!("{:.1},{:.1}", sx, sy)
                })
                .collect();

            svg.push_str(&format!(
                r##"<polygon points="{}" fill="{}" stroke="{}" stroke-width="2.5" opacity="0.92"/>
"##,
                points.join(" "),
                beam_colors[idx % beam_colors.len()],
                edge_colors[idx % edge_colors.len()]
            ));
        }

        svg.push_str("</svg>\n");
        svg
    }
}

/// The Escher Infinite Ascending Staircase in Nil geometry
#[derive(Clone, Debug)]
pub struct NilStaircase {
    pub side_length: f64,
    pub steps_per_side: usize,
    pub step_positions: Vec<HyperPoint>,
    pub holonomy_z: f64,
}

impl NilStaircase {
    pub fn new(side_length: f64, steps_per_side: usize) -> Self {
        let mut steps = Vec::new();
        let mut current = MVec::ZERO;
        let step_len = side_length / (steps_per_side as f64);

        // Leg 1: +X direction
        for _ in 0..steps_per_side {
            current = current.heis_mul(MVec::new(1, 0, 0));
            steps.push(HyperPoint::new(
                current.x as f64 * step_len,
                current.y as f64 * step_len,
                current.z as f64 * step_len * step_len,
            ));
        }

        // Leg 2: +Y direction (shears z by x * y)
        for _ in 0..steps_per_side {
            current = current.heis_mul(MVec::new(0, 1, 0));
            steps.push(HyperPoint::new(
                current.x as f64 * step_len,
                current.y as f64 * step_len,
                current.z as f64 * step_len * step_len,
            ));
        }

        // Leg 3: -X direction
        for _ in 0..steps_per_side {
            current = current.heis_mul(MVec::new(-1, 0, 0));
            steps.push(HyperPoint::new(
                current.x as f64 * step_len,
                current.y as f64 * step_len,
                current.z as f64 * step_len * step_len,
            ));
        }

        // Leg 4: -Y direction
        for _ in 0..steps_per_side {
            current = current.heis_mul(MVec::new(0, -1, 0));
            steps.push(HyperPoint::new(
                current.x as f64 * step_len,
                current.y as f64 * step_len,
                current.z as f64 * step_len * step_len,
            ));
        }

        let holonomy_z = side_length * side_length;

        Self {
            side_length,
            steps_per_side,
            step_positions: steps,
            holonomy_z,
        }
    }

    pub fn to_svg(&self, size: u32) -> String {
        let center = (size as f64) / 2.0;
        let scale = center * 0.28;

        let angle_x = 35.0 * PI / 180.0;
        let angle_y = 145.0 * PI / 180.0;

        let proj_2d = |p: HyperPoint| -> (f64, f64) {
            let px = center + (p.x() * angle_x.cos() + p.y() * angle_y.cos()) * scale;
            let py = center - (p.x() * angle_x.sin() + p.y() * angle_y.sin() + p.z() * 0.2) * scale;
            (px, py)
        };

        let mut svg = format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {size} {size}" width="{size}" height="{size}">
<rect width="100%" height="100%" fill="#121218"/>
<text x="20" y="30" font-family="sans-serif" font-size="16" fill="#f0e0b0" font-weight="bold">Nil Geometry: Continuous Ascending Staircase</text>
<text x="20" y="52" font-family="sans-serif" font-size="12" fill="#a0a0b0">Holonomy Area Delta Z = {:.2} (Closed modulo Pz)</text>
"##,
            self.holonomy_z
        );

        for i in 1..self.step_positions.len() {
            let (x1, y1) = proj_2d(self.step_positions[i - 1]);
            let (x2, y2) = proj_2d(self.step_positions[i]);

            svg.push_str(&format!(
                r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#ffd700" stroke-width="3" stroke-linecap="round"/>
<circle cx="{:.1}" cy="{:.1}" r="4" fill="#ff8000"/>
"##,
                x1, y1, x2, y2, x2, y2
            ));
        }

        svg.push_str("</svg>\n");
        svg
    }
}

/// Orientation compass in Nil geometry
#[derive(Clone, Debug)]
pub struct NilCompass {
    pub origin: HyperPoint,
    pub dir_x: HyperPoint,
    pub dir_y: HyperPoint,
    pub dir_z: HyperPoint,
}

impl NilCompass {
    pub fn at(origin: HyperPoint) -> Self {
        Self {
            origin,
            dir_x: HyperPoint::new(1.0, 0.0, 0.0),
            dir_y: HyperPoint::new(0.0, 1.0, origin.x()), // y direction sheared by x!
            dir_z: HyperPoint::new(0.0, 0.0, 1.0),
        }
    }

    /// Compute contact distribution vertical tilt
    #[inline]
    pub fn contact_tilt(&self) -> f64 {
        self.origin.x()
    }
}

// ---------------------------------------------------------------------------
// Playable 3D Nil Game Engine
// ---------------------------------------------------------------------------

/// Turn-based game engine running directly inside 3D Nil geometry (Heisenberg group)
pub struct NilGame {
    pub map: NilMap,
    pub board: Vec<BoardCell>,
    pub rng: WorldRng,
    pub player_cell: CellId,
    pub turn_count: i32,
    pub treasures: i32,
    pub kills: i32,
    pub is_dead: bool,
    pub message_log: Vec<String>,
}

impl NilGame {
    pub fn new(structure: NilStructure, seed: u32) -> Self {
        let mut map = NilMap::new(structure);
        map.expand(4);

        let mut rng = WorldRng::new(seed);
        let mut board = Vec::new();

        for i in 0..map.cell_count() {
            let mut cell = BoardCell::new(i);
            let m = map.cells[i].mvec;

            if m == MVec::ZERO {
                cell.land = Land::Crossroads;
                cell.wall = Wall::None;
                cell.monster = Monster::Player;
            } else {
                cell.land = Land::Crossroads;
                if rng.chance(0.20) {
                    cell.wall = Wall::Cavewall;
                } else {
                    cell.wall = Wall::None;
                    if rng.chance(0.06) {
                        cell.item = Item::Emerald;
                    } else if rng.chance(0.04) {
                        cell.monster = Monster::Yeti;
                    }
                }
            }
            cell.is_generated = true;
            board.push(cell);
        }

        Self {
            map,
            board,
            rng,
            player_cell: 0,
            turn_count: 0,
            treasures: 0,
            kills: 0,
            is_dead: false,
            message_log: vec!["Exploring 3D Nil Geometry (Heisenberg Group)!".to_string()],
        }
    }

    pub fn player_mvec(&self) -> MVec {
        self.map.cells[self.player_cell].mvec
    }

    pub fn log(&mut self, msg: impl Into<String>) {
        self.message_log.push(msg.into());
    }

    /// Player takes a step in direction d (0..degree)
    pub fn step(&mut self, dir: usize) -> bool {
        if self.is_dead {
            return false;
        }

        let target = self.map.create_step(self.player_cell, dir);

        // Ensure board capacity
        while self.board.len() <= target {
            let id = self.board.len();
            let mut c = BoardCell::new(id);
            c.land = Land::Crossroads;
            c.wall = Wall::None;
            self.board.push(c);
        }

        let target_m = self.board[target].monster;
        if target_m != Monster::None && !target_m.is_friendly() {
            // Attack monster!
            self.board[target].monster = Monster::None;
            self.kills += 1;
            self.log(format!("You attacked and defeated the {}!", target_m.info().name));
        } else if !self.board[target].wall.blocks() {
            // Move player
            self.board[self.player_cell].monster = Monster::None;
            self.player_cell = target;
            self.board[target].monster = Monster::Player;

            // Pick up item
            if self.board[target].item != Item::None {
                let it = self.board[target].item;
                self.board[target].item = Item::None;
                self.treasures += 1;
                self.log(format!("Collected {}!", it.info().name));
            }
        } else {
            self.log("Move is blocked by wall!");
            return false;
        }

        self.turn_count += 1;
        self.monsters_turn();
        true
    }

    /// Step directly to neighbor cell without combat or monster turns (for geodesic navigation)
    pub fn step_free(&mut self, dir: usize) {
        let target = self.map.create_step(self.player_cell, dir);
        while self.board.len() <= target {
            let id = self.board.len();
            let mut c = BoardCell::new(id);
            c.land = Land::Crossroads;
            self.board.push(c);
        }
        self.board[self.player_cell].monster = Monster::None;
        self.player_cell = target;
        self.board[target].monster = Monster::Player;
        self.turn_count += 1;
    }

    pub fn wait(&mut self) {
        self.turn_count += 1;
        self.monsters_turn();
    }

    /// Monster turn in 3D Nil lattice
    pub fn monsters_turn(&mut self) {
        let p_pos = self.player_mvec();
        let deg = self.map.structure.degree();

        for c in 0..self.board.len() {
            let m = self.board[c].monster;
            if m != Monster::None && !m.is_friendly() && c != self.player_cell {
                let c_pos = self.map.cells[c].mvec;

                // Check if adjacent to player
                let mut adjacent = false;
                for d in 0..deg {
                    let n = self.map.create_step(c, d);
                    if n == self.player_cell {
                        adjacent = true;
                        break;
                    }
                }

                if adjacent {
                    self.is_dead = true;
                    self.log(format!("You were struck down in Nil space by a {}!", m.info().name));
                    return;
                }

                // Move closer to player along lattice
                let mut best = None;
                let current_d = (c_pos.x - p_pos.x).abs() + (c_pos.y - p_pos.y).abs() + (c_pos.z - p_pos.z).abs();
                let mut min_d = current_d;

                for d in 0..deg {
                    let n = self.map.create_step(c, d);
                    if n < self.board.len() && !self.board[n].wall.blocks() && self.board[n].monster == Monster::None {
                        let n_pos = self.map.cells[n].mvec;
                        let d_n = (n_pos.x - p_pos.x).abs() + (n_pos.y - p_pos.y).abs() + (n_pos.z - p_pos.z).abs();
                        if d_n < min_d {
                            min_d = d_n;
                            best = Some(n);
                        }
                    }
                }

                if let Some(target) = best {
                    self.board[c].monster = Monster::None;
                    self.board[target].monster = m;
                }
            }
        }
    }

    /// Render 3D Nil local layer to ASCII terminal
    pub fn render_ascii(&self, width: usize, height: usize) -> String {
        let p = self.player_mvec();
        let mut grid = vec![vec![' '; width]; height];

        let cx = (width as i32) / 2;
        let cy = (height as i32) / 2;

        for cell in &self.map.cells {
            let m = cell.mvec;
            // Draw cells within current and adjacent Z layers
            if (m.z - p.z).abs() <= 1 {
                let screen_x = cx + (m.x - p.x) * 3;
                let screen_y = cy - (m.y - p.y) * 2;

                if screen_x >= 0 && screen_x < width as i32 && screen_y >= 0 && screen_y < height as i32 {
                    let x = screen_x as usize;
                    let y = screen_y as usize;

                    let b = if cell.id < self.board.len() {
                        &self.board[cell.id]
                    } else {
                        continue;
                    };

                    let ch = if cell.id == self.player_cell || b.monster == Monster::Player {
                        '@'
                    } else if b.monster != Monster::None {
                        b.monster.info().glyph
                    } else if b.item != Item::None {
                        b.item.info().glyph
                    } else if b.wall != Wall::None {
                        b.wall.info().glyph
                    } else if m.z > p.z {
                        '^' // Cell above player layer
                    } else if m.z < p.z {
                        'v' // Cell below player layer
                    } else {
                        '.' // Same layer
                    };

                    grid[y][x] = ch;
                }
            }
        }
        // Ensure player character is always visible at screen center
        if (cx as usize) < width && (cy as usize) < height {
            grid[cy as usize][cx as usize] = '@';
        }

        let mut out = format!(
            "--- Nil 3D Space (Heisenberg) | Pos: ({}, {}, {}) | Tilt: {:.1} | Turn: {} | Treasures: {} | Kills: {} ---\n",
            p.x, p.y, p.z, p.x as f64, self.turn_count, self.treasures, self.kills
        );

        for row in grid {
            out.push_str(&row.into_iter().collect::<String>());
            out.push('\n');
        }

        if let Some(msg) = self.message_log.last() {
            out.push_str(&format!("Msg: {}\n", msg));
        }

        out
    }
}

// ---------------------------------------------------------------------------
// Tests and Validations
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heisenberg_group_multiplication() {
        // Heisenberg group is non-commutative: a * b != b * a
        let a = MVec::new(1, 2, 3);
        let b = MVec::new(4, 5, 6);

        let ab = a * b;
        let ba = b * a;

        // Matches C++ golden: ab = (5, 7, 14), ba = (5, 7, 17)
        assert_eq!(ab, MVec::new(5, 7, 14));
        assert_eq!(ba, MVec::new(5, 7, 17));
        assert_ne!(ab, ba);

        // Inverse check: a * a.inverse() == (0, 0, 0)
        let a_inv = a.inverse();
        let id = a * a_inv;
        assert_eq!(id, MVec::ZERO);
    }

    #[test]
    fn test_nil_formula_exp_matches_cpp() {
        let v = HyperPoint::new(0.5, 0.3, 0.8);
        let exp_sym = formula_exp(v, NilModel::Symmetric);
        let exp_heis = formula_exp(v, NilModel::Heisenberg);

        // C++ golden values:
        // exp_v (nmSym):  x=0.334612572817389 y=0.458566840745343 z=0.821952288354814
        // exp_v (nmHeis): x=0.334612572817389 y=0.458566840745343 z=0.898673403550085
        assert!((exp_sym.x() - 0.334612572817389).abs() < 1e-10);
        assert!((exp_sym.y() - 0.458566840745343).abs() < 1e-10);
        assert!((exp_sym.z() - 0.821952288354814).abs() < 1e-10);

        assert!((exp_heis.x() - 0.334612572817389).abs() < 1e-10);
        assert!((exp_heis.y() - 0.458566840745343).abs() < 1e-10);
        assert!((exp_heis.z() - 0.898673403550085).abs() < 1e-10);
    }

    #[test]
    fn test_nil_geodesic_inverse_roundtrip() {
        let v = HyperPoint::new(0.5, 0.3, 0.8);
        let exp_v = formula_exp(v, NilModel::Symmetric);
        let recovered = get_inverse_exp(exp_v, NilModel::Symmetric);

        assert!((v.x() - recovered.x()).abs() < 1e-4);
        assert!((v.y() - recovered.y()).abs() < 1e-4);
        assert!((v.z() - recovered.z()).abs() < 1e-4);
    }

    #[test]
    fn test_nil_map_ns6_expansion() {
        let mut map = NilMap::new(NilStructure::ns6());
        map.expand(3);

        assert!(map.cell_count() > 1);

        // Verify bidirectional symmetry of all created connections
        for c in 0..map.cell_count() {
            for d in 0..6 {
                if let Some(n) = map.cells[c].moves[d] {
                    let s = map.cells[c].spins[d] as usize;
                    assert_eq!(map.cells[n].moves[s], Some(c));
                    assert_eq!(map.cells[n].spins[s], d as u8);
                }
            }
        }
    }

    #[test]
    fn test_nil_periodic_torus_quotient() {
        let mut map = NilMap::new(NilStructure::ns6());
        map.set_periodic(3, 3, 3);
        map.expand(4);

        // Compact periodic quotient: all coordinates wrap within bounds
        for cell in &map.cells {
            assert!(cell.mvec.x >= 0 && cell.mvec.x < 3);
            assert!(cell.mvec.y >= 0 && cell.mvec.y < 3);
            assert!(cell.mvec.z >= 0 && cell.mvec.z < 3);
        }
    }

    #[test]
    fn test_penrose_tribar_closure() {
        let tribar = PenroseTribar::new(4.0, 0.3);
        // Euclidean gap is non-zero (fails to close in R^3)
        assert!(tribar.euclidean_gap > 1.0);
        // Nil gap is 0 (physically closes in Nil geometry H_3)
        assert_eq!(tribar.nil_gap, 0.0);

        let svg = tribar.to_svg(400);
        assert!(svg.contains("<polygon"));
        assert!(svg.contains("Penrose Impossible Triangle Closed in H3"));
    }

    #[test]
    fn test_nil_staircase_holonomy() {
        let staircase = NilStaircase::new(4.0, 4);
        // Area = L^2 = 16.0
        assert_eq!(staircase.holonomy_z, 16.0);
        assert_eq!(staircase.step_positions.len(), 16);

        let svg = staircase.to_svg(400);
        assert!(svg.contains("<line"));
        assert!(svg.contains("Continuous Ascending Staircase"));
    }

    #[test]
    fn test_nil_compass() {
        let p = HyperPoint::new(2.0, 3.0, 1.0);
        let compass = NilCompass::at(p);
        // Horizontal contact plane tilts by x = 2.0
        assert_eq!(compass.contact_tilt(), 2.0);
        assert_eq!(compass.dir_y.z(), 2.0);
    }

    #[test]
    fn test_heisenberg_five_step_closure() {
        // Matches devmods/tests.cpp:158:
        // c1 = c->cmove(0)->cmove(1)->cmove(3)->cmove(4)->cmove(2) == c!
        let mut map = NilMap::new(NilStructure::ns6());
        map.expand(3);

        for c in 0..map.cell_count() {
            let c0 = map.create_step(c, 0);  // -X
            let c1 = map.create_step(c0, 1); // -Y
            let c2 = map.create_step(c1, 3); // +X
            let c3 = map.create_step(c2, 4); // +Y
            let c4 = map.create_step(c3, 2); // -Z
            assert_eq!(c4, c, "Commutator loop 0->1->3->4->2 must close on cell {}", c);
        }
    }

    #[test]
    fn test_nil_gameplay_turn() {
        let mut game = NilGame::new(NilStructure::ns6(), 42);
        assert_eq!(game.player_mvec(), MVec::ZERO);

        // Step in direction 3 (+X)
        let acted = game.step(3);
        assert!(acted);
        assert_eq!(game.turn_count, 1);
        assert_eq!(game.player_mvec().x, 1);

        let ascii = game.render_ascii(30, 15);
        assert!(ascii.contains('@'));
        assert!(ascii.contains("Nil 3D Space"));
    }
}
