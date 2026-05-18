#[allow(unused_imports)]
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

    #[test]
    fn spawn_player_updates_location() {
        let mut player = character_creation("TestPlayer".to_string(), CharacterType::Rogue);

        spawn_player(&mut player, (3, 7));

        assert_eq!(player.get_location(), (3, 7));
    }

    #[test]
    fn spawn_player_overwrites_existing_location() {
        let mut player = character_creation("TestPlayer".to_string(), CharacterType::Rogue);
        move_player(&mut player, 1, 1);

        spawn_player(&mut player, (9, 2));

        assert_eq!(player.get_location(), (9, 2));
    }

    #[test]
    fn move_player_can_be_called_multiple_times() {
        let mut player = character_creation("TestPlayer".to_string(), CharacterType::Warrior);

        move_player(&mut player, 2, 3);
        move_player(&mut player, 8, 1);

        assert_eq!(player.get_location(), (8, 1));
    }

    #[test]
    fn character_creation_starts_each_class_at_origin() {
        let rogue = character_creation("Rogue".to_string(), CharacterType::Rogue);
        let warrior = character_creation("Warrior".to_string(), CharacterType::Warrior);
        let wizard = character_creation("Wizard".to_string(), CharacterType::Wizard);

        assert_eq!(rogue.get_location(), (0, 0));
        assert_eq!(warrior.get_location(), (0, 0));
        assert_eq!(wizard.get_location(), (0, 0));
    }

    #[test]
    fn character_creation_starts_with_empty_inventory() {
        let player = character_creation("TestPlayer".to_string(), CharacterType::Rogue);

        assert!(player.get_items().is_empty());
    }

    #[test]
    fn moving_player_does_not_change_other_player_state() {
        let mut player = character_creation("TestPlayer".to_string(), CharacterType::Rogue);

        move_player(&mut player, 4, 6);

        assert_eq!(player.get_name(), "TestPlayer");
        assert_eq!(player.get_hp(), 100);
        assert_eq!(player.get_level(), 1);
        assert_eq!(player.get_xp(), 0);
    }
}
