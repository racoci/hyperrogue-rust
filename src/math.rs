//! Continuous Hyperbolic Geometry and Mathematics
//!
//! Models the Minkowski hyperboloid model of hyperbolic geometry:
//! Points in R^{2,1} satisfying -x^2 - y^2 + z^2 = 1 with z >= 1.
//! Also provides isometric transformations (Lorentz boosts and rotations)
//! and projections to Klein and Poincaré disk models.

use std::f64::consts::PI;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub const TAU: f64 = 2.0 * PI;
pub const DEGREE: f64 = PI / 180.0;

/// A point in continuous space using homogeneous coordinates.
/// In 2D hyperbolic space, this is the Minkowski hyperboloid model:
/// (x, y, z) such that -x^2 - y^2 + z^2 = 1 and z >= 1.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HyperPoint {
    pub coords: [f64; 4],
}

impl HyperPoint {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            coords: [x, y, z, 0.0],
        }
    }

    pub const fn new4(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self {
            coords: [x, y, z, w],
        }
    }

    pub const fn origin() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.coords[0]
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.coords[1]
    }

    #[inline]
    pub fn z(&self) -> f64 {
        self.coords[2]
    }

    #[inline]
    pub fn w(&self) -> f64 {
        self.coords[3]
    }

    #[inline]
    pub fn x_mut(&mut self) -> &mut f64 {
        &mut self.coords[0]
    }

    #[inline]
    pub fn y_mut(&mut self) -> &mut f64 {
        &mut self.coords[1]
    }

    #[inline]
    pub fn z_mut(&mut self) -> &mut f64 {
        &mut self.coords[2]
    }

    #[inline]
    pub fn w_mut(&mut self) -> &mut f64 {
        &mut self.coords[3]
    }
}

pub const C0: HyperPoint = HyperPoint::origin();
pub const HYPC: HyperPoint = HyperPoint::zero();
pub const CX1: HyperPoint = HyperPoint::new(1.0, 0.0, std::f64::consts::SQRT_2);

impl Add for HyperPoint {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::new4(
            self.coords[0] + rhs.coords[0],
            self.coords[1] + rhs.coords[1],
            self.coords[2] + rhs.coords[2],
            self.coords[3] + rhs.coords[3],
        )
    }
}

impl AddAssign for HyperPoint {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.coords[0] += rhs.coords[0];
        self.coords[1] += rhs.coords[1];
        self.coords[2] += rhs.coords[2];
        self.coords[3] += rhs.coords[3];
    }
}

impl Sub for HyperPoint {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::new4(
            self.coords[0] - rhs.coords[0],
            self.coords[1] - rhs.coords[1],
            self.coords[2] - rhs.coords[2],
            self.coords[3] - rhs.coords[3],
        )
    }
}

impl SubAssign for HyperPoint {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.coords[0] -= rhs.coords[0];
        self.coords[1] -= rhs.coords[1];
        self.coords[2] -= rhs.coords[2];
        self.coords[3] -= rhs.coords[3];
    }
}

impl Mul<f64> for HyperPoint {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f64) -> Self {
        Self::new4(
            self.coords[0] * rhs,
            self.coords[1] * rhs,
            self.coords[2] * rhs,
            self.coords[3] * rhs,
        )
    }
}

impl Mul<HyperPoint> for f64 {
    type Output = HyperPoint;
    #[inline]
    fn mul(self, rhs: HyperPoint) -> HyperPoint {
        rhs * self
    }
}

impl MulAssign<f64> for HyperPoint {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        self.coords[0] *= rhs;
        self.coords[1] *= rhs;
        self.coords[2] *= rhs;
        self.coords[3] *= rhs;
    }
}

impl Div<f64> for HyperPoint {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f64) -> Self {
        Self::new4(
            self.coords[0] / rhs,
            self.coords[1] / rhs,
            self.coords[2] / rhs,
            self.coords[3] / rhs,
        )
    }
}

impl DivAssign<f64> for HyperPoint {
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        self.coords[0] /= rhs;
        self.coords[1] /= rhs;
        self.coords[2] /= rhs;
        self.coords[3] /= rhs;
    }
}

impl Neg for HyperPoint {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self::new4(
            -self.coords[0],
            -self.coords[1],
            -self.coords[2],
            -self.coords[3],
        )
    }
}

impl std::ops::Index<usize> for HyperPoint {
    type Output = f64;
    #[inline]
    fn index(&self, index: usize) -> &f64 {
        &self.coords[index]
    }
}

impl std::ops::IndexMut<usize> for HyperPoint {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.coords[index]
    }
}

/// A 4x4 matrix for hyperbolic isometries and transformations.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransMatrix {
    pub tab: [[f64; 4]; 4],
}

impl TransMatrix {
    pub const fn identity() -> Self {
        Self {
            tab: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub const fn zero() -> Self {
        Self {
            tab: [[0.0; 4]; 4],
        }
    }

    pub const fn diag(a: f64, b: f64, c: f64, d: f64) -> Self {
        Self {
            tab: [
                [a, 0.0, 0.0, 0.0],
                [0.0, b, 0.0, 0.0],
                [0.0, 0.0, c, 0.0],
                [0.0, 0.0, 0.0, d],
            ],
        }
    }

    pub const fn matrix3(
        a: f64, b: f64, c: f64,
        d: f64, e: f64, f: f64,
        g: f64, h: f64, i: f64,
    ) -> Self {
        Self {
            tab: [
                [a, b, c, 0.0],
                [d, e, f, 0.0],
                [g, h, i, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}

pub const ID: TransMatrix = TransMatrix::identity();
pub const ZERO_MAT: TransMatrix = TransMatrix::zero();
pub const MIRROR: TransMatrix = TransMatrix::diag(1.0, -1.0, 1.0, 1.0);
pub const MIRROR_X: TransMatrix = TransMatrix::diag(-1.0, 1.0, 1.0, 1.0);
pub const MIRROR_Y: TransMatrix = TransMatrix::diag(1.0, -1.0, 1.0, 1.0);
pub const MIRROR_Z: TransMatrix = TransMatrix::diag(1.0, 1.0, -1.0, 1.0);
pub const PISPIN: TransMatrix = TransMatrix::diag(-1.0, -1.0, 1.0, 1.0);

impl Mul for TransMatrix {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        let mut r = Self::zero();
        for i in 0..4 {
            for j in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self.tab[i][k] * rhs.tab[k][j];
                }
                r.tab[i][j] = sum;
            }
        }
        r
    }
}

impl Mul<HyperPoint> for TransMatrix {
    type Output = HyperPoint;
    #[inline]
    fn mul(self, rhs: HyperPoint) -> HyperPoint {
        let mut res = HyperPoint::zero();
        for i in 0..4 {
            let mut sum = 0.0;
            for j in 0..4 {
                sum += self.tab[i][j] * rhs.coords[j];
            }
            res.coords[i] = sum;
        }
        res
    }
}

impl std::ops::Index<usize> for TransMatrix {
    type Output = [f64; 4];
    #[inline]
    fn index(&self, index: usize) -> &[f64; 4] {
        &self.tab[index]
    }
}

impl std::ops::IndexMut<usize> for TransMatrix {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut [f64; 4] {
        &mut self.tab[index]
    }
}

// ---------------------------------------------------------------------------
// Hyperbolic metric & trigonometry
// ---------------------------------------------------------------------------

#[inline]
pub fn squar(x: f64) -> f64 {
    x * x
}

#[inline]
pub fn acosh_clamp(x: f64) -> f64 {
    if x < 1.0 {
        0.0
    } else {
        x.acosh()
    }
}

#[inline]
pub fn acos_clamp(x: f64) -> f64 {
    if x > 1.0 {
        0.0
    } else if x < -1.0 {
        PI
    } else if x.is_nan() {
        0.0
    } else {
        x.acos()
    }
}

#[inline]
pub fn asin_clamp(x: f64) -> f64 {
    if x > 1.0 {
        PI / 2.0
    } else if x < -1.0 {
        -PI / 2.0
    } else if x.is_nan() {
        0.0
    } else {
        x.asin()
    }
}

/// Compute the third side of a hyperbolic triangle given its angles alpha, beta, gamma.
/// Dual to spherical cosine rule for angles: cosh(c) = (cos(alpha) + cos(beta)*cos(gamma)) / (sin(beta)*sin(gamma)).
#[inline]
pub fn edge_of_triangle_with_angles(alpha: f64, beta: f64, gamma: f64) -> f64 {
    let num = alpha.cos() + beta.cos() * gamma.cos();
    let den = beta.sin() * gamma.sin();
    acosh_clamp(num / den)
}

/// Minkowski inner product: -x1*x2 - y1*y2 + z1*z2
#[inline]
pub fn geo_inner(h1: &HyperPoint, h2: &HyperPoint) -> f64 {
    -h1.coords[0] * h2.coords[0] - h1.coords[1] * h2.coords[1] + h1.coords[2] * h2.coords[2]
}

/// Minkowski chord squared interval:
/// (x1 - x2)^2 + (y1 - y2)^2 - (z1 - z2)^2
#[inline]
pub fn intval(h1: HyperPoint, h2: HyperPoint) -> f64 {
    squar(h1.coords[0] - h2.coords[0]) + squar(h1.coords[1] - h2.coords[1])
        - squar(h1.coords[2] - h2.coords[2])
}

/// Distance from the origin C0 (0, 0, 1)
#[inline]
pub fn hdist0(h: HyperPoint) -> f64 {
    if h.coords[2] < 1.0 {
        0.0
    } else {
        h.coords[2].acosh()
    }
}

/// Distance between two points in hyperbolic space
#[inline]
pub fn hdist(h1: HyperPoint, h2: HyperPoint) -> f64 {
    let iv = intval(h1, h2);
    if iv < 0.0 {
        0.0
    } else {
        2.0 * (iv.sqrt() / 2.0).asinh()
    }
}

/// Normalizing factor to project onto the hyperboloid
#[inline]
pub fn zlevel(h: HyperPoint) -> f64 {
    let val = -intval(h, HYPC);
    if val <= 0.0 {
        1e-10
    } else {
        val.sqrt()
    }
}

/// Normalize coordinates so they lie exactly on the hyperboloid sheet
#[inline]
pub fn normalize(mut h: HyperPoint) -> HyperPoint {
    let z = zlevel(h);
    h.coords[0] /= z;
    h.coords[1] /= z;
    h.coords[2] /= z;
    h
}

/// Midpoint between two points on the hyperbolic plane
#[inline]
pub fn mid(h1: HyperPoint, h2: HyperPoint) -> HyperPoint {
    normalize(h1 + h2)
}

// ---------------------------------------------------------------------------
// Hyperbolic Isometries: Rotations and Boosts
// ---------------------------------------------------------------------------

/// Rotation in coordinates a, b by angle alpha
pub fn cspin(a: usize, b: usize, alpha: f64) -> TransMatrix {
    let mut t = ID;
    let cos_a = alpha.cos();
    let sin_a = alpha.sin();
    t.tab[a][a] = cos_a;
    t.tab[a][b] = sin_a;
    t.tab[b][a] = -sin_a;
    t.tab[b][b] = cos_a;
    t
}

/// Rotation in the XY plane by angle alpha
#[inline]
pub fn spin(alpha: f64) -> TransMatrix {
    cspin(0, 1, alpha)
}

/// Lorentz boost / hyperbolic translation along coordinate cid by distance alpha
pub fn cpush(cid: usize, alpha: f64) -> TransMatrix {
    let mut t = ID;
    let cosh_a = alpha.cosh();
    let sinh_a = alpha.sinh();
    t.tab[2][2] = cosh_a;
    t.tab[cid][cid] = cosh_a;
    t.tab[cid][2] = sinh_a;
    t.tab[2][cid] = sinh_a;
    t
}

/// Translation along X axis by alpha
#[inline]
pub fn xpush(alpha: f64) -> TransMatrix {
    cpush(0, alpha)
}

/// Translation along Y axis by alpha
#[inline]
pub fn ypush(alpha: f64) -> TransMatrix {
    cpush(1, alpha)
}

/// Translation along Z axis by alpha
#[inline]
pub fn zpush(alpha: f64) -> TransMatrix {
    cpush(2, alpha)
}

/// Push origin C0 along X axis by alpha
#[inline]
pub fn xpush0(alpha: f64) -> HyperPoint {
    HyperPoint::new(alpha.sinh(), 0.0, alpha.cosh())
}

/// Push origin C0 along Y axis by alpha
#[inline]
pub fn ypush0(alpha: f64) -> HyperPoint {
    HyperPoint::new(0.0, alpha.sinh(), alpha.cosh())
}

/// Rotate by alpha then push by distance d
#[inline]
pub fn xspinpush0(alpha: f64, d: f64) -> HyperPoint {
    spin(alpha) * xpush0(d)
}

/// Rotation taking vector H to positive X-axis
pub fn spintox(h: HyperPoint) -> TransMatrix {
    let mut t = ID;
    let r = (h.coords[0].powi(2) + h.coords[1].powi(2)).sqrt();
    if r >= 1e-15 {
        t.tab[0][0] = h.coords[0] / r;
        t.tab[0][1] = h.coords[1] / r;
        t.tab[1][0] = -h.coords[1] / r;
        t.tab[1][1] = h.coords[0] / r;
    }
    t
}

/// Inverse of spintox
pub fn rspintox(h: HyperPoint) -> TransMatrix {
    let mut t = ID;
    let r = (h.coords[0].powi(2) + h.coords[1].powi(2)).sqrt();
    if r >= 1e-15 {
        t.tab[0][0] = h.coords[0] / r;
        t.tab[0][1] = -h.coords[1] / r;
        t.tab[1][0] = h.coords[1] / r;
        t.tab[1][1] = h.coords[0] / r;
    }
    t
}

/// Check if two matrices are equal within epsilon
pub fn eq_matrix(a: &TransMatrix, b: &TransMatrix, eps: f64) -> bool {
    for i in 0..4 {
        for j in 0..4 {
            if (a.tab[i][j] - b.tab[i][j]).abs() > eps {
                return false;
            }
        }
    }
    true
}

// ---------------------------------------------------------------------------
// Projections (Klein and Poincaré disk models)
// ---------------------------------------------------------------------------

/// Klein disk projection: straight lines in hyperbolic space project to straight Euclidean lines.
/// Point (x, y, z) -> (x/z, y/z) inside the unit disk.
#[inline]
pub fn kleinize(h: HyperPoint) -> [f64; 2] {
    [h.coords[0] / h.coords[2], h.coords[1] / h.coords[2]]
}

/// Poincaré disk projection: conformal projection preserving angles.
/// Point (x, y, z) -> (x / (1 + z), y / (1 + z)) inside the unit disk.
#[inline]
pub fn poincare(h: HyperPoint) -> [f64; 2] {
    let denom = 1.0 + h.coords[2];
    [h.coords[0] / denom, h.coords[1] / denom]
}

/// Inverse Poincaré projection: disk coordinates (u, v) -> HyperPoint (x, y, z)
#[inline]
pub fn poincare_inv(u: f64, v: f64) -> HyperPoint {
    let r2 = u * u + v * v;
    let denom = (1.0 - r2).max(1e-10);
    HyperPoint::new(2.0 * u / denom, 2.0 * v / denom, (1.0 + r2) / denom)
}

/// Parametric disk projection:
/// alpha = 0 gives Klein projection (x/z, y/z)
/// alpha = 1 gives Poincaré projection (x/(1+z), y/(1+z))
#[inline]
pub fn hybrid_disk(h: HyperPoint, alpha: f64) -> [f64; 2] {
    let denom = alpha + h.coords[2];
    [h.coords[0] / denom, h.coords[1] / denom]
}

// ---------------------------------------------------------------------------
// Tests and Validations
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hyperbolic_distance() {
        let p1 = C0;
        let d = 1.5;
        let p2 = xpush0(d);
        let computed = hdist(p1, p2);
        assert!((computed - d).abs() < 1e-12, "Expected {}, got {}", d, computed);
    }

    #[test]
    fn test_lorentz_boost_invariance() {
        // Boost preserves inner product and distance between any two points
        let p1 = HyperPoint::new(0.5, 0.3, (1.0 + 0.5 * 0.5 + 0.3 * 0.3f64).sqrt());
        let p2 = HyperPoint::new(-0.2, 0.4, (1.0 + 0.2 * 0.2 + 0.4 * 0.4f64).sqrt());
        let d_orig = hdist(p1, p2);

        let t = xpush(0.8) * spin(0.5) * ypush(-0.3);
        let q1 = t * p1;
        let q2 = t * p2;
        let d_transformed = hdist(q1, q2);

        assert!(
            (d_orig - d_transformed).abs() < 1e-10,
            "Distance not invariant: {} vs {}",
            d_orig,
            d_transformed
        );
    }

    #[test]
    fn test_midpoint() {
        let p1 = xpush0(-1.0);
        let p2 = xpush0(1.0);
        let m = mid(p1, p2);
        assert!((m.x()).abs() < 1e-10);
        assert!((m.y()).abs() < 1e-10);
        assert!((m.z() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_heptagonal_triangle_constants() {
        // In {7, 3} tiling, a triangle with angles 2*PI/3, PI/7, PI/7 has opposite edge tessf
        let tessf = edge_of_triangle_with_angles(2.0 * PI / 3.0, PI / 7.0, PI / 7.0);
        // Golden value from C++ hyperrogue: 1.0905496635070873
        assert!(
            (tessf - 1.0905496635070873).abs() < 1e-10,
            "tessf mismatch: {}",
            tessf
        );

        let hcrossf = edge_of_triangle_with_angles(PI / 2.0, PI / 7.0, PI / 3.0);
        // Golden value from C++ hyperrogue: 0.6206717375563866
        assert!(
            (hcrossf - 0.6206717375563866).abs() < 1e-10,
            "hcrossf mismatch: {}",
            hcrossf
        );
    }

    #[test]
    fn test_projections() {
        let p = xpush0(0.8);
        let k = kleinize(p);
        let po = poincare(p);

        // For point on X axis: x = sinh(d), z = cosh(d)
        // Klein: tanh(d)
        // Poincaré: tanh(d / 2)
        let d: f64 = 0.8;
        assert!((k[0] - d.tanh()).abs() < 1e-12);
        assert!((po[0] - (d / 2.0).tanh()).abs() < 1e-12);

        // Inverse Poincaré check
        let reconstructed = poincare_inv(po[0], po[1]);
        assert!((reconstructed.x() - p.x()).abs() < 1e-12);
        assert!((reconstructed.z() - p.z()).abs() < 1e-12);
    }
}
