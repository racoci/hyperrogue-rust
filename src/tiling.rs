//! Discrete Hyperbolic Heptagonal and Bitruncated Tiling Engine
//!
//! Recreates the regular {7, 3} hyperbolic heptagonal tree automaton and the
//! bitruncated heptagonal-hexagonal tessellation of the hyperbolic plane,
//! matching HyperRogue's exact topology and geometry.

use crate::math::*;
use std::collections::{HashMap, VecDeque};
use std::f64::consts::PI;

pub const S7: usize = 7;
pub const S3: usize = 3;
pub const S6: usize = 6;

/// Automaton state for heptagonal grid generation
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HeptState {
    Origin,
    A,
    B,
}

/// A node in the regular {7, 3} heptagonal tiling tree
#[derive(Clone, Debug)]
pub struct Heptagon {
    pub id: usize,
    pub state: HeptState,
    pub distance: i32,
    pub moves: [Option<usize>; 7],
    pub spins: [u8; 7],
    pub c7: usize, // Index of central heptagonal cell
}

impl Heptagon {
    pub fn new(id: usize, state: HeptState, distance: i32, c7: usize) -> Self {
        Self {
            id,
            state,
            distance,
            moves: [None; 7],
            spins: [0; 7],
            c7,
        }
    }
}

/// Cell polygon type
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CellType {
    Heptagon, // Degree 7
    Hexagon,  // Degree 6
}

impl CellType {
    #[inline]
    pub fn degree(&self) -> usize {
        match self {
            CellType::Heptagon => 7,
            CellType::Hexagon => 6,
        }
    }
}

pub type CellId = usize;

/// A cell on the HyperRogue game board
#[derive(Clone, Debug)]
pub struct Cell {
    pub id: CellId,
    pub cell_type: CellType,
    pub master: usize, // Owning Heptagon ID
    pub moves: Vec<Option<CellId>>,
    pub spins: Vec<u8>,
    pub cpdist: i32,
    pub mpdist: i32,
    pub pathdist: i32,
}

impl Cell {
    pub fn new(id: CellId, cell_type: CellType, master: usize) -> Self {
        let deg = cell_type.degree();
        Self {
            id,
            cell_type,
            master,
            moves: vec![None; deg],
            spins: vec![0; deg],
            cpdist: 9999,
            mpdist: 9999,
            pathdist: 9999,
        }
    }

    #[inline]
    pub fn degree(&self) -> usize {
        self.cell_type.degree()
    }
}

/// Constants for continuous geometric embedding of {7,3} bitruncated tiling
#[derive(Clone, Debug)]
pub struct GeometryParams {
    pub tessf: f64,
    pub hcrossf: f64,
    pub crossf: f64,
    pub hexf: f64,
    pub rhexf: f64,
    pub edgelen: f64,
    pub hexvdist: f64,
    pub hexhexdist: f64,
}

impl GeometryParams {
    pub fn compute_standard_hyperbolic() -> Self {
        let alpha = TAU / (S7 as f64);
        let beta = TAU / (S3 as f64);

        let tessf = edge_of_triangle_with_angles(beta, PI / (S7 as f64), PI / (S7 as f64));
        let hcrossf = edge_of_triangle_with_angles(PI / 2.0, PI / (S7 as f64), beta / 2.0);
        let crossf = hcrossf; // BITRUNCATED

        let mut fmin = 0.0;
        let mut fmax = tessf;
        for _ in 0..100 {
            let f = (fmin + fmax) / 2.0;
            let h = xpush0(f);
            let h1 = spin(TAU / (S7 as f64)) * h;
            let h2 = xpush0(tessf - f);
            let v1 = intval(h, h1);
            let v2 = intval(h, h2);
            if v1 < v2 {
                fmin = f;
            } else {
                fmax = f;
            }
        }
        let hexf = fmin;
        let rhexf = hexf;
        let edgelen = hdist(xpush0(rhexf), xspinpush0(TAU / (S7 as f64), rhexf));
        let hexvdist = hdist(xpush0(hexf), xspinpush0(alpha / 2.0, hcrossf));
        let hexhexdist = hdist(xpush0(crossf), xspinpush0(TAU / (S7 as f64), crossf));

        Self {
            tessf,
            hcrossf,
            crossf,
            hexf,
            rhexf,
            edgelen,
            hexvdist,
            hexhexdist,
        }
    }
}

/// The discrete tiling engine managing the hyperbolic grid
pub struct Tiling {
    pub hepts: Vec<Heptagon>,
    pub cells: Vec<Cell>,
    pub geom: GeometryParams,
}

impl Default for Tiling {
    fn default() -> Self {
        Self::new()
    }
}

impl Tiling {
    pub fn new() -> Self {
        let mut tiling = Self {
            hepts: Vec::new(),
            cells: Vec::new(),
            geom: GeometryParams::compute_standard_hyperbolic(),
        };

        // Origin cell c7 (degree 7)
        let origin_c7 = Cell::new(0, CellType::Heptagon, 0);
        tiling.cells.push(origin_c7);

        // Origin heptagon (degree 7)
        let origin_h = Heptagon::new(0, HeptState::Origin, 0, 0);
        tiling.hepts.push(origin_h);

        tiling
    }

    /// Number of heptagons currently created
    #[inline]
    pub fn hept_count(&self) -> usize {
        self.hepts.len()
    }

    /// Number of cells currently created
    #[inline]
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    // Connect two heptagons symmetrically
    fn connect_hepts(&mut self, h1: usize, d1: usize, h2: usize, d2: usize) {
        self.hepts[h1].moves[d1] = Some(h2);
        self.hepts[h1].spins[d1] = d2 as u8;
        self.hepts[h2].moves[d2] = Some(h1);
        self.hepts[h2].spins[d2] = d1 as u8;
    }

    // Connect two cells symmetrically
    fn connect_cells(&mut self, c1: usize, d1: usize, c2: usize, d2: usize) {
        self.cells[c1].moves[d1] = Some(c2);
        self.cells[c1].spins[d1] = d2 as u8;
        self.cells[c2].moves[d2] = Some(c1);
        self.cells[c2].spins[d2] = d1 as u8;
    }

    fn add_spin(&mut self, h: usize, d: usize, from: usize, rot: i32, spin_offset: i32) {
        let rot_fixed = ((rot % 7 + 7) % 7) as usize;
        let h1 = self.create_step(from, rot_fixed);
        let from_spin = self.hepts[from].spins[rot_fixed] as i32;
        let fr = (((from_spin + spin_offset) % 7 + 7) % 7) as usize;
        let target = self.hepts[from].moves[rot_fixed].unwrap();
        self.connect_hepts(h, d, target, fr);
    }

    fn build_heptagon(&mut self, parent: usize, d: usize, s: HeptState) -> usize {
        let new_id = self.hepts.len();
        let new_cell_id = self.cells.len();

        let new_cell = Cell::new(new_cell_id, CellType::Heptagon, new_id);
        self.cells.push(new_cell);

        let parent_dist = self.hepts[parent].distance;
        let new_h = Heptagon::new(new_id, s, parent_dist + 1, new_cell_id);
        self.hepts.push(new_h);

        self.connect_hepts(new_id, 0, parent, d);
        new_id
    }

    /// Create or retrieve neighbor heptagon in direction d
    pub fn create_step(&mut self, h: usize, d: usize) -> usize {
        let d = d % 7;
        if let Some(target) = self.hepts[h].moves[d] {
            return target;
        }

        let state = self.hepts[h].state;
        if state == HeptState::Origin {
            self.build_heptagon(h, d, HeptState::A);
        } else if d == 1 {
            let p0 = self.hepts[h].moves[0].unwrap();
            let sp0 = self.hepts[h].spins[0] as i32;
            self.add_spin(h, d, p0, sp0 - 1, -1);
        } else if d == 6 {
            let p0 = self.hepts[h].moves[0].unwrap();
            let sp0 = self.hepts[h].spins[0] as i32;
            self.add_spin(h, d, p0, sp0 + 1, 1);
        } else if d == 2 {
            let p0 = self.hepts[h].moves[0].unwrap();
            let sp0 = self.hepts[h].spins[0] as i32;
            let rot = ((sp0 - 1 + 7) % 7) as usize;
            self.create_step(p0, rot);
            let uncle = self.hepts[p0].moves[rot].unwrap();
            let uncle_spin = self.hepts[p0].spins[rot] as i32;
            self.add_spin(h, d, uncle, 5 + uncle_spin, -1);
        } else if d == 5 && state == HeptState::B {
            let p0 = self.hepts[h].moves[0].unwrap();
            let sp0 = self.hepts[h].spins[0] as i32;
            let rot = ((sp0 + 1) % 7) as usize;
            self.create_step(p0, rot);
            let uncle = self.hepts[p0].moves[rot].unwrap();
            let uncle_spin = self.hepts[p0].spins[rot] as i32;
            self.add_spin(h, d, uncle, 2 + uncle_spin, 1);
        } else {
            let next_s = if d == 5 || (state == HeptState::B && d == 4) {
                HeptState::B
            } else {
                HeptState::A
            };
            self.build_heptagon(h, d, next_s);
        }

        self.hepts[h].moves[d].unwrap()
    }

    /// Create or retrieve neighbor cell in direction d
    pub fn create_mov(&mut self, c: CellId, d: usize) -> CellId {
        let deg = self.cells[c].degree();
        let d = d % deg;
        if let Some(target) = self.cells[c].moves[d] {
            return target;
        }

        self.find_cell_connection(c, d);
        self.cells[c].moves[d].unwrap()
    }

    fn find_cell_connection(&mut self, c: CellId, d: usize) {
        let master_hept = self.cells[c].master;
        let is_hept_cell = c == self.hepts[master_hept].c7;

        if is_hept_cell {
            // Exploring from a heptagon cell c7: create a new hexagon (degree 6)
            let new_hex_id = self.cells.len();
            let new_hex = Cell::new(new_hex_id, CellType::Hexagon, master_hept);
            self.cells.push(new_hex);

            let mut cur_h = master_hept;
            let mut cur_spin = d;

            for u in (0..6).step_by(2) {
                let hept_c7 = self.hepts[cur_h].c7;
                self.connect_cells(hept_c7, cur_spin, new_hex_id, u);

                // Advance walker: hs = hs + 3 + wstep - 4
                cur_spin = (cur_spin + 3) % 7;
                self.create_step(cur_h, cur_spin);
                let next_spin = self.hepts[cur_h].spins[cur_spin] as usize;
                cur_h = self.hepts[cur_h].moves[cur_spin].unwrap();
                cur_spin = (next_spin + 7 - 4) % 7;
            }
        } else {
            // Exploring from a hexagon cell along an odd edge (connecting to adjacent hexagon)
            // cw2 = cw - 1 + wstep - 1 + wstep - 1
            let mut walker = CellWalker::new(c, d);
            walker.turn(-1, self);
            walker.step(self);
            walker.turn(-1, self);
            walker.step(self);
            walker.turn(-1, self);

            self.connect_cells(c, d, walker.at, walker.spin);
        }
    }

    /// Expand the bitruncated board to distance radius around origin cell
    pub fn expand_disk(&mut self, radius: i32) {
        let mut q = VecDeque::new();
        let mut dist: HashMap<CellId, i32> = HashMap::new();

        let origin_c7 = 0;
        dist.insert(origin_c7, 0);
        q.push_back(origin_c7);

        while let Some(c) = q.pop_front() {
            let d = dist[&c];
            self.cells[c].cpdist = d;
            self.cells[c].mpdist = d;

            if d < radius {
                let deg = self.cells[c].degree();
                for i in 0..deg {
                    let next_c = self.create_mov(c, i);
                    if !dist.contains_key(&next_c) {
                        dist.insert(next_c, d + 1);
                        q.push_back(next_c);
                    }
                }
            }
        }
    }

    /// Run BFS from target cell to compute cpdist
    pub fn bfs_distances(&mut self, start_cell: CellId, max_range: i32) -> Vec<CellId> {
        let mut q = VecDeque::new();
        let mut visited = vec![false; self.cells.len()];
        let mut ordered = Vec::new();

        // Reset cpdist
        for c in &mut self.cells {
            c.cpdist = 9999;
        }

        self.cells[start_cell].cpdist = 0;
        visited[start_cell] = true;
        q.push_back(start_cell);
        ordered.push(start_cell);

        while let Some(c) = q.pop_front() {
            let d = self.cells[c].cpdist;
            if d < max_range {
                let deg = self.cells[c].degree();
                for i in 0..deg {
                    let n = self.create_mov(c, i);
                    if n >= visited.len() {
                        visited.resize(n + 1, false);
                    }
                    if !visited[n] {
                        visited[n] = true;
                        self.cells[n].cpdist = d + 1;
                        q.push_back(n);
                        ordered.push(n);
                    }
                }
            }
        }

        ordered
    }

    /// Spin angle for cell edge d: PI - d * TAU / degree
    #[inline]
    pub fn spin_angle(&self, c: CellId, d: usize) -> f64 {
        let deg = self.cells[c].degree() as f64;
        PI - (d as f64) * TAU / deg
    }

    /// Transformation matrix from cell center to edge d: spin(spin_angle(c, d))
    #[inline]
    pub fn ddspin(&self, c: CellId, d: usize, bonus: f64) -> TransMatrix {
        spin(self.spin_angle(c, d) + bonus)
    }

    /// Inverse transformation matrix from edge d to cell center: spin(-spin_angle(c, d) + bonus)
    #[inline]
    pub fn iddspin(&self, c: CellId, d: usize, bonus: f64) -> TransMatrix {
        spin(bonus - self.spin_angle(c, d))
    }

    /// Distance between cell centers for edge d
    #[inline]
    pub fn spacedist(&self, c: CellId, d: usize) -> f64 {
        if self.cells[c].cell_type == CellType::Hexagon && (d & 1 == 1) {
            self.geom.hexhexdist
        } else {
            self.geom.crossf
        }
    }

    /// Transformation matrix mapping cell coordinates to neighbor coordinates
    pub fn adj(&mut self, c: CellId, d: usize) -> TransMatrix {
        let dist = self.spacedist(c, d);
        let n = self.create_mov(c, d);
        let n_spin = self.cells[c].spins[d] as usize;

        self.ddspin(c, d, 0.0) * xpush(dist) * self.iddspin(n, n_spin, PI)
    }

    /// Get the hyperbolic coordinates of polygon vertex cid of cell c
    pub fn get_corner(&self, c: CellId, cid: usize) -> HyperPoint {
        let cid = cid % self.cells[c].degree();
        let deg = self.cells[c].degree();
        if deg == 7 {
            self.ddspin(c, cid, PI / 7.0) * xpush0(self.geom.rhexf)
        } else {
            self.ddspin(c, cid, PI / 6.0) * xpush0(self.geom.hexvdist)
        }
    }
}

/// Navigation walker on the cell graph
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CellWalker {
    pub at: CellId,
    pub spin: usize,
    pub mirrored: bool,
}

impl CellWalker {
    pub fn new(at: CellId, spin: usize) -> Self {
        Self {
            at,
            spin,
            mirrored: false,
        }
    }

    /// Step forward in direction spin
    pub fn step(&mut self, tiling: &mut Tiling) {
        let n = tiling.create_mov(self.at, self.spin);
        let n_spin = tiling.cells[self.at].spins[self.spin] as usize;
        self.at = n;
        self.spin = n_spin;
    }

    /// Turn clockwise (positive) or counterclockwise (negative)
    pub fn turn(&mut self, delta: i32, tiling: &Tiling) {
        let deg = tiling.cells[self.at].degree() as i32;
        let d = if self.mirrored { -delta } else { delta };
        self.spin = (((self.spin as i32 + d) % deg + deg) % deg) as usize;
    }

    /// Turn left by 1 step
    #[inline]
    pub fn turn_left(&mut self, tiling: &Tiling) {
        self.turn(1, tiling);
    }

    /// Turn right by 1 step
    #[inline]
    pub fn turn_right(&mut self, tiling: &Tiling) {
        self.turn(-1, tiling);
    }

    /// Look ahead at target cell without stepping
    pub fn peek(&self, tiling: &mut Tiling) -> CellId {
        tiling.create_mov(self.at, self.spin)
    }
}

// ---------------------------------------------------------------------------
// Tests and Validations
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heptagon_tree_counts() {
        let mut tiling = Tiling::new();

        // Expand heptagons up to distance 4
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

        // Validate against C++ golden counts:
        // Dist 0: 1
        // Dist 1: 7
        // Dist 2: 21
        // Dist 3: 56
        // Dist 4: 147
        assert_eq!(counts[&0], 1);
        assert_eq!(counts[&1], 7);
        assert_eq!(counts[&2], 21);
        assert_eq!(counts[&3], 56);
        assert_eq!(counts[&4], 147);
        assert_eq!(tiling.hept_count(), 232);

        // Verify bidirectional symmetry
        for h in 0..tiling.hept_count() {
            for d in 0..7 {
                if let Some(n) = tiling.hepts[h].moves[d] {
                    let sp = tiling.hepts[h].spins[d] as usize;
                    assert_eq!(tiling.hepts[n].moves[sp], Some(h));
                    assert_eq!(tiling.hepts[n].spins[sp], d as u8);
                }
            }
        }
    }

    #[test]
    fn test_bitruncated_cell_counts() {
        let mut tiling = Tiling::new();
        tiling.expand_disk(5);

        // Validate against C++ golden cumulative counts:
        // Dist 0: 1 (cumulative: 1)
        // Dist 1: 7 (cumulative: 8)
        // Dist 2: 14 (cumulative: 22)
        // Dist 3: 28 (cumulative: 50)
        // Dist 4: 49 (cumulative: 99)
        // Dist 5: 84 (cumulative: 183)
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

        let cumulative_5: i32 = (0..=5).map(|d| counts[&d]).sum();
        assert_eq!(cumulative_5, 183);

        // Verify bidirectional symmetry of all cells
        for c in 0..tiling.cell_count() {
            let deg = tiling.cells[c].degree();
            for d in 0..deg {
                if let Some(n) = tiling.cells[c].moves[d] {
                    let sp = tiling.cells[c].spins[d] as usize;
                    assert_eq!(tiling.cells[n].moves[sp], Some(c));
                    assert_eq!(tiling.cells[n].spins[sp], d as u8);
                }
            }
        }
    }

    #[test]
    fn test_walker_closed_cycle() {
        let mut tiling = Tiling::new();

        // On a heptagon, walking 7 forward steps after turning should return to start or form closed cycles
        // A hexagon has 6 edges. In bitruncated tiling, around each vertex 3 polygons meet:
        // Heptagon - Hexagon - Hexagon (7.6.6 bitruncated heptagonal tiling!)
        let mut walker = CellWalker::new(0, 0);
        let start_c = walker.at;
        // Step to hexagon
        walker.step(&mut tiling);
        assert_eq!(tiling.cells[walker.at].cell_type, CellType::Hexagon);

        // Step back
        walker.step(&mut tiling);
        assert_eq!(walker.at, start_c);
    }

    #[test]
    fn test_adj_matrix_consistency() {
        let mut tiling = Tiling::new();
        let adj_mat = tiling.adj(0, 0);

        // The adj matrix maps origin C0 to neighbor's center
        let neighbor_center = adj_mat * C0;
        let dist = hdist(C0, neighbor_center);
        assert!(
            (dist - tiling.geom.crossf).abs() < 1e-10,
            "Center distance mismatch: {} vs {}",
            dist,
            tiling.geom.crossf
        );
    }
}
