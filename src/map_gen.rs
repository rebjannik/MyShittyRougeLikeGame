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
    tiles: Vec<Vec<TileType>>,
}

impl Map {
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

mod map_gen {
    use super::*;

    pub fn generate_map(width: usize, height: usize) -> Result<Map, MapGenError> {
        if width < 3 || height < 3 {
            return Err(MapGenError::TooSmall);
        }

        todo!("implement map generation")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_map_returns_requested_dimensions() {
        let map = generate_map(10, 8).expect("map generation should succeed");

        assert_eq!(map.width(), 10);
        assert_eq!(map.height(), 8);
    }

    #[test]
    fn generated_map_has_only_walls_on_outer_border() {
        let map = generate_map(10, 8).expect("map generation should succeed");

        for x in 0..map.width() {
            assert_eq!(map.tile_at(x, 0), TileType::Wall);
            assert_eq!(map.tile_at(x, map.height() - 1), TileType::Wall);
        }

        for y in 0..map.height() {
            assert_eq!(map.tile_at(0, y), TileType::Wall);
            assert_eq!(map.tile_at(map.width() - 1, y), TileType::Wall);
        }
    }

    #[test]
    fn generated_map_contains_at_least_one_walkable_tile() {
        let map = generate_map(10, 8).expect("map generation should succeed");

        let walkable_tiles = map
            .tiles()
            .iter()
            .flatten()
            .filter(|tile| matches!(tile, TileType::Floor | TileType::Door))
            .count();

        assert!(walkable_tiles > 0);
    }

    #[test]
    fn generating_too_small_map_is_rejected() {
        let result = generate_map(2, 2);

        assert_eq!(result, Err(MapGenError::TooSmall));
    }
}
