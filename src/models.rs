use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Wall,
    Floor,
    Door,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MapGenError {
    TooSmall,
}

pub struct Map {
    pub tiles: Vec<Vec<TileType>>,
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

    pub fn tile_at(&self, x: usize, y: usize) -> TileType {
        self.tiles[y][x]
    }

    pub fn tiles(&self) -> &[Vec<TileType>] {
        &self.tiles
    }
}

pub struct MapRNG {
    rng: ChaCha8Rng,
}

impl MapRNG{
    pub fn new(seed: u64) -> Self {
        Self {
            rng: ChaCha8Rng::seed_from_u65(seed),
        }
    }

    pub fn walk_direction(&mut self) -> (i32, i32) {
        let directions = [(0,1), (0,-1), (1,0), (-1,0)];
        directions[self.rng.gen_range(0..4)]
    }
}