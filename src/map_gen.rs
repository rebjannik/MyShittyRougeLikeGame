#[allow(dead_code)]
pub enum TileType {
    Wall,
    Floor,
    Door,
}

pub struct Map {
    tiles: Vec<Vec<TileType>>,
}



#[test]
fn test_tile_type() {
    use TileType::*;

    for tile in [Wall, Floor, Door] {
        let _ = tile;
    }
}

#[test]
fn test_map_gen() {
    let map = Map {
        tiles: vec![vec![TileType::Floor]],
    };
    let _ = map;
}