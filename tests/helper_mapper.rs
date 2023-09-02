#[cfg(test)]
mod tests {
    use ggez::mint::Point2;
    use bee_battle::{game_constants, entities::Direction, mapper::*, helper::get_neighbours_count};

    // Test the get_neighbours_count function
    #[test]
    fn test_get_neighbours_count() {
        assert_eq!(get_neighbours_count(0, 0), 2);
        assert_eq!(get_neighbours_count(0, game_constants::LAST_ROWCOL_INDEX), 2);
        assert_eq!(get_neighbours_count(game_constants::LAST_ROWCOL_INDEX, 0), 2);
        assert_eq!(get_neighbours_count(game_constants::LAST_ROWCOL_INDEX, game_constants::LAST_ROWCOL_INDEX), 2);
    }

    // Test the from_direction_to_rotation function
    #[test]
    fn test_from_direction_to_rotation() {
        assert_eq!(from_direction_to_rotation(&Direction::LEFT), 1.57);
        assert_eq!(from_direction_to_rotation(&Direction::UP), 0.0);
        assert_eq!(from_direction_to_rotation(&Direction::RIGHT), 4.71);
        assert_eq!(from_direction_to_rotation(&Direction::DOWN), 3.14);
    }

    // Test the from_direction_to_go_pos function
    #[test]
    fn test_from_direction_to_go_pos() {
        let pos = Point2 { x: 120.0, y: 120.0 };

        assert_eq!(from_direction_to_go_pos(&Direction::LEFT, pos), Point2 { x: 45.0, y: 120.0 });
        assert_eq!(from_direction_to_go_pos(&Direction::RIGHT, pos), Point2 { x: 195.0, y: 120.0 });
        assert_eq!(from_direction_to_go_pos(&Direction::UP, pos), Point2 { x: 120.0, y: 45.0 });
        assert_eq!(from_direction_to_go_pos(&Direction::DOWN, pos), Point2 { x: 120.0, y: 195.0 });
    }
}
