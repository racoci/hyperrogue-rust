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
}
