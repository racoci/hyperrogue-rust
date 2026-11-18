//! Procedural World Generation and Land Partitioning
//!
//! Generates lands, Great Walls (barriers), terrain features, items, treasures,
//! and monsters on the hyperbolic bitruncated grid.

use crate::content::*;
use crate::tiling::*;
use rand_mt::Mt19937GenRand32;

/// Mersenne Twister PRNG matching C++ std::mt19937 in HyperRogue
pub struct WorldRng {
    pub rng: Mt19937GenRand32,
}

impl WorldRng {
    pub fn new(seed: u32) -> Self {
        Self {
            rng: Mt19937GenRand32::new(seed),
        }
    }

    /// Random integer in [0, max_val) matching HyperRogue hrand
    pub fn hrand(&mut self, max_val: u32) -> u32 {
        if max_val <= 1 {
            return 0;
        }
        let rand_val = self.rng.next_u32() & 0x7FFF_FFFF;
        rand_val % max_val
    }

    /// Random float in [0.0, 1.0)
    pub fn randf(&mut self) -> f64 {
        let val = self.rng.next_u32();
        (val as f64) / (u32::MAX as f64 + 1.0)
    }

    /// Returns true with probability p
    pub fn chance(&mut self, p: f64) -> bool {
        self.randf() < p
    }
}

/// Dynamic game state on an individual cell
#[derive(Clone, Debug)]
pub struct BoardCell {
    pub id: CellId,
    pub land: Land,
    pub wall: Wall,
    pub monster: Monster,
    pub item: Item,
    pub heat: f64,
    pub wparam: i32,
    pub stuntime: u8,
    pub hitpoints: i32,
    pub mondir: usize,
    pub is_generated: bool,
}

impl BoardCell {
    pub fn new(id: CellId) -> Self {
        Self {
            id,
            land: Land::None,
            wall: Wall::None,
            monster: Monster::None,
            item: Item::None,
            heat: 0.0,
            wparam: 0,
            stuntime: 0,
            hitpoints: 1,
            mondir: 0,
            is_generated: false,
        }
    }
}

/// The game world containing the hyperbolic board and world generator
pub struct World {
    pub tiling: Tiling,
    pub board: Vec<BoardCell>,
    pub rng: WorldRng,
    pub seed: u32,
}

impl World {
    pub fn new(seed: u32) -> Self {
        let tiling = Tiling::new();
        let mut world = Self {
            tiling,
            board: Vec::new(),
            rng: WorldRng::new(seed),
            seed,
        };

        // Initialize board cell for origin
        world.ensure_board_capacity(world.tiling.cell_count());
        world.generate_area(0, 7);
        world
    }

    pub fn ensure_board_capacity(&mut self, count: usize) {
        while self.board.len() < count {
            let id = self.board.len();
            self.board.push(BoardCell::new(id));
        }
    }

    /// Determine land for a cell based on hyperbolic distance and sector
    pub fn assign_land(&self, c: CellId) -> Land {
        let dist = self.tiling.cells[c].cpdist;
        if dist <= 6 {
            return Land::Crossroads;
        }

        // Great Wall barrier ring at distance 7
        if dist == 7 {
            // Every 5th cell is a gate opening to enter the new land
            if c % 5 == 0 {
                return Land::Crossroads;
            } else {
                return Land::Barrier;
            }
        }

        // Sector partitioning outside the Great Wall
        // Determine branch from the master heptagon ID
        let master = self.tiling.cells[c].master;
        let sector = (master * 7 + (c % 13)) % 8;
        match sector {
            0 => Land::Ice,
            1 => Land::Caves,
            2 => Land::Desert,
            3 => Land::Jungle,
            4 => Land::Alchemist,
            5 => Land::Graveyard,
            6 => Land::DryForest,
            _ => Land::Motion,
        }
    }

    /// Procedural content generator for an individual cell
    pub fn generate_cell_content(&mut self, c: CellId) {
        if self.board[c].is_generated {
            return;
        }

        let land = self.assign_land(c);
        self.board[c].land = land;

        if c == 0 {
            // Origin starting cell: Rogue starts here, clear and safe!
            self.board[c].land = Land::Crossroads;
            self.board[c].wall = Wall::None;
            self.board[c].monster = Monster::None;
            self.board[c].item = Item::None;
            self.board[c].is_generated = true;
            return;
        }

        match land {
            Land::Barrier => {
                self.board[c].wall = Wall::Barrier;
                self.board[c].monster = Monster::None;
                self.board[c].item = Item::None;
            }
            Land::Crossroads => {
                self.board[c].wall = Wall::None;
                // Occasional Hyperstone or Orb
                if self.rng.chance(0.04) {
                    self.board[c].item = Item::Hyperstone;
                } else if self.rng.chance(0.02) {
                    let orbs = [
                        Item::OrbSpeed,
                        Item::OrbFlash,
                        Item::OrbLightning,
                        Item::OrbShield,
                        Item::OrbTeleport,
                    ];
                    let idx = self.rng.hrand(orbs.len() as u32) as usize;
                    self.board[c].item = orbs[idx];
                }
            }
            Land::Ice => {
                self.board[c].heat = -1.0;
                if self.rng.chance(0.35) {
                    self.board[c].wall = Wall::Icewall;
                    if self.rng.chance(0.12) {
                        self.board[c].item = Item::Diamond;
                    }
                } else {
                    self.board[c].wall = Wall::None;
                    if self.rng.chance(0.05) {
                        self.board[c].item = Item::Diamond;
                    } else if self.rng.chance(0.02) {
                        self.board[c].item = Item::OrbWinter;
                    } else if self.rng.chance(0.04) {
                        if self.rng.chance(0.6) {
                            self.board[c].monster = Monster::Wolf;
                        } else {
                            self.board[c].monster = Monster::Yeti;
                        }
                    }
                }
            }
            Land::Caves => {
                if self.rng.chance(0.40) {
                    self.board[c].wall = Wall::Cavewall;
                    if self.rng.chance(0.10) {
                        self.board[c].item = Item::Gold;
                    }
                } else {
                    self.board[c].wall = Wall::Cavefloor;
                    if self.rng.chance(0.05) {
                        self.board[c].item = Item::Gold;
                    } else if self.rng.chance(0.02) {
                        self.board[c].item = Item::OrbDigging;
                    } else if self.rng.chance(0.04) {
                        if self.rng.chance(0.5) {
                            self.board[c].monster = Monster::Troll;
                        } else {
                            self.board[c].monster = Monster::Goblin;
                        }
                    }
                }
            }
            Land::Desert => {
                if self.rng.chance(0.25) {
                    self.board[c].wall = Wall::Dune;
                } else {
                    self.board[c].wall = Wall::None;
                    if self.rng.chance(0.06) {
                        self.board[c].item = Item::Spice;
                    } else if self.rng.chance(0.02) {
                        self.board[c].item = Item::OrbSpeed;
                    } else if self.rng.chance(0.03) {
                        if self.rng.chance(0.5) {
                            self.board[c].monster = Monster::Worm;
                        } else {
                            self.board[c].monster = Monster::Desertman;
                        }
                    }
                }
            }
            Land::Jungle => {
                self.board[c].wall = Wall::None;
                if self.rng.chance(0.05) {
                    self.board[c].item = Item::Ruby;
                } else if self.rng.chance(0.02) {
                    self.board[c].item = Item::OrbLife;
                } else if self.rng.chance(0.04) {
                    if self.rng.chance(0.4) {
                        self.board[c].monster = Monster::IvyRoot;
                    } else {
                        self.board[c].monster = Monster::Monkey;
                    }
                }
            }
            Land::Alchemist => {
                let pool = self.rng.hrand(3);
                if pool == 0 {
                    self.board[c].wall = Wall::FloorA; // Red slime
                } else if pool == 1 {
                    self.board[c].wall = Wall::FloorB; // Blue slime
                } else {
                    self.board[c].wall = Wall::None;
                }

                if self.rng.chance(0.05) {
                    self.board[c].item = Item::Elixir;
                } else if self.rng.chance(0.04) {
                    self.board[c].monster = Monster::Slime;
                }
            }
            Land::Graveyard => {
                if self.rng.chance(0.20) {
                    self.board[c].wall = Wall::AncientGrave;
                } else if self.rng.chance(0.10) {
                    self.board[c].wall = Wall::FreshGrave;
                } else {
                    self.board[c].wall = Wall::None;
                    if self.rng.chance(0.06) {
                        self.board[c].item = Item::Bone;
                    } else if self.rng.chance(0.04) {
                        let r = self.rng.hrand(3);
                        match r {
                            0 => self.board[c].monster = Monster::Zombie,
                            1 => self.board[c].monster = Monster::Ghost,
                            _ => self.board[c].monster = Monster::Necromancer,
                        }
                    }
                }
            }
            Land::DryForest => {
                if self.rng.chance(0.20) {
                    self.board[c].wall = Wall::SmallTree;
                } else if self.rng.chance(0.10) {
                    self.board[c].wall = Wall::BigTree;
                } else {
                    self.board[c].wall = Wall::None;
                    if self.rng.chance(0.05) {
                        self.board[c].item = Item::FernFlower;
                    } else if self.rng.chance(0.02) {
                        self.board[c].item = Item::OrbFire;
                    } else if self.rng.chance(0.03) {
                        self.board[c].wall = Wall::Fire;
                    }
                }
            }
            _ => {
                self.board[c].wall = Wall::None;
                if self.rng.chance(0.05) {
                    self.board[c].item = land.info().treasure;
                }
            }
        }

        self.board[c].is_generated = true;
    }

    /// Generate area around a cell up to radius
    pub fn generate_area(&mut self, _center: CellId, radius: i32) {
        self.tiling.expand_disk(radius);
        self.ensure_board_capacity(self.tiling.cell_count());

        for c in 0..self.tiling.cell_count() {
            if self.tiling.cells[c].cpdist <= radius {
                self.generate_cell_content(c);
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
    fn test_rng_determinism() {
        let mut rng1 = WorldRng::new(12345);
        let mut rng2 = WorldRng::new(12345);
        for _ in 0..100 {
            assert_eq!(rng1.hrand(100), rng2.hrand(100));
            assert_eq!(rng1.randf(), rng2.randf());
        }
    }

    #[test]
    fn test_world_start_area() {
        let world = World::new(42);
        // Origin must be clear and Crossroads
        assert_eq!(world.board[0].land, Land::Crossroads);
        assert_eq!(world.board[0].wall, Wall::None);
        assert_eq!(world.board[0].monster, Monster::None);

        // Distance <= 6 is Crossroads
        for c in 0..world.tiling.cell_count() {
            if world.tiling.cells[c].cpdist <= 6 {
                assert_eq!(world.board[c].land, Land::Crossroads);
            }
        }
    }

    #[test]
    fn test_great_walls_exist() {
        let mut world = World::new(42);
        world.generate_area(0, 8);

        let mut barrier_found = false;
        for c in 0..world.tiling.cell_count() {
            if world.board[c].land == Land::Barrier {
                assert_eq!(world.board[c].wall, Wall::Barrier);
                barrier_found = true;
            }
        }
        assert!(barrier_found, "Great Wall barriers must generate around Crossroads");
    }
}
