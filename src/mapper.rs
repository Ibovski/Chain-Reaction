use ggez::mint::Point2;

use crate::game_constants;

pub fn pos_to_row_col(pos_x: f32, pos_y: f32) -> (usize, usize) {
    ((((pos_x - game_constants::MARGIN) / game_constants::CELL_WIDTH) as usize), (((pos_y - game_constants::MARGIN) / game_constants::CELL_HEIGHT) as usize))
}

pub fn row_col_to_pos(row: usize, column: usize) -> Point2<f32> {
    Point2{x:(row  as f32) * game_constants::CELL_WIDTH + game_constants::MARGIN,
           y: (column as f32) * game_constants::CELL_HEIGHT + game_constants::MARGIN}
}