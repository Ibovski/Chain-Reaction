use ggez::{graphics::Image, GameError, GameResult, Context};
use crate::{game_constants, entities::Direction};
use std::path::Path;

/// Checks if two floating-point numbers are approximately equal within a specified boundary.
///
/// # Arguments
///
/// * `actual` - The actual floating-point number.
/// * `expected` - The expected floating-point number.
/// * `boundary` - The allowed difference between `actual` and `expected`.
///
/// # Returns
///
/// `true` if `actual` is approximately equal to `expected` within the given boundary, `false` otherwise.
pub fn approximately_eq(actual: f32, expected: f32, boundary: f32) -> bool {
    if expected + boundary > actual && expected - boundary < actual {
        return true;
    }
    false
}

/// Loads an image from a file path and returns it as an `Image` object.
///
/// # Arguments
///
/// * `ctx` - A mutable reference to the game context.
/// * `path` - The path to the image file.
///
/// # Returns
///
/// A `Result` containing the loaded `Image` or a `GameError` if the file does not exist.
pub fn image_from_path(ctx: &mut Context, path: String) -> GameResult<Image> {
    let file = game_constants::DIRECTORY.to_string() + &path;
    let file_path = Path::new(&file);
    if file_path.exists() {
        Ok(Image::from_path(ctx, path).unwrap())
    } else {
        Err(GameError::FilesystemError(format!("No file with path '{}' exists", file)))
    }
}

/// Determines the number of neighboring cells for a given cell in the grid.
///
/// # Arguments
///
/// * `row` - The row index of the cell.
/// * `column` - The column index of the cell.
///
/// # Returns
///
/// The number of neighboring cells, which can be 2, 3, or 4, based on the cell's position in the grid.
pub fn get_neighbours_count(row: usize, column: usize) -> usize {
    match (row, column) {
        (0, 0) | (0, game_constants::LAST_ROWCOL_INDEX) |
        (game_constants::LAST_ROWCOL_INDEX, 0) | (game_constants::LAST_ROWCOL_INDEX, game_constants::LAST_ROWCOL_INDEX) =>
            2,
        (0, _) | (game_constants::LAST_ROWCOL_INDEX, _) | (_, game_constants::LAST_ROWCOL_INDEX) | (_, 0) =>
            3,
        (_, _) =>
            4,
    }
}

/// Determines the directions of neighboring cells for a given cell in the grid.
///
/// # Arguments
///
/// * `column` - The column index of the cell.
/// * `row` - The row index of the cell.
///
/// # Returns
///
/// A vector containing `Direction` enum values representing the directions of neighboring cells.
pub fn get_image_dir(column: usize, row: usize) -> Vec<Direction> {
    match (row, column) {
        (game_constants::LAST_ROWCOL_INDEX, col) => {
            if col < game_constants::LAST_ROWCOL_INDEX {
                vec![Direction::UP, Direction::LEFT, Direction::RIGHT, Direction::DOWN]
            } else {
                vec![Direction::UP, Direction::RIGHT, Direction::DOWN, Direction::LEFT]
            }
        },
        (0, col) => {
            if col < game_constants::LAST_ROWCOL_INDEX {
                vec![Direction::DOWN, Direction::LEFT, Direction::RIGHT, Direction::UP]
            } else {
                vec![Direction::DOWN, Direction::RIGHT, Direction::UP, Direction::LEFT]
            }
        },
        (_, 0) => {
            vec![Direction::UP, Direction::LEFT, Direction::DOWN, Direction::RIGHT]
        },
        (_, game_constants::LAST_ROWCOL_INDEX) => {
            vec![Direction::UP, Direction::RIGHT, Direction::DOWN, Direction::LEFT]
        }
        (_,_) => {
            vec![Direction::DOWN, Direction::RIGHT, Direction::UP, Direction::LEFT]
        }
    }
}
