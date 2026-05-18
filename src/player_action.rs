use crate::models::{CharacterType, Player, Weapon, WeaponType};

pub fn character_creation(name: String, character_type: CharacterType) -> Player {
    Player::new(name, character_type, (0, 0))
}

pub fn spawn_player(player: &mut Player, location: (usize, usize)) {
    player.move_to(location.0, location.1);
}

pub fn move_player(player: &mut Player, x: usize, y: usize) {
    // TODO: Add check for collision with walls and monsters
    player.move_to(x, y);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_character_creation() {
        let player = character_creation("TestPlayer".to_string(), CharacterType::Rogue);
        assert_eq!(player.get_name(), "TestPlayer");
        assert_eq!(player.get_hp(), 100);
        assert_eq!(player.get_attr()[0], 0);
        assert_eq!(player.get_attr()[1], 0);
        assert_eq!(player.get_attr()[2], 0);
        assert_eq!(player.get_attr()[3], 0);
        assert_eq!(player.get_level(), 1);
        assert_eq!(player.get_xp(), 0);
        assert_eq!(player.get_location(), (0, 0));
    }

    #[test]
    fn test_move_player() {
        let mut player = character_creation("TestPlayer".to_string(), CharacterType::Warrior);
        move_player(&mut player, 5, 5);
        assert_eq!(player.get_location(), (5, 5));
    }
}
