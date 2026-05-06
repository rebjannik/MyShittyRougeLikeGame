use rand::{Rng, SeedableRng};
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

    pub fn set_tile(&mut self, x: usize, y: usize, tile:TileType){
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
    pub fn new(seed: u64) -> Self {
        Self {
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }

    pub fn walk_direction(&mut self) -> (i32, i32) {
        const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        DIRECTIONS[self.rng.random_range(0..DIRECTIONS.len())]
    }
}