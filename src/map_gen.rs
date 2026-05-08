use crate::models::{Map, MapGenError, TileType, MapRng};

pub fn print_map(map: &Map) -> String {
    let mut out = String::new();

    for row in map.tiles() {
        for tile in row {
            let symbol = match tile {
                TileType::Wall => '#',
                TileType::Floor => '.',
                TileType::Door => '|',
            };

            out.push(symbol);
        }
        
        out.push('\n');
    }

    println!("{out}");

    out
}

fn generate_walkable_tiles(map: &mut Map, rng: &mut MapRng) {
    // Simple random walk to create walkable tiles
    // TODO: implement a real random walk later.
}


pub fn generate_map(width: usize, height: usize) -> Result<Map, MapGenError> {
    if width < 3 || height < 3 {
        return Err(MapGenError::TooSmall);
    }

    let mut map = Map::new(width, height, TileType::Wall);

    let _map_area = width * height;
    let map_midpoint_width = width / 2;

    // Place the entry at the middle of the bottom row.
    map.set_tile(map_midpoint_width, height - 1, TileType::Door);
    
    //generate_walkable_tiles(&mut map, &mut rng);
    
    Ok(map)
}

#[allow(dead_code)]
pub fn generate_map_with_seed(width: usize, height: usize, _seed: u64) -> Result<Map, MapGenError> {
    if width < 3 || height < 3 {
        return Err(MapGenError::TooSmall);
    }

    let mut map = Map::new(width, height, TileType::Wall);

    let _map_area = width * height;
    let map_midpoint_width = width / 2;

    map.set_tile(map_midpoint_width, height - 1, TileType::Door);
    
    //generate_walkable_tiles(&mut map, &mut rng);
    
    Ok(map)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn print_map_returns_string_representation_of_map() {
        let map = generate_map(3, 3).expect("map generation should succeed");

        let expected = "###\n###\n#|#\n";
        let output = print_map(&map);

        assert_eq!(output, expected);
    }

    #[test]
    fn generate_map_returns_requested_dimensions() {
        let map = generate_map(10, 8).expect("map generation should succeed");

        assert_eq!(map.width(), 10);
        assert_eq!(map.height(), 8);
    }

    #[test]
    fn generated_map_has_only_walls_on_outer_border() {
        let map = generate_map(10, 8).expect("map generation should succeed");
        let entry_point = (map.width() / 2, 0);
        for x in 0..map.width() {
            if x == entry_point.0 && entry_point.1 == 0 {
                continue; // Skip the entry point
            }

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
