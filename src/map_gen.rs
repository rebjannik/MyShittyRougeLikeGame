use crate::models::{Map, MapGenError, MapRng, TileType};

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
    let map_area = map.width() * map.height();
    let walkable_tiles_to_generate = map_area / 3;
    let mut walkable_count = 1; // Start with the door

    // Start from the entry point
    let mut x = map.width() / 2;
    let mut y = map.height() - 1;

    let mut steps_without_progress = 0;
    let max_steps_without_progress = 100;

    while walkable_count < walkable_tiles_to_generate
        && steps_without_progress < max_steps_without_progress
    {
        // Get a random direction offset
        let (dx, dy) = rng.walk_direction();

        // Apply the offset, keeping within bounds (leave 1 tile margin for walls)
        let new_x = ((x as i32 + dx).max(1) as usize).min(map.width() - 2);
        let new_y = ((y as i32 + dy).max(1) as usize).min(map.height() - 2);

        x = new_x;
        y = new_y;

        // Only carve if it's a wall and adjacent to an already-carved tile
        if map.tile_at(x, y) == TileType::Wall && has_walkable_neighbor(map, x, y) {
            map.set_tile(x, y, TileType::Floor);
            walkable_count += 1;
            steps_without_progress = 0;
        } else {
            steps_without_progress += 1;
        }
    }
}

fn has_walkable_neighbor(map: &Map, x: usize, y: usize) -> bool {
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    for (dx, dy) in directions {
        let nx = (x as i32 + dx) as usize;
        let ny = (y as i32 + dy) as usize;
        if map.in_bounds(nx, ny) {
            let tile = map.tile_at(nx, ny);
            if tile == TileType::Floor || tile == TileType::Door {
                return true;
            }
        }
    }
    false
}

pub fn generate_map(width: usize, height: usize) -> Result<Map, MapGenError> {
    if width < 3 || height < 3 {
        return Err(MapGenError::TooSmall);
    }

    let mut map = Map::new(width, height, TileType::Wall);
    let mut rng = MapRng::new_random();

    let _map_area = width * height;
    let map_midpoint_width = width / 2;

    // Place the entry at the middle of the bottom row.
    map.set_tile(map_midpoint_width, height - 1, TileType::Door);

    generate_walkable_tiles(&mut map, &mut rng);

    Ok(map)
}

pub fn generate_map_with_seed(width: usize, height: usize, seed: u64) -> Result<Map, MapGenError> {
    if width < 3 || height < 3 {
        return Err(MapGenError::TooSmall);
    }

    let mut map = Map::new(width, height, TileType::Wall);
    let mut rng = MapRng::new_fixed(seed);

    let _map_area = width * height;
    let map_midpoint_width = width / 2;

    map.set_tile(map_midpoint_width, height - 1, TileType::Door);

    generate_walkable_tiles(&mut map, &mut rng);

    Ok(map)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn print_map_returns_string_representation_of_map() {
        let map = generate_map(3, 3).expect("map generation should succeed");

        // A 3x3 map should include the bottom-center door and at least one floor tile
        let output = print_map(&map);

        assert!(output.contains('|'), "map should contain door");
        assert!(
            output.contains('.'),
            "map should contain at least one floor tile"
        );
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
        let entry_point = (map.width() / 2, map.height() - 1);
        for x in 0..map.width() {
            assert_eq!(map.tile_at(x, 0), TileType::Wall);
            if x == entry_point.0 {
                assert_eq!(map.tile_at(x, map.height() - 1), TileType::Door);
            } else {
                assert_eq!(map.tile_at(x, map.height() - 1), TileType::Wall);
            }
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
    fn generate_map_with_seed_returns_same_map_for_same_seed() {
        let seed = 12345;
        let map1 = generate_map_with_seed(10, 8, seed).expect("map generation should succeed");
        let map2 = generate_map_with_seed(10, 8, seed).expect("map generation should succeed");

        assert_eq!(map1.tiles(), map2.tiles());
    }

    #[test]
    fn no_islands_created() {
        let map = generate_map(10, 10).expect("map generation should succeed");
        let mut visited = vec![vec![false; map.width()]; map.height()];

        // Implement BFS to find all reachable tiles from the entry point
        let entry_x = map.width() / 2;
        let entry_y = map.height() - 1; // Entry is at bottom, not top
        let mut to_visit = vec![(entry_x, entry_y)];
        visited[entry_y][entry_x] = true;

        while let Some((x, y)) = to_visit.pop() {
            // Check all 4 adjacent tiles
            let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
            for (dx, dy) in directions {
                let new_x = (x as i32 + dx) as usize;
                let new_y = (y as i32 + dy) as usize;

                if map.in_bounds(new_x, new_y) && !visited[new_y][new_x] {
                    let tile = map.tile_at(new_x, new_y);
                    if tile == TileType::Floor || tile == TileType::Door {
                        visited[new_y][new_x] = true;
                        to_visit.push((new_x, new_y));
                    }
                }
            }
        }

        // Verify all walkable tiles are reachable
        for (y, row) in visited.iter().enumerate() {
            for (x, &was_visited) in row.iter().enumerate() {
                let tile = map.tile_at(x, y);
                if tile == TileType::Floor || tile == TileType::Door {
                    assert!(
                        was_visited,
                        "Walkable tile at ({}, {}) is unreachable (island)",
                        x, y
                    );
                }
            }
        }
    }
}
