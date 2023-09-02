use ggez::mint::Point2;

use crate::{game_constants, entities::Direction};

pub fn pos_to_row_col(pos_x: f32, pos_y: f32) -> (usize, usize) {
    ((((pos_x - game_constants::MARGIN) / game_constants::CELL_WIDTH) as usize), (((pos_y - game_constants::MARGIN) / game_constants::CELL_HEIGHT) as usize))
}

pub fn row_col_to_pos(row: usize, column: usize) -> Point2<f32> {
    Point2{x:(row  as f32) * game_constants::CELL_WIDTH + game_constants::MARGIN,
           y: (column as f32) * game_constants::CELL_HEIGHT + game_constants::MARGIN}
}

pub fn from_direction_to_rotation(dir: &Direction) -> f32 {
    match dir {
        Direction::LEFT => 1.57,
        Direction::UP => 0.0,
        Direction::RIGHT => 4.71,
        Direction::DOWN => 3.14
    }
}
pub fn from_direction_to_go_pos(dir: &Direction, pos: Point2<f32>) -> Point2<f32> {
    match dir {
        Direction::LEFT => Point2 { x: pos.x - game_constants::CELL_WIDTH, y: pos.y},
        Direction::RIGHT => Point2{x: pos.x + game_constants::CELL_WIDTH, y: pos.y},
        Direction::UP => Point2{x: pos.x, y: pos.y - game_constants::CELL_HEIGHT},
        Direction::DOWN => Point2{x: pos.x, y: pos.y + game_constants::CELL_HEIGHT}
    }
}