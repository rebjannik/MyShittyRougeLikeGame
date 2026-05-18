use rand::{Rng, SeedableRng, random};
use rand_chacha::ChaCha8Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Wall,
    Floor,
    Door,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapGenError {
    TooSmall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    MainMenu,
    InGame,
    CharacterCreation,
    Settings,
    Saves,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuOption {
    StartGame,
    Exit,
}
pub enum AppAction {
    Continue,
    ChangeState(GameState),
    Quit,
}

pub enum MonsterType {
    Goblin,
    Orc,
    Troll,
}

pub enum CharacterType {
    Rogue,
    Warrior,
    Wizard,
}

#[allow(dead_code)]
pub struct Monster {
    hp: i32,
    monster_type: MonsterType,
    attack: i32,
    speed: i32,
    stamina: i32,
    stealth: i32,
    strength: i32,
    location: (usize, usize),
}

#[allow(dead_code)]
pub struct Player {
    name: String,
    hp: i32,
    speed: i32,
    stamina: i32,
    stealth: i32,
    strength: i32,
    level: i32,
    xp: i32,
    location: (usize, usize),
}

impl Player {
    pub fn new(name: String, character_type: CharacterType, location: (usize, usize)) -> Self {
        let (speed, stamina, stealth, strength) = match character_type {
            CharacterType::Rogue => (0, 0, 0, 0),
            CharacterType::Warrior => (0, 0, 0, 0),
            CharacterType::Wizard => (0, 0, 0, 0),
        };
        Self {
            name: name,
            hp: 100,
            speed,
            stamina,
            stealth,
            strength,
            level: 1,
            xp: 0,
            location: location,
        }
    }

    pub fn move_to(&mut self, x: usize, y: usize) {
        self.location = (x, y);
    }
}
pub struct MainMenuState {
    pub selected: MenuOption,
}

impl Default for MainMenuState {
    fn default() -> Self {
        Self {
            selected: MenuOption::StartGame,
        }
    }
}

impl MainMenuState {
    pub fn new() -> Self {
        Self {
            selected: MenuOption::StartGame,
        }
    }

    pub fn toggle(&mut self) {
        self.selected = match self.selected {
            MenuOption::StartGame => MenuOption::Exit,
            MenuOption::Exit => MenuOption::StartGame,
        };
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    tiles: Vec<Vec<TileType>>,
}

impl Map {
    pub fn new(width: usize, height: usize, default: TileType) -> Self {
        Self {
            tiles: vec![vec![default; width]; height],
        }
    }

    pub fn width(&self) -> usize {
        self.tiles.first().map_or(0, Vec::len)
    }

    pub fn height(&self) -> usize {
        self.tiles.len()
    }

    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        y < self.height() && x < self.width()
    }

    pub fn tile_at(&self, x: usize, y: usize) -> TileType {
        self.tiles[y][x]
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: TileType) {
        self.tiles[y][x] = tile;
    }
    pub fn tiles(&self) -> &[Vec<TileType>] {
        &self.tiles
    }
}

pub struct MapRng {
    rng: ChaCha8Rng,
}

impl MapRng {
    /// Create a MapRng with a fixed seed for reproducible gameplay
    pub fn new_fixed(seed: u64) -> Self {
        Self {
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }

    /// Create a MapRng with a random seed for testing
    pub fn new_random() -> Self {
        let seed = random::<u64>();
        Self {
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }

    pub fn walk_direction(&mut self) -> (i32, i32) {
        const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        DIRECTIONS[self.rng.random_range(0..DIRECTIONS.len())]
    }
}
