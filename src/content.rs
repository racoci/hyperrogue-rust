//! Game Content: Lands, Walls, Monsters, Items, and Orbs
//!
//! Recreates all entity types, flags, color palettes, symbols, and classification
//! helpers directly corresponding to HyperRogue's content tables.

/// All Land environments in HyperRogue
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Land {
    None,
    Barrier,          // Great Wall separating lands
    Crossroads,       // Hub connecting lands
    Crossroads2,
    Crossroads3,
    Crossroads4,
    Crossroads5,
    Desert,           // Sand dunes, worms, spice
    Ice,              // Melting ice walls, yetis, diamonds
    Caves,            // Living caves CA, trolls, gold
    Jungle,           // Growing ivies, monkeys, rubies
    Alchemist,        // Red/blue slime pools, elixir
    Mirror,           // Magic mirrors, rangers, shards
    Graveyard,        // Graves, zombies, ghosts, bones
    Rlyeh,            // Statues, cultists, tentacles
    Hell,             // Demons, sulphur, daisies
    Cocytus,          // Frozen lake of hell, sapphires
    Motion,           // Eternal motion, running dogs, feathers
    DryForest,        // Trees, bushfire, fern flowers
    Emerald,          // Emerald mines, miners
    Wineyard,         // Vines, wine
    DeadCaves,        // Dead caves, silver
    Hive,             // Hyperbugs, royal jelly
    Power,            // Witch covens, powerstones
    Camelot,          // Castle walls, round tables, holy grail
    Temple,           // Concentric horocycles, grimoires
    Caribbean,        // Sea, islands, pirates
    RedRock,          // Red rock valley, red gems
    Minefield,        // Minesweeper in hyperbolic space
    Ocean,            // Deep waters, amber
    Whirlpool,        // Oceanic vortex, pearls
    Palace,           // Palace walls, guards, palace quest
    Livefjord,        // Living fjord, vikings
    IvoryTower,       // Gravity land, ivory
    Zebra,            // Striped zebra land
    Storms,           // Lightning-charged land
    Overgrown,        // Mutant ivies
    Clearing,         // Cleared forest
    Haunted,          // Haunted woods, lotus
    Whirlwind,        // Windy plains, windstones
    Rose,             // Rose garden
    Dragon,           // Dragon chasms
    Trollheim,        // Troll homeland
}

/// All Wall and Terrain types
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Wall {
    None,
    Icewall,          // Melting ice wall in Icy Land
    Barrier,          // Impassable Great Wall
    FloorA,           // Red slime pool
    FloorB,           // Blue slime pool
    Cavewall,         // Living cave wall
    Cavefloor,        // Living cave floor
    DeadTroll,        // Petrified dead troll wall
    Dune,             // Sand dune in Desert
    Mirror,           // Magic mirror
    Fire,             // Burning fire
    AncientGrave,     // Ancient grave in Graveyard
    FreshGrave,       // Fresh grave
    Column,           // Temple column
    Lake,             // Water lake
    FrozenLake,       // Frozen ice sheet
    Chasm,            // Bottomless pit
    BigTree,          // Dry Forest big tree (takes 2 turns to chop)
    SmallTree,        // Dry Forest small tree
    VinePlant,        // Vineyard vine
    VineHalfA,        // Vineyard half-vine
    VineHalfB,
    Deadwall,         // Dead cave wall
    Deadfloor,        // Dead cave floor
    Deadfloor2,       // Rubble floor
    WaxWall,          // Hive weird rock
    Camelot,          // Camelot castle wall
    RoundTable,       // Camelot Round Table
    CamelotMoat,      // Castle moat
    Sea,              // Open sea water
    Boat,             // Player/monster boat
    Red1,             // Red Rock tier 1
    Red2,             // Red Rock tier 2
    Red3,             // Red Rock tier 3
    MineUnknown,      // Minefield covered tile
    MineMine,         // Minefield mine
    MineOpen,         // Minefield revealed safe tile
    Rubble,           // Broken wall rubble
}

/// All Monster types
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Monster {
    None,
    Player,
    Yeti,             // Icy Land brute
    Wolf,             // Icewolf tracking heat
    WolfMoved,
    Ranger,           // Mirror protector
    Troll,            // Living Cave rock troll
    Goblin,           // Living Cave goblin
    Worm,             // Sandworm head
    WormTail,         // Sandworm tail
    WormWait,         // Burrowed sandworm segment
    Hedge,            // Hedgehog warrior
    Desertman,        // Desert nomad
    IvyRoot,          // Jungle ivy root
    IvyHead,          // Growing active ivy branch
    IvyBranch,        // Intermediate ivy branch
    IvyWait,          // Dormant ivy
    IvyDead,          // Dead withered ivy
    Monkey,           // Giant ape
    Slime,            // Alchemist slime beast
    Mimic,            // Magical mirror copy
    Golem,            // Friendly construct
    GolemMoved,
    Eagle,            // Fast flying bird
    Seep,             // Wall-dwelling slime
    Zombie,           // Graveyard undead
    Ghost,            // Wall-phasing ghost
    Necromancer,      // Undead summoner
    Demon,            // Lesser demon of Hell
    Cultist,          // Cthulhu cultist
    Tentacle,         // Kraken / Cthulhu tentacle
    DarkTroll,        // Underworld troll
    FjordTroll,       // Living Fjord troll
    RedTroll,         // Red rock troll
    Viking,           // Coastal raider
    Pirate,           // Caribbean pirate
    Shark,            // Sea predator
    Outlaw,           // Wild West outlaw
}

/// Item classification
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ItemClass {
    Treasure,
    Orb,
    Other,
}

/// All Items and Orbs
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Item {
    None,
    // --- Treasures ---
    Diamond,          // Icy Land treasure
    Gold,             // Living Cave treasure
    Spice,            // Desert treasure
    Ruby,             // Jungle treasure
    Elixir,           // Alchemist Lab treasure
    Shard,            // Hall of Mirrors treasure
    Bone,             // Graveyard treasure
    DemonDaisy,       // Hell treasure
    Statue,           // R'Lyeh treasure
    Feather,          // Land of Eternal Motion treasure
    Sapphire,         // Cocytus treasure
    Hyperstone,       // Crossroads treasure
    FernFlower,       // Dry Forest treasure
    HolyGrail,        // Camelot Holy Grail
    Grimoire,         // Temple of Cthulhu grimoire
    Wine,             // Vineyard treasure
    Silver,           // Dead Cave treasure
    RoyalJelly,       // Hive treasure
    Emerald,          // Emerald Mine treasure
    Power,            // Land of Power powerstone
    RedGem,           // Red Rock treasure
    Coast,            // Ocean amber
    Whirlpool,        // Whirlpool pearl
    Palace,           // Palace treasure
    Lotus,            // Haunted Woods lotus

    // --- Key items ---
    Key,
    DeadOrb,
    OrbYendor,        // The ultimate goal: Orb of Yendor

    // --- Magical Orbs ---
    OrbLightning,     // Strikes all enemies in line of sight
    OrbFlash,         // Stuns / slays surrounding enemies
    OrbWinter,        // Cools down heat, freezes water
    OrbSpeed,         // Player moves 2 actions per monster turn
    OrbLife,          // Summons friendly golem
    OrbShield,        // Protects from one lethal attack
    OrbDigging,       // Destroys walls / digs through earth
    OrbTeleport,      // Ranged teleport to clicked cell
    OrbSafety,        // Teleports to starting crossroads on fatal danger
    OrbThorns,        // Damages surrounding enemies
    OrbFire,          // Sets target cell on fire / melts ice
    OrbPsi,           // Mind attack at distance
    OrbAether,        // Phasing through impassable terrain
    OrbInvis,         // Invisibility from monsters
    OrbDragon,        // Breath of flame
    OrbSpace,         // Teleporting swap
    OrbTime,          // Freezes monster turns
    OrbFriend,        // Summons ally
    OrbWater,         // Creates or controls water
}

// ---------------------------------------------------------------------------
// Entity Descriptors & Properties
// ---------------------------------------------------------------------------

pub struct LandInfo {
    pub name: &'static str,
    pub color: u32,
    pub treasure: Item,
}

impl Land {
    pub fn info(&self) -> LandInfo {
        match self {
            Land::None => LandInfo { name: "Void", color: 0x000000, treasure: Item::None },
            Land::Barrier => LandInfo { name: "Great Wall", color: 0xC06000, treasure: Item::None },
            Land::Crossroads => LandInfo { name: "Crossroads", color: 0xFF0000, treasure: Item::Hyperstone },
            Land::Crossroads2 => LandInfo { name: "Crossroads II", color: 0xFF8000, treasure: Item::Hyperstone },
            Land::Crossroads3 => LandInfo { name: "Crossroads III", color: 0x4040FF, treasure: Item::Hyperstone },
            Land::Crossroads4 => LandInfo { name: "Crossroads IV", color: 0x808080, treasure: Item::Hyperstone },
            Land::Crossroads5 => LandInfo { name: "Crossroads V", color: 0xFFFFFF, treasure: Item::Hyperstone },
            Land::Desert => LandInfo { name: "Desert", color: 0xCDA98F, treasure: Item::Spice },
            Land::Ice => LandInfo { name: "Icy Land", color: 0x8080FF, treasure: Item::Diamond },
            Land::Caves => LandInfo { name: "Living Cave", color: 0x306060, treasure: Item::Gold },
            Land::Jungle => LandInfo { name: "Jungle", color: 0x00C000, treasure: Item::Ruby },
            Land::Alchemist => LandInfo { name: "Alchemist Lab", color: 0x900090, treasure: Item::Elixir },
            Land::Mirror => LandInfo { name: "Hall of Mirrors", color: 0x704070, treasure: Item::Shard },
            Land::Graveyard => LandInfo { name: "Graveyard", color: 0x404070, treasure: Item::Bone },
            Land::Rlyeh => LandInfo { name: "R'Lyeh", color: 0x00FF00, treasure: Item::Statue },
            Land::Hell => LandInfo { name: "Hell", color: 0xC00000, treasure: Item::DemonDaisy },
            Land::Cocytus => LandInfo { name: "Cocytus", color: 0x00FF00, treasure: Item::Sapphire },
            Land::Motion => LandInfo { name: "Land of Eternal Motion", color: 0xFFFF00, treasure: Item::Feather },
            Land::DryForest => LandInfo { name: "Dry Forest", color: 0x008000, treasure: Item::FernFlower },
            Land::Emerald => LandInfo { name: "Emerald Mine", color: 0x60C060, treasure: Item::Emerald },
            Land::Wineyard => LandInfo { name: "Vineyard", color: 0x421C52, treasure: Item::Wine },
            Land::DeadCaves => LandInfo { name: "Dead Cave", color: 0x104040, treasure: Item::Silver },
            Land::Hive => LandInfo { name: "Hive", color: 0x705020, treasure: Item::RoyalJelly },
            Land::Power => LandInfo { name: "Land of Power", color: 0xFFFF00, treasure: Item::Power },
            Land::Camelot => LandInfo { name: "Camelot", color: 0xD0D0D0, treasure: Item::HolyGrail },
            Land::Temple => LandInfo { name: "Temple of Cthulhu", color: 0xD000D0, treasure: Item::Grimoire },
            Land::Caribbean => LandInfo { name: "Caribbean", color: 0x0000FF, treasure: Item::Coast },
            Land::RedRock => LandInfo { name: "Red Rock Valley", color: 0x400000, treasure: Item::RedGem },
            Land::Minefield => LandInfo { name: "Minefield", color: 0xD0D0D0, treasure: Item::None },
            Land::Ocean => LandInfo { name: "Ocean", color: 0x2020FF, treasure: Item::Coast },
            Land::Whirlpool => LandInfo { name: "Whirlpool", color: 0x0000C0, treasure: Item::Whirlpool },
            Land::Palace => LandInfo { name: "Palace", color: 0xFFD500, treasure: Item::Palace },
            Land::Livefjord => LandInfo { name: "Living Fjord", color: 0xC0C000, treasure: Item::None },
            Land::IvoryTower => LandInfo { name: "Ivory Tower", color: 0xFFFFE0, treasure: Item::None },
            Land::Zebra => LandInfo { name: "Zebra", color: 0x606060, treasure: Item::None },
            Land::Storms => LandInfo { name: "Land of Storms", color: 0x80A080, treasure: Item::None },
            Land::Overgrown => LandInfo { name: "Overgrown Woods", color: 0x20A050, treasure: Item::None },
            Land::Clearing => LandInfo { name: "Clearing", color: 0x20D050, treasure: Item::None },
            Land::Haunted => LandInfo { name: "Haunted Woods", color: 0x303030, treasure: Item::Lotus },
            Land::Whirlwind => LandInfo { name: "Windy Plains", color: 0xC0C0FF, treasure: Item::None },
            Land::Rose => LandInfo { name: "Rose Garden", color: 0xEC9CE8, treasure: Item::None },
            Land::Dragon => LandInfo { name: "Dragon Chasms", color: 0x800000, treasure: Item::None },
            Land::Trollheim => LandInfo { name: "Trollheim", color: 0x505050, treasure: Item::None },
        }
    }

    #[inline]
    pub fn is_icy(&self) -> bool {
        matches!(self, Land::Ice | Land::Cocytus)
    }

    #[inline]
    pub fn is_caves(&self) -> bool {
        matches!(self, Land::Caves | Land::DeadCaves | Land::Trollheim)
    }
}

pub struct WallInfo {
    pub glyph: char,
    pub color: u32,
    pub name: &'static str,
    pub blocks_movement: bool,
    pub is_water: bool,
    pub is_chasm: bool,
    pub is_fire: bool,
}

impl Wall {
    pub fn info(&self) -> WallInfo {
        match self {
            Wall::None => WallInfo { glyph: '.', color: 0x808080, name: "floor", blocks_movement: false, is_water: false, is_chasm: false, is_fire: false },
            Wall::Icewall => WallInfo { glyph: '#', color: 0x8080FF, name: "ice wall", blocks_movement: true, is_water: false, is_chasm: false, is_fire: false },
            Wall::Barrier => WallInfo { glyph: '#', color: 0xC06000, name: "great wall", blocks_movement: true, is_water: false, is_chasm: false, is_fire: false },
            Wall::FloorA => WallInfo { glyph: '+', color: 0x900030, name: "red slime", blocks_movement: false, is_water: false, is_chasm: false, is_fire: false },
            Wall::FloorB => WallInfo { glyph: '+', color: 0x300090, name: "blue slime", blocks_movement: false, is_water: false, is_chasm: false, is_fire: false },
            Wall::Cavewall => WallInfo { glyph: '#', color: 0xA0D0A0, name: "living wall", blocks_movement: true, is_water: false, is_chasm: false, is_fire: false },
            Wall::Cavefloor => WallInfo { glyph: '.', color: 0x306060, name: "living floor", blocks_movement: false, is_water: false, is_chasm: false, is_fire: false },
            Wall::DeadTroll => WallInfo { glyph: '#', color: 0xD03030, name: "dead rock troll", blocks_movement: true, is_water: false, is_chasm: false, is_fire: false },
            Wall::Dune => WallInfo { glyph: '#', color: 0xCDA98F, name: "sand dune", blocks_movement: true, is_water: false, is_chasm: false, is_fire: false },
            Wall::Mirror => WallInfo { glyph: '%', color: 0xC0C0FF, name: "magic mirror", blocks_movement: true, is_water: false, is_chasm: false, is_fire: false },
            Wall::Fire => WallInfo { glyph: '^', color: 0xFF4000, name: "fire", blocks_movement: false, is_water: false, is_chasm: false, is_fire: true },
            Wall::AncientGrave => WallInfo { glyph: '+', color: 0xC0C0C0, name: "ancient grave", blocks_movement: true, is_water: false, is_chasm: false, is_fire: false },
            Wall::FreshGrave => WallInfo { glyph: '+', color: 0xD0D080, name: "fresh grave", blocks_movement: true, is_water: false, is_chasm: false, is_fire: false },
            Wall::Column => WallInfo { glyph: '#', color: 0x00FFFF, name: "column", blocks_movement: true, is_water: false, is_chasm: false, is_fire: false },
            Wall::Lake => WallInfo { glyph: '=', color: 0x000080, name: "lake", blocks_movement: true, is_water: true, is_chasm: false, is_fire: false },
            Wall::FrozenLake => WallInfo { glyph: '_', color: 0x000080, name: "frozen lake", blocks_movement: false, is_water: false, is_chasm: false, is_fire: false },
            Wall::Chasm => WallInfo { glyph: '>', color: 0x101010, name: "chasm", blocks_movement: true, is_water: false, is_chasm: true, is_fire: false },
            Wall::BigTree => WallInfo { glyph: '#', color: 0x60C000, name: "big tree", blocks_movement: true, is_water: false, is_chasm: false, is_fire: false },
            Wall::SmallTree => WallInfo { glyph: '#', color: 0x006000, name: "tree", blocks_movement: true, is_water: false, is_chasm: false, is_fire: false },
            Wall::VinePlant => WallInfo { glyph: '#', color: 0x8438A4, name: "vine", blocks_movement: true, is_water: false, is_chasm: false, is_fire: false },
            Wall::VineHalfA => WallInfo { glyph: ':', color: 0x006000, name: "vine", blocks_movement: false, is_water: false, is_chasm: false, is_fire: false },
            Wall::VineHalfB => WallInfo { glyph: ';', color: 0x006000, name: "vine", blocks_movement: false, is_water: false, is_chasm: false, is_fire: false },
            Wall::Deadwall => WallInfo { glyph: '#', color: 0xA07070, name: "dead wall", blocks_movement: true, is_water: false, is_chasm: false, is_fire: false },
            Wall::Deadfloor => WallInfo { glyph: '.', color: 0x401010, name: "dead floor", blocks_movement: false, is_water: false, is_chasm: false, is_fire: false },
            Wall::Deadfloor2 => WallInfo { glyph: '.', color: 0x905050, name: "rubble", blocks_movement: false, is_water: false, is_chasm: false, is_fire: false },
            Wall::WaxWall => WallInfo { glyph: '#', color: 0xD0D010, name: "weird rock", blocks_movement: true, is_water: false, is_chasm: false, is_fire: false },
            Wall::Camelot => WallInfo { glyph: '#', color: 0xC0C0C0, name: "wall of Camelot", blocks_movement: true, is_water: false, is_chasm: false, is_fire: false },
            Wall::RoundTable => WallInfo { glyph: '+', color: 0xA06000, name: "Round Table", blocks_movement: false, is_water: false, is_chasm: false, is_fire: false },
            Wall::CamelotMoat => WallInfo { glyph: '=', color: 0x0000A0, name: "moat of Camelot", blocks_movement: true, is_water: true, is_chasm: false, is_fire: false },
            Wall::Sea => WallInfo { glyph: '=', color: 0x0000A0, name: "sea", blocks_movement: true, is_water: true, is_chasm: false, is_fire: false },
            Wall::Boat => WallInfo { glyph: '+', color: 0x808000, name: "boat", blocks_movement: false, is_water: false, is_chasm: false, is_fire: false },
            Wall::Red1 => WallInfo { glyph: ',', color: 0x800000, name: "rock I", blocks_movement: false, is_water: false, is_chasm: false, is_fire: false },
            Wall::Red2 => WallInfo { glyph: ':', color: 0xC00000, name: "rock II", blocks_movement: false, is_water: false, is_chasm: false, is_fire: false },
            Wall::Red3 => WallInfo { glyph: ';', color: 0xFF0000, name: "rock III", blocks_movement: false, is_water: false, is_chasm: false, is_fire: false },
            Wall::MineUnknown => WallInfo { glyph: '.', color: 0xD0D0D0, name: "minefield", blocks_movement: false, is_water: false, is_chasm: false, is_fire: false },
            Wall::MineMine => WallInfo { glyph: '*', color: 0xD0D0D0, name: "mine", blocks_movement: false, is_water: false, is_chasm: false, is_fire: false },
            Wall::MineOpen => WallInfo { glyph: '.', color: 0x909090, name: "open cell", blocks_movement: false, is_water: false, is_chasm: false, is_fire: false },
            Wall::Rubble => WallInfo { glyph: '.', color: 0x707070, name: "rubble", blocks_movement: false, is_water: false, is_chasm: false, is_fire: false },
        }
    }

    #[inline]
    pub fn blocks(&self) -> bool {
        self.info().blocks_movement
    }
}

pub struct MonsterInfo {
    pub glyph: char,
    pub color: u32,
    pub name: &'static str,
    pub can_pass_walls: bool,
    pub is_friendly: bool,
    pub tracks_heat: bool,
}

impl Monster {
    pub fn info(&self) -> MonsterInfo {
        match self {
            Monster::None => MonsterInfo { glyph: ' ', color: 0, name: "none", can_pass_walls: false, is_friendly: false, tracks_heat: false },
            Monster::Player => MonsterInfo { glyph: '@', color: 0xFFFFFF, name: "Rogue", can_pass_walls: false, is_friendly: true, tracks_heat: false },
            Monster::Yeti => MonsterInfo { glyph: 'Y', color: 0x4040FF, name: "Yeti", can_pass_walls: false, is_friendly: false, tracks_heat: false },
            Monster::Wolf | Monster::WolfMoved => MonsterInfo { glyph: 'W', color: 0xD08040, name: "Icewolf", can_pass_walls: false, is_friendly: false, tracks_heat: true },
            Monster::Ranger => MonsterInfo { glyph: 'R', color: 0xFF8000, name: "Ranger", can_pass_walls: false, is_friendly: false, tracks_heat: false },
            Monster::Troll => MonsterInfo { glyph: 'T', color: 0xD0D0D0, name: "Rock Troll", can_pass_walls: false, is_friendly: false, tracks_heat: false },
            Monster::Goblin => MonsterInfo { glyph: 'G', color: 0x20D020, name: "Goblin", can_pass_walls: false, is_friendly: false, tracks_heat: false },
            Monster::Worm => MonsterInfo { glyph: 'S', color: 0xE0E040, name: "Sand Worm", can_pass_walls: false, is_friendly: false, tracks_heat: false },
            Monster::WormTail => MonsterInfo { glyph: 's', color: 0x808000, name: "Sand Worm Tail", can_pass_walls: false, is_friendly: false, tracks_heat: false },
            Monster::WormWait => MonsterInfo { glyph: 'S', color: 0x808000, name: "Sand Worm Segment", can_pass_walls: false, is_friendly: false, tracks_heat: false },
            Monster::Hedge => MonsterInfo { glyph: 'H', color: 0x80FF00, name: "Hedgehog Warrior", can_pass_walls: false, is_friendly: false, tracks_heat: false },
            Monster::Desertman => MonsterInfo { glyph: 'M', color: 0x806050, name: "Desert Man", can_pass_walls: false, is_friendly: false, tracks_heat: false },
            Monster::IvyRoot => MonsterInfo { glyph: 'C', color: 0x00FFFF, name: "Ivy Root", can_pass_walls: false, is_friendly: false, tracks_heat: false },
            Monster::IvyHead => MonsterInfo { glyph: 'C', color: 0xFFFF00, name: "Active Ivy", can_pass_walls: false, is_friendly: false, tracks_heat: false },
            Monster::IvyBranch => MonsterInfo { glyph: 'C', color: 0x40FF00, name: "Ivy Branch", can_pass_walls: false, is_friendly: false, tracks_heat: false },
            Monster::IvyWait => MonsterInfo { glyph: 'C', color: 0x006030, name: "Dormant Ivy", can_pass_walls: false, is_friendly: false, tracks_heat: false },
            Monster::IvyDead => MonsterInfo { glyph: 'C', color: 0x800000, name: "Dead Ivy", can_pass_walls: false, is_friendly: false, tracks_heat: false },
            Monster::Monkey => MonsterInfo { glyph: 'M', color: 0x804000, name: "Giant Ape", can_pass_walls: false, is_friendly: false, tracks_heat: false },
            Monster::Slime => MonsterInfo { glyph: 'B', color: 0x909000, name: "Slime Beast", can_pass_walls: false, is_friendly: false, tracks_heat: false },
            Monster::Mimic => MonsterInfo { glyph: '@', color: 0xFF80FF, name: "Mimic", can_pass_walls: false, is_friendly: true, tracks_heat: false },
            Monster::Golem | Monster::GolemMoved => MonsterInfo { glyph: '@', color: 0x509050, name: "Golem", can_pass_walls: false, is_friendly: true, tracks_heat: false },
            Monster::Eagle => MonsterInfo { glyph: 'E', color: 0xD09050, name: "Eagle", can_pass_walls: false, is_friendly: false, tracks_heat: false },
            Monster::Seep => MonsterInfo { glyph: 'S', color: 0xFF8080, name: "Seep", can_pass_walls: true, is_friendly: false, tracks_heat: false },
            Monster::Zombie => MonsterInfo { glyph: 'Z', color: 0x804000, name: "Zombie", can_pass_walls: false, is_friendly: false, tracks_heat: false },
            Monster::Ghost => MonsterInfo { glyph: 'G', color: 0xFFFFFF, name: "Ghost", can_pass_walls: true, is_friendly: false, tracks_heat: false },
            Monster::Necromancer => MonsterInfo { glyph: 'N', color: 0xA000A0, name: "Necromancer", can_pass_walls: false, is_friendly: false, tracks_heat: false },
            Monster::Demon => MonsterInfo { glyph: 'D', color: 0xFF0000, name: "Demon", can_pass_walls: false, is_friendly: false, tracks_heat: false },
            Monster::Cultist => MonsterInfo { glyph: 'c', color: 0x00A000, name: "Cultist", can_pass_walls: false, is_friendly: false, tracks_heat: false },
            Monster::Tentacle => MonsterInfo { glyph: 'T', color: 0x00FF00, name: "Tentacle", can_pass_walls: false, is_friendly: false, tracks_heat: false },
            Monster::DarkTroll => MonsterInfo { glyph: 'T', color: 0x404040, name: "Dark Troll", can_pass_walls: false, is_friendly: false, tracks_heat: false },
            Monster::FjordTroll => MonsterInfo { glyph: 'T', color: 0x00A0A0, name: "Fjord Troll", can_pass_walls: false, is_friendly: false, tracks_heat: false },
            Monster::RedTroll => MonsterInfo { glyph: 'T', color: 0xA02020, name: "Red Troll", can_pass_walls: false, is_friendly: false, tracks_heat: false },
            Monster::Viking => MonsterInfo { glyph: 'V', color: 0x4080FF, name: "Viking", can_pass_walls: false, is_friendly: false, tracks_heat: false },
            Monster::Pirate => MonsterInfo { glyph: 'P', color: 0xFF4040, name: "Pirate", can_pass_walls: false, is_friendly: false, tracks_heat: false },
            Monster::Shark => MonsterInfo { glyph: 'S', color: 0x2020A0, name: "Shark", can_pass_walls: false, is_friendly: false, tracks_heat: false },
            Monster::Outlaw => MonsterInfo { glyph: 'O', color: 0xD0A040, name: "Outlaw", can_pass_walls: false, is_friendly: false, tracks_heat: false },
        }
    }

    #[inline]
    pub fn is_ivy(&self) -> bool {
        matches!(
            self,
            Monster::IvyRoot
                | Monster::IvyHead
                | Monster::IvyBranch
                | Monster::IvyWait
                | Monster::IvyDead
        )
    }

    #[inline]
    pub fn is_worm(&self) -> bool {
        matches!(self, Monster::Worm | Monster::WormTail | Monster::WormWait)
    }

    #[inline]
    pub fn is_friendly(&self) -> bool {
        self.info().is_friendly
    }
}

pub struct ItemInfo {
    pub glyph: char,
    pub color: u32,
    pub name: &'static str,
    pub class: ItemClass,
    pub orb_charges: i32,
}

impl Item {
    pub fn info(&self) -> ItemInfo {
        match self {
            Item::None => ItemInfo { glyph: ' ', color: 0, name: "none", class: ItemClass::Other, orb_charges: 0 },
            Item::Diamond => ItemInfo { glyph: '*', color: 0xFFFFFF, name: "Ice Diamond", class: ItemClass::Treasure, orb_charges: 0 },
            Item::Gold => ItemInfo { glyph: '$', color: 0xFFD700, name: "Gold", class: ItemClass::Treasure, orb_charges: 0 },
            Item::Spice => ItemInfo { glyph: ';', color: 0xFF4000, name: "Spice", class: ItemClass::Treasure, orb_charges: 0 },
            Item::Ruby => ItemInfo { glyph: '*', color: 0xC00000, name: "Ruby", class: ItemClass::Treasure, orb_charges: 0 },
            Item::Elixir => ItemInfo { glyph: '!', color: 0xFFFF00, name: "Elixir of Life", class: ItemClass::Treasure, orb_charges: 0 },
            Item::Shard => ItemInfo { glyph: '%', color: 0xFF00FF, name: "Shard", class: ItemClass::Treasure, orb_charges: 0 },
            Item::Bone => ItemInfo { glyph: '/', color: 0xFF8000, name: "Necromancer's Totem", class: ItemClass::Treasure, orb_charges: 0 },
            Item::DemonDaisy => ItemInfo { glyph: '%', color: 0x00D000, name: "Demon Daisy", class: ItemClass::Treasure, orb_charges: 0 },
            Item::Statue => ItemInfo { glyph: '/', color: 0x00FF00, name: "Statue of Cthulhu", class: ItemClass::Treasure, orb_charges: 0 },
            Item::Feather => ItemInfo { glyph: '*', color: 0xFF8000, name: "Phoenix Feather", class: ItemClass::Treasure, orb_charges: 0 },
            Item::Sapphire => ItemInfo { glyph: '*', color: 0x8080FF, name: "Ice Sapphire", class: ItemClass::Treasure, orb_charges: 0 },
            Item::Hyperstone => ItemInfo { glyph: '*', color: 0xEEFF20, name: "Hyperstone", class: ItemClass::Treasure, orb_charges: 0 },
            Item::HolyGrail => ItemInfo { glyph: '!', color: 0xFFFF00, name: "Holy Grail", class: ItemClass::Other, orb_charges: 0 },
            Item::Grimoire => ItemInfo { glyph: '?', color: 0x00FF00, name: "Grimoire", class: ItemClass::Treasure, orb_charges: 0 },
            Item::Wine => ItemInfo { glyph: '!', color: 0x900000, name: "Wine", class: ItemClass::Treasure, orb_charges: 0 },
            Item::Silver => ItemInfo { glyph: '$', color: 0xFFFFC0, name: "Silver", class: ItemClass::Treasure, orb_charges: 0 },
            Item::RoyalJelly => ItemInfo { glyph: '!', color: 0xE2B227, name: "Royal Jelly", class: ItemClass::Treasure, orb_charges: 0 },
            Item::Emerald => ItemInfo { glyph: '*', color: 0x60C060, name: "Emerald", class: ItemClass::Treasure, orb_charges: 0 },
            Item::Power => ItemInfo { glyph: '*', color: 0xFFFF00, name: "Powerstone", class: ItemClass::Treasure, orb_charges: 0 },
            Item::RedGem => ItemInfo { glyph: '*', color: 0xFF8080, name: "Red Gem", class: ItemClass::Treasure, orb_charges: 0 },
            Item::Coast => ItemInfo { glyph: '*', color: 0xFFBF00, name: "Amber", class: ItemClass::Treasure, orb_charges: 0 },
            Item::Whirlpool => ItemInfo { glyph: '$', color: 0xFFFFFF, name: "Pearl", class: ItemClass::Treasure, orb_charges: 0 },
            Item::Palace => ItemInfo { glyph: '$', color: 0xFFD700, name: "Palace Gold", class: ItemClass::Treasure, orb_charges: 0 },
            Item::Lotus => ItemInfo { glyph: '%', color: 0x303030, name: "Black Lotus", class: ItemClass::Treasure, orb_charges: 0 },
            Item::FernFlower => ItemInfo { glyph: '%', color: 0x0000FF, name: "Fern Flower", class: ItemClass::Treasure, orb_charges: 0 },

            Item::Key => ItemInfo { glyph: '[', color: 0x8080FF, name: "Key", class: ItemClass::Other, orb_charges: 0 },
            Item::DeadOrb => ItemInfo { glyph: 'o', color: 0x306030, name: "Dead Orb", class: ItemClass::Other, orb_charges: 0 },
            Item::OrbYendor => ItemInfo { glyph: 'o', color: 0xFF20FF, name: "Orb of Yendor", class: ItemClass::Other, orb_charges: 0 },

            // Orbs
            Item::OrbLightning => ItemInfo { glyph: 'o', color: 0xFFFF00, name: "Orb of Storms", class: ItemClass::Orb, orb_charges: 30 },
            Item::OrbFlash => ItemInfo { glyph: 'o', color: 0xFFFFFF, name: "Orb of Flash", class: ItemClass::Orb, orb_charges: 30 },
            Item::OrbWinter => ItemInfo { glyph: 'o', color: 0x8080FF, name: "Orb of Winter", class: ItemClass::Orb, orb_charges: 25 },
            Item::OrbSpeed => ItemInfo { glyph: 'o', color: 0xFF6060, name: "Orb of Speed", class: ItemClass::Orb, orb_charges: 30 },
            Item::OrbLife => ItemInfo { glyph: 'o', color: 0x90B090, name: "Orb of Life", class: ItemClass::Orb, orb_charges: 0 },
            Item::OrbShield => ItemInfo { glyph: 'o', color: 0x60D760, name: "Orb of Shielding", class: ItemClass::Orb, orb_charges: 3 },
            Item::OrbDigging => ItemInfo { glyph: 'o', color: 0x606060, name: "Orb of Earth", class: ItemClass::Orb, orb_charges: 20 },
            Item::OrbTeleport => ItemInfo { glyph: 'o', color: 0x20FFFF, name: "Orb of Teleport", class: ItemClass::Orb, orb_charges: 5 },
            Item::OrbSafety => ItemInfo { glyph: 'o', color: 0xA0FF40, name: "Orb of Safety", class: ItemClass::Orb, orb_charges: 1 },
            Item::OrbThorns => ItemInfo { glyph: 'o', color: 0x40C000, name: "Orb of Thorns", class: ItemClass::Orb, orb_charges: 25 },
            Item::OrbFire => ItemInfo { glyph: 'o', color: 0xFF4000, name: "Orb of Fire", class: ItemClass::Orb, orb_charges: 25 },
            Item::OrbPsi => ItemInfo { glyph: 'o', color: 0x005050, name: "Orb of the Mind", class: ItemClass::Orb, orb_charges: 15 },
            Item::OrbAether => ItemInfo { glyph: 'o', color: 0x706070, name: "Orb of Aether", class: ItemClass::Orb, orb_charges: 20 },
            Item::OrbInvis => ItemInfo { glyph: 'o', color: 0x421C52, name: "Orb of Invisibility", class: ItemClass::Orb, orb_charges: 25 },
            Item::OrbDragon => ItemInfo { glyph: 'o', color: 0xFF8000, name: "Orb of the Dragon", class: ItemClass::Orb, orb_charges: 10 },
            Item::OrbSpace => ItemInfo { glyph: 'o', color: 0x40C0C0, name: "Orb of Space", class: ItemClass::Orb, orb_charges: 15 },
            Item::OrbTime => ItemInfo { glyph: 'o', color: 0x6060FF, name: "Orb of Time", class: ItemClass::Orb, orb_charges: 15 },
            Item::OrbFriend => ItemInfo { glyph: 'o', color: 0x306000, name: "Orb of Friendship", class: ItemClass::Orb, orb_charges: 0 },
            Item::OrbWater => ItemInfo { glyph: 'o', color: 0x000060, name: "Orb of Water", class: ItemClass::Orb, orb_charges: 20 },
        }
    }

    #[inline]
    pub fn is_treasure(&self) -> bool {
        self.info().class == ItemClass::Treasure
    }

    #[inline]
    pub fn is_orb(&self) -> bool {
        self.info().class == ItemClass::Orb
    }
}

// ---------------------------------------------------------------------------
// Tests and Validations
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_land_treasures() {
        assert_eq!(Land::Ice.info().treasure, Item::Diamond);
        assert_eq!(Land::Caves.info().treasure, Item::Gold);
        assert_eq!(Land::Desert.info().treasure, Item::Spice);
        assert_eq!(Land::Jungle.info().treasure, Item::Ruby);
        assert_eq!(Land::Alchemist.info().treasure, Item::Elixir);
        assert_eq!(Land::Mirror.info().treasure, Item::Shard);
        assert_eq!(Land::Graveyard.info().treasure, Item::Bone);
        assert_eq!(Land::Crossroads.info().treasure, Item::Hyperstone);
    }

    #[test]
    fn test_wall_blocks() {
        assert!(Wall::Icewall.blocks());
        assert!(Wall::Barrier.blocks());
        assert!(Wall::Cavewall.blocks());
        assert!(Wall::Dune.blocks());
        assert!(!Wall::None.blocks());
        assert!(!Wall::Cavefloor.blocks());
        assert!(!Wall::FloorA.blocks());
    }

    #[test]
    fn test_monster_properties() {
        assert!(Monster::Ghost.info().can_pass_walls);
        assert!(Monster::Seep.info().can_pass_walls);
        assert!(!Monster::Yeti.info().can_pass_walls);
        assert!(Monster::Golem.info().is_friendly);
        assert!(Monster::Player.info().is_friendly);
        assert!(!Monster::Troll.info().is_friendly);
    }

    #[test]
    fn test_item_classes() {
        assert!(Item::Diamond.is_treasure());
        assert!(Item::Gold.is_treasure());
        assert!(Item::OrbSpeed.is_orb());
        assert!(Item::OrbLightning.is_orb());
        assert_eq!(Item::OrbSpeed.info().orb_charges, 30);
    }
}
