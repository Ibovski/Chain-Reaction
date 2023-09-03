use ggez::mint::Point2;

use crate::{game_constants, entities::Direction};

/// Converts a position (pos_x, pos_y) to row and column indices in a grid.
/// 
/// # Arguments
/// 
/// * `pos_x` - The x-coordinate of the position.
/// * `pos_y` - The y-coordinate of the position.
/// 
/// # Returns
/// 
/// A tuple containing the row and column indices corresponding to the position.
pub fn pos_to_row_col(pos_x: f32, pos_y: f32) -> (usize, usize) {
    // Calculate the row index by subtracting MARGIN and dividing by CELL_WIDTH
    let row = (((pos_x - game_constants::MARGIN) / game_constants::CELL_WIDTH) as usize);
    // Calculate the column index by subtracting MARGIN and dividing by CELL_HEIGHT
    let column = (((pos_y - game_constants::MARGIN) / game_constants::CELL_HEIGHT) as usize);
    // Return a tuple containing row and column indices
    (row, column)
}

/// Converts row and column indices to a position (Point2<f32>) in a grid.
/// 
/// # Arguments
/// 
/// * `row` - The row index.
/// * `column` - The column index.
/// 
/// # Returns
/// 
/// A Point2<f32> representing the position corresponding to the row and column indices.
pub fn row_col_to_pos(row: usize, column: usize) -> Point2<f32> {
    // Calculate the x-coordinate by multiplying row by CELL_WIDTH and adding MARGIN
    let x = (row as f32) * game_constants::CELL_WIDTH + game_constants::MARGIN;
    // Calculate the y-coordinate by multiplying column by CELL_HEIGHT and adding MARGIN
    let y = (column as f32) * game_constants::CELL_HEIGHT + game_constants::MARGIN;
    // Return a Point2<f32> struct representing the position
    Point2 { x, y }
}

/// Converts a Direction enum into a rotation angle in radians.
/// 
/// # Arguments
/// 
/// * `dir` - A reference to the Direction enum.
/// 
/// # Returns
/// 
/// A f32 value representing the rotation angle in radians.
pub fn from_direction_to_rotation(dir: &Direction) -> f32 {
    // Match the direction and return the corresponding rotation angle
    match dir {
        Direction::LEFT => 1.57,   // 90 degrees in radians
        Direction::UP => 0.0,      // 0 degrees in radians
        Direction::RIGHT => 4.71,  // 270 degrees in radians
        Direction::DOWN => 3.14,   // 180 degrees in radians
    }
}

/// Calculates the new position based on a Direction enum and the current position.
/// 
/// # Arguments
/// 
/// * `dir` - A reference to the Direction enum indicating the direction to move.
/// * `pos` - The current position as a Point2<f32>.
/// 
/// # Returns
/// 
/// A Point2<f32> representing the destination for the player.
pub fn from_direction_to_go_pos(dir: &Direction, pos: Point2<f32>) -> Point2<f32> {
    // Match the direction and calculate the new position accordingly
    match dir {
        Direction::LEFT => Point2 { x: pos.x - game_constants::CELL_WIDTH, y: pos.y },
        Direction::RIGHT => Point2 { x: pos.x + game_constants::CELL_WIDTH, y: pos.y },
        Direction::UP => Point2 { x: pos.x, y: pos.y - game_constants::CELL_HEIGHT },
        Direction::DOWN => Point2 { x: pos.x, y: pos.y + game_constants::CELL_HEIGHT },
    }
}
