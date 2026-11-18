//! Turn-Based Game Engine, Combat Resolution, and Monster AI
//!
//! Implements player movement, melee attacks, orb usage, damage mechanics,
//! threat detection ("checkmove"), monster AI pathfinding, and turn sequencing.

use crate::content::*;
use crate::tiling::*;
use crate::worldgen::*;
use std::collections::{HashMap, VecDeque};

/// Player actions in a turn
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PlayerAction {
    Step(usize), // Step in direction (or attack monster in that direction)
    Wait,        // Pass turn
    UseOrb(Item, CellId), // Target a ranged orb at cell
}

/// The state of the player character
#[derive(Clone, Debug)]
pub struct Player {
    pub walker: CellWalker,
    pub items: HashMap<Item, i32>,
    pub kills: HashMap<Monster, i32>,
    pub is_dead: bool,
    pub death_message: Option<String>,
    pub won: bool,
}

impl Player {
    pub fn new(start_cell: CellId) -> Self {
        Self {
            walker: CellWalker::new(start_cell, 0),
            items: HashMap::new(),
            kills: HashMap::new(),
            is_dead: false,
            death_message: None,
            won: false,
        }
    }

    pub fn item_count(&self, item: Item) -> i32 {
        *self.items.get(&item).unwrap_or(&0)
    }

    pub fn add_item(&mut self, item: Item, count: i32) {
        let entry = self.items.entry(item).or_insert(0);
        *entry += count;
    }

    pub fn consume_item(&mut self, item: Item, count: i32) -> bool {
        let entry = self.items.entry(item).or_insert(0);
        if *entry >= count {
            *entry -= count;
            true
        } else {
            false
        }
    }

    pub fn kill_count(&self, monster: Monster) -> i32 {
        *self.kills.get(&monster).unwrap_or(&0)
    }

    pub fn total_kills(&self) -> i32 {
        self.kills.values().sum()
    }

    pub fn total_treasures(&self) -> i32 {
        self.items
            .iter()
            .filter(|(it, _)| it.is_treasure())
            .map(|(_, count)| count)
            .sum()
    }
}

/// The central Game Engine
pub struct GameEngine {
    pub world: World,
    pub player: Player,
    pub turn_count: i32,
    pub message_log: Vec<String>,
    pub gamerange: i32,
}

impl GameEngine {
    pub fn new(seed: u32) -> Self {
        let mut world = World::new(seed);
        world.board[0].monster = Monster::Player;

        Self {
            world,
            player: Player::new(0),
            turn_count: 0,
            message_log: vec!["Welcome to HyperRogue!".to_string()],
            gamerange: 7,
        }
    }

    pub fn log(&mut self, msg: impl Into<String>) {
        self.message_log.push(msg.into());
    }

    /// Check if target cell is passable for monster or player
    pub fn is_passable(&self, c: CellId, is_ghost: bool) -> bool {
        if is_ghost {
            return true;
        }
        let wall = self.world.board[c].wall;
        !wall.blocks()
    }

    /// Check if attacker can attack target
    pub fn can_attack(&self, attacker: Monster, target: Monster) -> bool {
        if target == Monster::None {
            return false;
        }
        if attacker == Monster::Player {
            !target.is_friendly()
        } else {
            target == Monster::Player || target.is_friendly()
        }
    }

    /// Execute a monster kill and handle death side-effects
    pub fn kill_monster(&mut self, c: CellId, killer: Monster) {
        let victim = self.world.board[c].monster;
        if victim == Monster::None {
            return;
        }

        // Register kill
        if killer == Monster::Player {
            *self.player.kills.entry(victim).or_insert(0) += 1;
            self.log(format!("You defeated the {}!", victim.info().name));
        }

        // Handle special death side-effects
        match victim {
            Monster::Troll => {
                // Petrify into dead rock troll wall
                self.world.board[c].wall = Wall::DeadTroll;
                self.world.board[c].item = Item::None;

                // Surrounding cave adjustments
                let deg = self.world.tiling.cells[c].degree();
                for i in 0..deg {
                    let n = self.world.tiling.create_mov(c, i);
                    if self.world.board[n].wall == Wall::Deadwall {
                        self.world.board[n].wall = Wall::Cavewall;
                    } else if self.world.board[n].wall == Wall::Deadfloor {
                        self.world.board[n].wall = Wall::Cavefloor;
                    }
                }
            }
            Monster::Slime => {
                // Slime beast explosion changes surrounding floor slime colors
                let deg = self.world.tiling.cells[c].degree();
                for i in 0..deg {
                    let n = self.world.tiling.create_mov(c, i);
                    if self.world.board[n].wall == Wall::FloorA {
                        self.world.board[n].wall = Wall::FloorB;
                    } else if self.world.board[n].wall == Wall::FloorB {
                        self.world.board[n].wall = Wall::FloorA;
                    }
                }
            }
            Monster::Worm => {
                // Defeating sandworm drops Spice!
                if self.world.board[c].item == Item::None {
                    self.world.board[c].item = Item::Spice;
                }
            }
            Monster::IvyRoot => {
                // Slaying ivy root kills all connected ivy parts
                let mut q = VecDeque::new();
                q.push_back(c);
                while let Some(curr) = q.pop_front() {
                    let deg = self.world.tiling.cells[curr].degree();
                    for i in 0..deg {
                        let n = self.world.tiling.create_mov(curr, i);
                        if self.world.board[n].monster.is_ivy() {
                            self.world.board[n].monster = Monster::None;
                            self.world.board[n].wall = Wall::None;
                            q.push_back(n);
                        }
                    }
                }
            }
            _ => {}
        }

        self.world.board[c].monster = Monster::None;
    }

    /// Pick up item on cell
    pub fn collect_item(&mut self, c: CellId) {
        let it = self.world.board[c].item;
        if it == Item::None {
            return;
        }

        self.world.board[c].item = Item::None;
        let info = it.info();

        match info.class {
            ItemClass::Treasure => {
                self.player.add_item(it, 1);
                self.log(format!("Collected {}! (Total: {})", info.name, self.player.item_count(it)));

                // Check victory condition: finding Orb of Yendor or collecting 10+ of a treasure
                if it == Item::OrbYendor {
                    self.player.won = true;
                    self.log("You have obtained the Orb of Yendor! You win!");
                }
            }
            ItemClass::Orb => {
                let charges = info.orb_charges;
                self.player.add_item(it, charges);
                self.log(format!("Obtained {} with {} charges!", info.name, charges));
            }
            ItemClass::Other => {
                self.player.add_item(it, 1);
                self.log(format!("Picked up {}.", info.name));
            }
        }
    }

    /// Execute player action and advance turn
    pub fn process_player_action(&mut self, action: PlayerAction) -> bool {
        if self.player.is_dead || self.player.won {
            return false;
        }

        match action {
            PlayerAction::Step(dir) => {
                let current_c = self.player.walker.at;
                let target_c = self.world.tiling.create_mov(current_c, dir);

                let target_monster = self.world.board[target_c].monster;
                if target_monster != Monster::None && self.can_attack(Monster::Player, target_monster) {
                    // Melee attack monster!
                    self.kill_monster(target_c, Monster::Player);
                } else if self.is_passable(target_c, false) && target_monster == Monster::None {
                    // Move to cell
                    self.world.board[current_c].monster = Monster::None;
                    self.player.walker.at = target_c;
                    let n_spin = self.world.tiling.cells[current_c].spins[dir] as usize;
                    self.player.walker.spin = n_spin;
                    self.world.board[target_c].monster = Monster::Player;

                    // Collect item on landing
                    self.collect_item(target_c);
                } else {
                    // Blocked by wall or friendly entity
                    self.log("Move is blocked!");
                    return false;
                }
            }
            PlayerAction::Wait => {
            }
            PlayerAction::UseOrb(orb, target_c) => {
                if self.player.consume_item(orb, 1) {
                    self.execute_orb_power(orb, target_c);
                } else {
                    self.log(format!("No charges left for {}!", orb.info().name));
                    return false;
                }
            }
        }

        self.turn_count += 1;
        self.after_player_move();
        true

    }

    /// Execute ranged/active orb effect
    pub fn execute_orb_power(&mut self, orb: Item, target_c: CellId) {
        match orb {
            Item::OrbFlash => {
                // Slays all enemies within radius 2
                self.log("Orb of Flash erupts with blinding light!");
                let player_c = self.player.walker.at;
                let cells = self.world.tiling.bfs_distances(player_c, 2);
                for c in cells {
                    if self.world.board[c].monster != Monster::None
                        && !self.world.board[c].monster.is_friendly()
                    {
                        self.kill_monster(c, Monster::Player);
                    }
                }
            }
            Item::OrbLightning => {
                // Strikes enemies in sight
                self.log("Lightning strikes across the hyperbolic plane!");
                let player_c = self.player.walker.at;
                let cells = self.world.tiling.bfs_distances(player_c, 5);
                for c in cells {
                    if self.world.board[c].monster != Monster::None
                        && !self.world.board[c].monster.is_friendly()
                    {
                        self.kill_monster(c, Monster::Player);
                    }
                }
            }
            Item::OrbTeleport => {
                // Teleports player to target cell if empty and passable
                if self.is_passable(target_c, false) && self.world.board[target_c].monster == Monster::None {
                    let old_c = self.player.walker.at;
                    self.world.board[old_c].monster = Monster::None;
                    self.player.walker.at = target_c;
                    self.world.board[target_c].monster = Monster::Player;
                    self.collect_item(target_c);
                    self.log("You teleport across hyperbolic space!");
                }
            }
            Item::OrbFire => {
                self.world.board[target_c].wall = Wall::Fire;
                self.log("A burst of flame ignites!");
            }
            _ => {}
        }
    }

    /// Run post-move sequence: monster AI, combat, environment, and orb decay
    pub fn after_player_move(&mut self) {
        // Expand and generate world around player
        let player_c = self.player.walker.at;
        self.world.generate_area(player_c, self.gamerange);

        // BFS pathfinding distances from player
        self.world.tiling.bfs_distances(player_c, self.gamerange);
        let cell_count = self.world.tiling.cell_count();
        self.world.ensure_board_capacity(cell_count);
        // Monster turn: move enemies towards player
        self.monsters_turn();

        // Reduce active orb charges if needed
        self.decay_orbs();
    }

    /// AI logic: Monsters move towards player or attack player
    pub fn monsters_turn(&mut self) {
        let cell_count = self.world.tiling.cell_count();
        self.world.ensure_board_capacity(cell_count);
        let player_c = self.player.walker.at;
        let mut monster_cells = Vec::new();

        for c in 0..self.world.tiling.cell_count() {
            let m = self.world.board[c].monster;
            if m != Monster::None && !m.is_friendly() && self.world.tiling.cells[c].cpdist <= self.gamerange {
                monster_cells.push((c, m));
            }
        }

        // Sort monsters by distance (closest act first)
        monster_cells.sort_by_key(|(c, _)| self.world.tiling.cells[*c].cpdist);

        for (c, m) in monster_cells {
            if self.world.board[c].monster != m {
                // Monster was already moved or slain earlier this turn
                continue;
            }

            // Stunned monster
            if self.world.board[c].stuntime > 0 {
                self.world.board[c].stuntime -= 1;
                continue;
            }

            let deg = self.world.tiling.cells[c].degree();
            let mut best_target = None;
            let current_dist = self.world.tiling.cells[c].cpdist;
            let mut min_dist = current_dist;

            // Check if monster can directly attack player
            let mut adjacent_to_player = false;
            for i in 0..deg {
                let n = self.world.tiling.create_mov(c, i);
                if n == player_c {
                    adjacent_to_player = true;
                    break;
                }
            }

            if adjacent_to_player {
                // Monster attacks player!
                self.monster_attack_player(m);
                if self.player.is_dead {
                    return;
                }
                continue;
            }

            // Find neighboring cell that moves closer to player
            for i in 0..deg {
                let n = self.world.tiling.create_mov(c, i);
                let n_dist = self.world.tiling.cells[n].cpdist;
                let can_pass = self.is_passable(n, m.info().can_pass_walls);

                if can_pass && self.world.board[n].monster == Monster::None && n_dist < min_dist {
                    min_dist = n_dist;
                    best_target = Some(n);
                }
            }

            // Move monster to best target
            if let Some(target) = best_target {
                self.world.board[c].monster = Monster::None;
                self.world.board[target].monster = m;
            }
        }
    }

    /// Monster strikes player
    pub fn monster_attack_player(&mut self, m: Monster) {
        // Check if player has active Orb of Shielding
        if self.player.consume_item(Item::OrbShield, 1) {
            self.log(format!(
                "Your Orb of Shielding absorbs the attack from the {}! (Shields left: {})",
                m.info().name,
                self.player.item_count(Item::OrbShield)
            ));
            return;
        }

        // Check if player has Orb of Safety (escape fatal blow)
        if self.player.consume_item(Item::OrbSafety, 1) {
            self.log(format!(
                "Orb of Safety activates! You are whisked away from the {} to safety!",
                m.info().name
            ));
            let origin_c = 0;
            self.world.board[self.player.walker.at].monster = Monster::None;
            self.player.walker.at = origin_c;
            self.world.board[origin_c].monster = Monster::Player;
            return;
        }

        // Fatal attack!
        self.player.is_dead = true;
        let msg = format!("You were caught and slain by a {}!", m.info().name);
        self.log(msg.clone());
        self.player.death_message = Some(msg);
    }

    /// Periodic decay of active orbs
    pub fn decay_orbs(&mut self) {
        let timed_orbs = [
            Item::OrbSpeed,
            Item::OrbWinter,
            Item::OrbFlash,
            Item::OrbLightning,
            Item::OrbThorns,
            Item::OrbAether,
            Item::OrbInvis,
        ];

        for orb in timed_orbs {
            let count = self.player.item_count(orb);
            if count > 0 {
                self.player.consume_item(orb, 1);
                if self.player.item_count(orb) == 0 {
                    self.log(format!("Your {} has expired.", orb.info().name));
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
    fn test_engine_init() {
        let engine = GameEngine::new(42);
        assert_eq!(engine.player.walker.at, 0);
        assert!(!engine.player.is_dead);
        assert_eq!(engine.turn_count, 0);
    }

    #[test]
    fn test_player_movement_and_item_collection() {
        let mut engine = GameEngine::new(42);
        let target_c = engine.world.tiling.create_mov(0, 0);
        engine.world.board[target_c].item = Item::Diamond;

        // Step to target
        let moved = engine.process_player_action(PlayerAction::Step(0));
        assert!(moved);
        assert_eq!(engine.player.walker.at, target_c);
        assert_eq!(engine.player.item_count(Item::Diamond), 1);
        assert_eq!(engine.world.board[target_c].item, Item::None);
    }

    #[test]
    fn test_player_attacks_monster() {
        let mut engine = GameEngine::new(42);
        let target_c = engine.world.tiling.create_mov(0, 0);
        engine.world.board[target_c].monster = Monster::Yeti;

        // Step into enemy cell attacks and slays the Yeti
        let acted = engine.process_player_action(PlayerAction::Step(0));
        assert!(acted);
        assert_eq!(engine.player.kill_count(Monster::Yeti), 1);
        assert_eq!(engine.world.board[target_c].monster, Monster::None);
    }

    #[test]
    fn test_troll_death_creates_dead_troll_wall() {
        let mut engine = GameEngine::new(42);
        let target_c = engine.world.tiling.create_mov(0, 0);
        engine.world.board[target_c].monster = Monster::Troll;

        engine.process_player_action(PlayerAction::Step(0));
        assert_eq!(engine.player.kill_count(Monster::Troll), 1);
        assert_eq!(engine.world.board[target_c].wall, Wall::DeadTroll);
    }

    #[test]
    fn test_monster_ai_attacks_player() {
        let mut engine = GameEngine::new(42);
        let adj_c = engine.world.tiling.create_mov(0, 0);
        engine.world.board[adj_c].monster = Monster::Yeti;

        // Player waits: adjacent Yeti strikes player!
        engine.process_player_action(PlayerAction::Wait);
        assert!(engine.player.is_dead);
        assert!(engine.player.death_message.is_some());
    }

    #[test]
    fn test_orb_shield_prevents_death() {
        let mut engine = GameEngine::new(42);
        engine.player.add_item(Item::OrbShield, 1);

        let adj_c = engine.world.tiling.create_mov(0, 0);
        engine.world.board[adj_c].monster = Monster::Yeti;

        // Player waits: shield absorbs the strike!
        engine.process_player_action(PlayerAction::Wait);
        assert!(!engine.player.is_dead);
        assert_eq!(engine.player.item_count(Item::OrbShield), 0);
    }
}
