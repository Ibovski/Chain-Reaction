use std::collections::VecDeque;

use ggez::{graphics::{self, DrawParam}, GameResult, mint::{Point2, Vector2}};

use crate::{game_constants, entities::{Player, Direction}, mapper::*, helper::*};

#[derive(Debug)]
/// Enum representing different states for an image.
enum ImageState {
    /// The default state.
    Default,
    /// First state.
    First,
    /// Second state.
    Second,
    /// Third state.
    Third,
    /// Fourth state.
    Fourth,
}

impl ImageState {
    /// Converts an `ImageState` variant to its corresponding integer value.
    ///
    /// # Returns
    ///
    /// An integer representing the `ImageState` variant.
    fn to_int(&self) -> usize {
        match self {
            ImageState::Default => 0,
            ImageState::First => 1,
            ImageState::Second => 2,
            ImageState::Third => 3,
            ImageState::Fourth => 4,
        }
    }

    /// Converts an integer to the corresponding `ImageState` variant.
    ///
    /// # Arguments
    ///
    /// * `idx` - The integer value to convert to an `ImageState` variant.
    ///
    /// # Returns
    ///
    /// An `ImageState` variant corresponding to the input integer, or `ImageState::Default` if no match is found.
    fn to_image_state(idx: i32) -> Self {
        match idx {
            0 => ImageState::Default,
            1 => ImageState::First,
            2 => ImageState::Second,
            3 => ImageState::Third,
            4 => ImageState::Fourth,
            _ => ImageState::Default,
        }
    }

    /// Gets the next state in the sequence.
    ///
    /// # Returns
    ///
    /// The next `ImageState` variant in the sequence.
    fn next(&self) -> Self {
        match self {
            ImageState::Default => ImageState::First,
            ImageState::First => ImageState::Second,
            ImageState::Second => ImageState::Third,
            ImageState::Third => ImageState::Fourth,
            ImageState::Fourth => ImageState::Fourth,
        }
    }
}

/// Represents a cell in the grid with its current state and player entities.
struct Cell {
    /// The state of the cell's image.
    state: ImageState,
    /// The position of the cell.
    pos: Point2<f32>,
    /// The players currently occupying the cell.
    players: VecDeque<Player>,
    /// The directions associated with the cell's image.
    image_dir: Vec<Direction>,
    /// Indicates if the cell has been triggered.
    triggered: bool,
}

impl Cell {
    /// Creates a new `Cell` at the specified position.
    ///
    /// # Arguments
    ///
    /// * `pos` - The position of the cell.
    ///
    /// # Returns
    ///
    /// A `Result` containing the initialized `Cell` or an error if creation fails.
    fn new(pos: Point2<f32>) -> GameResult<Cell> {
        let (row, column) = pos_to_row_col(pos.x, pos.y);
        Ok(Cell {
            state: ImageState::Default,
            pos,
            players: VecDeque::new(),
            image_dir: get_image_dir(row, column),
            triggered: false,
        })
    }

    /// Checks if the cell has been triggered and updates the players in the cell.
    ///
    /// This method checks if the cell has been triggered and updates the players in the cell.
    ///
    /// # Returns
    ///
    /// `true` if the cell has been triggered and updated, `false` if the cell has not been triggered.
    fn has_triggered(&mut self) -> bool {
        if self.triggered {
            let temp = &mut self.players;
            let mut state = self.state.to_int();
            let (row, column) = pos_to_row_col(self.pos.x, self.pos.y);

            if get_neighbours_count(row, column) == 2 && state == 3 {
                state = 2;
            }

            if get_neighbours_count(row, column) == 3 && state == 4 {
                state = 3;
            }

            for idx in 0..state {
                temp[idx].set_speed(game_constants::PLAYER_SPEED);
                temp[idx].update(&mut self.triggered);
            }

            if !self.triggered {
                return true;
            }
        }

        false
    }

    /// Draws the cell's players on the given canvas.
    ///
    /// # Arguments
    ///
    /// * `canvas` - A mutable reference to the graphics canvas to draw on.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    fn draw_cell(&mut self, canvas: &mut graphics::Canvas) -> GameResult<()> {
        let temp = &self.players;
        for player in temp {
            canvas.draw(
                player.get_texture(),
                DrawParam::default()
                    .dest(player.get_position())
                    .rotation(player.get_rotation())
                    .scale(Vector2 { x: 1.0, y: 1.0 }),
            );
        }
        Ok(())
    }

    /// Changes the state of the cell and adds a player to it.
    ///
    /// # Arguments
    ///
    /// * `player` - The player to be added to the cell.
    ///
    /// # Returns
    ///
    /// `true` if the state was successfully changed and the player added, `false` otherwise.
    fn change_state(&mut self, player: Player) -> bool {
        if self.players.is_empty() {
            self.players.push_back(player);
        } else if self.players[0].get_id() != player.get_id() {
            return false;
        } else {
            self.players.push_back(player);
        }

        self.state = self.state.next();
        true
    }

    /// Triggers the cell if its state equals the number of its neighbours.
    ///
    /// This method triggers the cell if its state equals the number of its neighbours.
    ///
    /// # Arguments
    ///
    /// * `row` - The row index of the cell.
    /// * `column` - The column index of the cell.
    fn trigger(&mut self, row: usize, column: usize) {
        let neighbour_counts = get_neighbours_count(row, column);
        if neighbour_counts == 2 {
            if let ImageState::Second = self.state {
                self.triggered = true;        
            }
        }
        if neighbour_counts == 3 {
            if let ImageState::Third = self.state {
                self.triggered = true;        
            }
        }
        if neighbour_counts == 4 {
            if let ImageState::Fourth = self.state {
                self.triggered = true;        
            }
        }
    }
}

/// Represents a grid of cells containing players and their interactions.
pub struct Grid {
    /// The grid containing cells.
    grid: VecDeque<VecDeque<Cell>>,
    /// Counts of players based on their IDs.
    players_id_count: Vec<i32>,
}

impl Grid {
    /// Creates a new empty grid.
    ///
    /// Initializes a grid with empty cells based on game constants.
    ///
    /// # Returns
    ///
    /// A `Result` containing the initialized `Grid` or an error if creation fails.
    pub fn create() -> GameResult<Grid> {
        let mut grid: VecDeque<VecDeque<Cell>> = VecDeque::new();
        for row in 0..game_constants::TABLE_ROWS {
            grid.push_back(VecDeque::new());
            for column in 0..game_constants::TABLE_COLUMNS {
                let pos = row_col_to_pos(row, column);
                grid[row].push_back(Cell::new(pos)?);
            }
        }
        Ok(Grid {
            grid,
            players_id_count: vec![0, 0],
        })
    }

    /// Updates the grid by triggering cells and managing player interactions.
    ///
    /// This method iterates through the grid, checks for triggered cells, and updates
    /// player interactions based.
    pub fn update(&mut self) {
        for row in 0..game_constants::TABLE_ROWS {
            for column in 0..game_constants::TABLE_COLUMNS {
                let triggering = self.grid[row][column].has_triggered();
                if triggering {
                    let player = self.grid[row][column].players[0].clone();
                    let remove_players_count = get_neighbours_count(row, column);
                    self.players_id_count[player.get_id() as usize] -= remove_players_count as i32;
                    self.update_neighbours(row, column, player);
                    for _ in 0..remove_players_count {
                        self.grid[row][column].players.pop_front();
                    }
                    self.grid[row][column].state =
                        ImageState::to_image_state(self.grid[row][column].players.len() as i32);
                }
            }
        }
    }

    /// Draws the entire grid on the given canvas.
    ///
    /// # Arguments
    ///
    /// * `canvas` - A mutable reference to the graphics canvas to draw on.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    pub fn draw_grid(&mut self, canvas: &mut graphics::Canvas) -> GameResult<()> {
        for row in 0..game_constants::TABLE_ROWS {
            for column in 0..game_constants::TABLE_COLUMNS {
                let _ = self.grid[row][column].draw_cell(canvas);
            }
        }
        Ok(())
    }

    /// Changes the state of a cell and adds a player to it.
    ///
    /// # Arguments
    ///
    /// * `row` - The row index of the cell.
    /// * `column` - The column index of the cell.
    /// * `player` - The player to be added to the cell.
    ///
    /// # Returns
    ///
    /// `true` if the state was successfully changed and the player added, `false` otherwise.
    pub fn change_cell_state(&mut self, row: usize, column: usize, player: &Player) -> bool {
        let pos = row_col_to_pos(row, column);
       
        let mut new_player = player.clone();
        let current_state = &self.grid[row][column].state;
        new_player.set_pos(Point2 {
            x: pos.x + game_constants::IMAGE_WIDTH,
            y: pos.y + game_constants::IMAGE_HEIGHT,
        });
        match current_state {
            ImageState::First => {
                new_player.set_rotation(from_direction_to_rotation(
                    &self.grid[row][column].image_dir[1],
                ));
                new_player.set_dir(self.grid[row][column].image_dir[1].clone());
                new_player.set_go_to_pos(from_direction_to_go_pos(
                    &self.grid[row][column].image_dir[1],
                    new_player.get_position(),
                ))
            }
            ImageState::Second => {
                new_player.set_rotation(from_direction_to_rotation(
                    &self.grid[row][column].image_dir[2],
                ));
                new_player.set_dir(self.grid[row][column].image_dir[2].clone());
                new_player.set_go_to_pos(from_direction_to_go_pos(
                    &self.grid[row][column].image_dir[2],
                    new_player.get_position(),
                ))
            }
            ImageState::Third => {
                new_player.set_rotation(from_direction_to_rotation(
                    &self.grid[row][column].image_dir[3],
                ));
                new_player.set_dir(self.grid[row][column].image_dir[3].clone());
                new_player.set_go_to_pos(from_direction_to_go_pos(
                    &self.grid[row][column].image_dir[3],
                    new_player.get_position(),
                ))
            }
            _ => {
                new_player.set_rotation(from_direction_to_rotation(
                    &self.grid[row][column].image_dir[0],
                ));
                new_player.set_dir(self.grid[row][column].image_dir[0].clone());
                new_player.set_go_to_pos(from_direction_to_go_pos(
                    &self.grid[row][column].image_dir[0],
                    new_player.get_position(),
                ))
            }
        }
        let success_turn = self.grid[row][column].change_state(new_player);
        if success_turn == true {
            self.players_id_count[player.get_id() as usize] += 1;
            self.grid[row][column].trigger(row, column);
        }
        success_turn
    }

    /// Gets a reference to the count of players based on their IDs.
    ///
    /// # Returns
    ///
    /// A reference to the vector containing player counts.
    pub fn get_all_players(&self) -> &Vec<i32> {
        &self.players_id_count
    }

    /// Gets the total count of players in the grid.
    ///
    /// # Returns
    ///
    /// The total count of players in the grid.
    pub fn get_players_count(&self) -> usize {
        let mut sum: usize = 0;
        for i in 0..self.players_id_count.len() {
            sum += self.players_id_count[i] as usize;
        }
        sum
    }

    /// Clears and fills a cell with a player and updates its state.
    ///
    /// # Arguments
    ///
    /// * `row` - The row index of the cell.
    /// * `column` - The column index of the cell.
    /// * `player` - The player to be added to the cell.
    fn clear_and_fill(&mut self, row: usize, column: usize, player: &Player) {
        let temp = &mut self.grid[row][column];
        let new_len = temp.players.len() + 1;
        if new_len > 1 {
            self.players_id_count[temp.players[0].get_id() as usize] -= temp.players.len() as i32;
        }
        temp.players.clear();
        temp.state = ImageState::Default;
        for _i in 0..new_len {
            self.change_cell_state(row, column, &player);
        }
    }

    /// Updates the neighboring cells of a triggered cell.
    ///
    /// # Arguments
    ///
    /// * `row` - The row index of the triggered cell.
    /// * `column` - The column index of the triggered cell.
    /// * `player` - The player that triggered the cell.
    fn update_neighbours(&mut self, row: usize, column: usize, player: Player) {
        if row > 0 {
            self.clear_and_fill(row - 1, column, &player);
        }
        if row < game_constants::LAST_ROWCOL_INDEX {
            self.clear_and_fill(row + 1, column, &player)
        }
        if column > 0 {
            self.clear_and_fill(row, column - 1, &player)
        }
        if column < game_constants::LAST_ROWCOL_INDEX {
            self.clear_and_fill(row, column + 1, &player)
        }
    }
}
