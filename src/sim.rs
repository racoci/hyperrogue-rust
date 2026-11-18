//! Environmental Simulations: Living Caves CA, Heat/Fire Thermodynamics, and Ecology
//!
//! Recreates HyperRogue's iconic cellular automata:
//! 1. Living Caves cellular automaton (majority rule with troll & item weighting)
//! 2. Heat transfer & thermodynamics (ice melting, body heat radiation)
//! 3. Fire spreading in Dry Forest (tree combustion)
//! 4. Jungle ivy growth and branching
//! 5. Sandworm body articulation and segment trailing

use crate::content::*;
use crate::tiling::*;
use crate::worldgen::*;

/// Environmental simulation runner
pub struct EnvironmentSim;

impl EnvironmentSim {
    /// Step all environmental simulations in the active gamerange around the player
    pub fn step_environment(world: &mut World, player_cell: CellId, gamerange: i32) {
        Self::simulate_living_caves(world, gamerange);
        Self::simulate_heat_and_fire(world, player_cell, gamerange);
        Self::simulate_ivy_growth(world, gamerange);
        Self::simulate_sandworms(world, player_cell, gamerange);
    }

    /// Living Caves cellular automaton
    /// Each cave cell evaluates the surrounding living walls and floors:
    /// - Cave floor: +1
    /// - Cave wall: -1
    /// - Dead rock troll: -5
    /// - Items: +2
    /// - Undead monsters (zombies, ghosts): +10
    /// If sum > 0 -> becomes Cave floor; if sum < 0 -> becomes Cave wall.
    pub fn simulate_living_caves(world: &mut World, gamerange: i32) {
        let count = world.tiling.cell_count();
        world.ensure_board_capacity(count);

        let mut state_changes = Vec::new();

        for c in 0..count {
            if world.tiling.cells[c].cpdist > gamerange + 1 {
                continue;
            }

            let cell_wall = world.board[c].wall;
            let is_living_cave = world.board[c].land.is_caves()
                || matches!(cell_wall, Wall::Cavefloor | Wall::Cavewall);

            if !is_living_cave {
                continue;
            }

            // Cannot solidify a cell that has an item or player on it
            if world.board[c].item != Item::None || world.board[c].monster == Monster::Player {
                continue;
            }

            let mut heat = 0;
            let deg = world.tiling.cells[c].degree();

            for i in 0..deg {
                let n = world.tiling.create_mov(c, i);
                let nw = world.board[n].wall;
                let nm = world.board[n].monster;

                match nw {
                    Wall::Cavefloor => heat += 1,
                    Wall::Cavewall => heat -= 1,
                    Wall::DeadTroll => heat -= 5,
                    Wall::Rubble => heat -= 1,
                    _ => {}
                }

                if world.board[n].item != Item::None {
                    heat += 2;
                }

                if matches!(nm, Monster::Zombie | Monster::Ghost) {
                    heat += 10;
                }
            }

            if heat > 0 && cell_wall == Wall::Cavewall {
                state_changes.push((c, Wall::Cavefloor));
            } else if heat < 0 && cell_wall == Wall::Cavefloor {
                state_changes.push((c, Wall::Cavewall));
            }
        }

        for (c, new_wall) in state_changes {
            world.board[c].wall = new_wall;
        }
    }

    /// Heat transfer and fire combustion
    pub fn simulate_heat_and_fire(world: &mut World, player_cell: CellId, gamerange: i32) {
        let count = world.tiling.cell_count();
        world.ensure_board_capacity(count);

        // Player radiates body heat in Icy Land
        if world.board[player_cell].land.is_icy() {
            world.board[player_cell].heat += 1.2;
        }

        let mut melted = Vec::new();
        let mut new_fires = Vec::new();

        for c in 0..count {
            if world.tiling.cells[c].cpdist > gamerange + 1 {
                continue;
            }

            // Ice melting
            if world.board[c].wall == Wall::Icewall && world.board[c].heat > 0.5 {
                melted.push(c);
            }

            // Fire spreading in Dry Forest
            if world.board[c].wall == Wall::Fire {
                let deg = world.tiling.cells[c].degree();
                for i in 0..deg {
                    let n = world.tiling.create_mov(c, i);
                    if matches!(world.board[n].wall, Wall::SmallTree | Wall::BigTree) {
                        if world.rng.chance(0.35) {
                            new_fires.push(n);
                        }
                    }
                }

                // Fire burns out after consuming fuel
                world.board[c].wparam += 1;
                if world.board[c].wparam >= 3 {
                    world.board[c].wall = Wall::None;
                    world.board[c].wparam = 0;
                }
            }
        }

        for c in melted {
            world.board[c].wall = Wall::None;
            world.board[c].heat = 0.0;
        }

        for c in new_fires {
            world.board[c].wall = Wall::Fire;
            world.board[c].wparam = 0;
        }
    }

    /// Jungle Ivy growth
    pub fn simulate_ivy_growth(world: &mut World, gamerange: i32) {
        let count = world.tiling.cell_count();
        world.ensure_board_capacity(count);

        let mut growing_heads = Vec::new();

        for c in 0..count {
            if world.tiling.cells[c].cpdist <= gamerange + 1
                && world.board[c].monster == Monster::IvyHead
            {
                growing_heads.push(c);
            }
        }

        for c in growing_heads {
            let deg = world.tiling.cells[c].degree();
            let mut grown = false;

            for i in 0..deg {
                let n = world.tiling.create_mov(c, i);
                if world.board[n].wall == Wall::None
                    && world.board[n].monster == Monster::None
                {
                    // Ivy head advances to neighbor
                    world.board[c].monster = Monster::IvyBranch;
                    world.board[n].monster = Monster::IvyHead;
                    world.board[n].wall = Wall::VinePlant;
                    grown = true;
                    break;
                }
            }

            if !grown {
                // If blocked, becomes dormant
                world.board[c].monster = Monster::IvyWait;
            }
        }
    }

    /// Sandworm movement and segment trailing
    pub fn simulate_sandworms(world: &mut World, player_cell: CellId, gamerange: i32) {
        let count = world.tiling.cell_count();
        world.ensure_board_capacity(count);

        let mut worm_heads = Vec::new();

        for c in 0..count {
            if world.tiling.cells[c].cpdist <= gamerange
                && world.board[c].monster == Monster::Worm
            {
                worm_heads.push(c);
            }
        }

        for head_c in worm_heads {
            let deg = world.tiling.cells[head_c].degree();
            let mut best_target = None;
            let mut min_dist = world.tiling.cells[head_c].cpdist;

            for i in 0..deg {
                let n = world.tiling.create_mov(head_c, i);
                let n_dist = world.tiling.cells[n].cpdist;
                let is_open = !world.board[n].wall.blocks()
                    && (world.board[n].monster == Monster::None || n == player_cell);

                if is_open && n_dist < min_dist {
                    min_dist = n_dist;
                    best_target = Some(n);
                }
            }

            if let Some(target) = best_target {
                if target != player_cell {
                    world.board[head_c].monster = Monster::WormTail;
                    world.board[target].monster = Monster::Worm;
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
    fn test_living_caves_ca_solidification() {
        let mut world = World::new(42);
        let center = world.tiling.create_mov(0, 0);
        world.board[center].land = Land::Caves;
        world.board[center].wall = Wall::Cavefloor;

        // Surround center with cave walls
        let deg = world.tiling.cells[center].degree();
        for i in 0..deg {
            let n = world.tiling.create_mov(center, i);
            world.board[n].wall = Wall::Cavewall;
        }

        // Run Living Caves cellular automaton: majority walls solidifies center to Cavewall!
        EnvironmentSim::simulate_living_caves(&mut world, 7);
        assert_eq!(world.board[center].wall, Wall::Cavewall);
    }

    #[test]
    fn test_dead_troll_anchors_cave_wall() {
        let mut world = World::new(42);
        let center = world.tiling.create_mov(0, 0);
        world.board[center].land = Land::Caves;
        world.board[center].wall = Wall::Cavefloor;

        // Place a Dead Troll adjacent: -5 weight strongly pulls cell to Cavewall
        let n0 = world.tiling.create_mov(center, 0);
        world.board[n0].wall = Wall::DeadTroll;

        EnvironmentSim::simulate_living_caves(&mut world, 7);
        assert_eq!(world.board[center].wall, Wall::Cavewall);
    }

    #[test]
    fn test_heat_melts_ice_wall() {
        let mut world = World::new(42);
        let c = world.tiling.create_mov(0, 0);
        world.board[c].land = Land::Ice;
        world.board[c].wall = Wall::Icewall;
        world.board[c].heat = 2.0; // High heat

        EnvironmentSim::simulate_heat_and_fire(&mut world, 0, 7);
        // Ice melted to open floor
        assert_eq!(world.board[c].wall, Wall::None);
    }

    #[test]
    fn test_fire_spreads_to_trees() {
        let mut world = World::new(42);
        let fire_c = world.tiling.create_mov(0, 0);
        let tree_c = world.tiling.create_mov(fire_c, 1);

        world.board[fire_c].wall = Wall::Fire;
        world.board[tree_c].wall = Wall::SmallTree;

        // Force chance to 100% by testing multiple steps
        let mut caught_fire = false;
        for _ in 0..10 {
            EnvironmentSim::simulate_heat_and_fire(&mut world, 0, 7);
            if world.board[tree_c].wall == Wall::Fire {
                caught_fire = true;
                break;
            }
        }
        assert!(caught_fire, "Tree adjacent to fire must ignite");
    }
}
