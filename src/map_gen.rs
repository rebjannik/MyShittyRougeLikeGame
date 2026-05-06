use crate::models::{Map, MapGenError, TileType};
use crate::map_gen::models::{Map, TileType, MapGenError};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub fn generate_map(width: usize, height: usize) -> Result<Map, MapGenError> {
    todo!("Implement map generation logic here.");

    if width < 3 || height < 3 {
        return Err(MapGenError::TooSmall);
    }

    let map = Map::new(width, height, TileType::Wall);
    
    return Ok(map);
}

pub fn generate_map_with_seed(){
    todo!("Implement map generation with seed for reproducibility.");
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Map, MapGenError, TileType};

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

    #[test]
    fn generate_same_map_with_same_seed(){
        // For reproducibility, we should be able to generate the same map given the same seed.
        let seed = 12345;
        let map1 = generate_map_with_seed(10, 8, seed);
        let map2 = generate_map_with_seed(10, 8, seed);
        assert_eq!(map1, map2);
    }
}
