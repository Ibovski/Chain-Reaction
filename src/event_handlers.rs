use ggez::mint::Point2;
use ggez::{Context, GameResult};
use ggez::graphics::{self, Image};
use ggez::event::{self, EventHandler};
use crate::entities::Player;
use crate::game_constants;
use crate::grid::Grid;
use crate::helper::image_from_path;
use crate::mapper::pos_to_row_col;


/**
 * Enum representing the possible states of a game.
 *
 * This enum is used to define the different states that a game can be in.
 * It includes three variants: MainMenu, InGame, and EndGame, which represent
 * the main menu screen, the in-game state, and the end game state respectively.
 */
pub enum GameState {
    /// Represents the main menu state of the game.
    MainMenu,

    /// Represents the in-game state of the game.
    InGame,

    /// Represents the end game state of the game.
    EndGame,
}


/**
 * GameConfiguration Struct
 *
 * Represents the configuration and state of a game. This struct encapsulates
 * various game-related data and methods for initializing and checking the game's
 * progress.
 */
pub struct GameConfiguration {
    /// The game grid where the gameplay takes place.
    grid: Grid,

    /// The background image for in game displayed in the game.
    game_background: Image,

    /// The background image for start game displayed in the game.
    main_menu_background: Image,

    /// The background image for player1 win displayed in the game.
    p1_win_background: Image,

    /// The background image for player2 win displayed in the game.
    p2_win_background: Image,

    /// The current player's turn.
    turn: usize,

    /// The list of players, including their game results.
    players: Vec<GameResult<Player>>,

    /// The current state of the game.
    game_state: GameState,

    /// The player ID of the winner (0 if no winner).
    winner: i32,
}

impl GameConfiguration {
    /**
     * Creates a new instance of GameConfiguration.
     *
     * Initializes a new game configuration with a game grid, background image, players,
     * and sets the initial game state to MainMenu.
     *
     * # Arguments
     * - `ctx`: A mutable reference to the game context.
     *
     * # Returns
     * A Result containing the initialized GameConfiguration if successful,
     * or an error if any initialization step fails.
     */
    pub fn new(ctx: &mut Context) -> GameResult<GameConfiguration> {
        let grid = Grid::create()?;
        let mut players = Vec::new();
        let background = image_from_path(ctx, r"\grid.png".to_string())?;
        let start_game = image_from_path(ctx, r"\start_game.png".to_string())?;
        let p1_win = image_from_path(ctx, r"\p1_win.png".to_string())?;
        let p2_win = image_from_path(ctx, r"\p2_win.png".to_string())?;
        // Create and add two players with their respective images.
        players.push(Player::from_path(String::from(r"\player1.png"), ctx));
        players.push(Player::from_path(String::from(r"\player2.png"), ctx));

        Ok(GameConfiguration {
            grid,
            game_background: background,
            main_menu_background: start_game,
            p1_win_background: p1_win,
            p2_win_background: p2_win,
            turn: 0,
            players,
            game_state: GameState::MainMenu,
            winner: 0,
        })
    }

    /**
     * Checks for a winner in the game.
     *
     * This method examines the current state of the game grid and updates the winner
     * if there is one. The winner is identified by player IDs (1 or 2) or set to 0 if
     * there is no winner yet.
     */
    pub fn winner_check(&mut self) {
        let players = self.grid.get_all_players();
        if self.grid.get_players_count() > 1 {
            if players[0] == 0 {
                self.winner = 2;
            } else if players[1] == 0 {
                self.winner = 1;
            }
        }
    }
}


/**
 * EventHandler Implementation for GameConfiguration
 *
 * This implementation provides event handling functions for the `GameConfiguration` struct,
 * allowing it to respond to game-related events, such as updates, mouse button clicks, and drawing.
 */
impl EventHandler for GameConfiguration {
    /**
     * Handles the game update event.
     *
     * Updates the game's state by calling necessary update functions, including grid updates
     * and winner checking. If a winner is found, the game state transitions to EndGame.
     *
     * # Arguments
     * - `_ctx`: A mutable reference to the game context.
     *
     * # Returns
     * A `GameResult` indicating success or an error, with no additional data.
     */
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.grid.update();
        self.winner_check();
        if self.winner != 0 {
            self.game_state = GameState::EndGame;
        }
        Ok(())
    }

    /**
     * Handles the mouse button up event.
     *
     * Processes mouse button clicks based on the current game state. In the InGame state,
     * it allows players to interact with the grid by changing cell states. In other states,
     * it checks for a click on the Start Game button to transition to the InGame state.
     *
     * # Arguments
     * - `_ctx`: A mutable reference to the game context.
     * - `button`: The mouse button that was released.
     * - `pos_x`: The X-coordinate of the mouse click.
     * - `pos_y`: The Y-coordinate of the mouse click.
     *
     * # Returns
     * A `GameResult` indicating success or an error, with no additional data.
     */
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: event::MouseButton, pos_x: f32, pos_y: f32) -> GameResult<()> {
        if button == event::MouseButton::Left &&
            pos_x > game_constants::MARGIN && pos_x < game_constants::SCREEN_WIDTH - game_constants::MARGIN &&
            pos_y > game_constants::MARGIN && pos_y < game_constants::SCREEN_HEIGHT - game_constants::MARGIN {
            if let GameState::InGame = self.game_state {
                let (row, column) = pos_to_row_col(pos_x, pos_y);
                let success_turn = self.grid.change_cell_state(row, column, self.players[self.turn].as_ref().expect("No player in this turn "));
                if success_turn {
                    self.turn = (self.turn + 1) % game_constants::PLAYER_COUNT;
                }
            } else {
                if 103.0 <= pos_x && pos_x <= 506.0 &&
                    263.0 <= pos_y && pos_y <= 349.0 {
                    self.game_state = GameState::InGame
                }
            }
        }
        Ok(())
    }

    /**
     * Handles the drawing event.
     *
     * Draws the game's elements based on the current game state. In the EndGame state,
     * it displays a message indicating the winner. In the InGame state, it draws the
     * background and the game grid. In the MainMenu state, it shows the "Start Game" button.
     *
     * # Arguments
     * - `ctx`: A mutable reference to the game context.
     *
     * # Returns
     * A `GameResult` indicating success or an error, with no additional data.
     */
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas;
        if let GameState::EndGame = self.game_state {
            canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
            if self.winner == 1 {
                canvas.draw(&self.p1_win_background, Point2 { x: 0.0, y: 0.0});
            } else {
                canvas.draw(&self.p2_win_background, Point2 { x: 0.0, y: 0.0});
            }
        } else if let GameState::InGame = self.game_state {
            canvas = graphics::Canvas::from_frame(ctx, graphics::Color::WHITE);
            canvas.draw(&self.game_background, Point2 { x: 0.0, y: 0.0 });
            let _ = self.grid.draw_grid(&mut canvas);
        } else {
            canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
            canvas.draw(&self.main_menu_background, Point2 { x: 0.0, y: 0.0});
        }
        canvas.finish(ctx)?;
        Ok(())
    }
}

