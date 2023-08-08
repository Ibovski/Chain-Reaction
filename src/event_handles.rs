use ggez::{Context, GameResult};
use ggez::graphics::{self};
use ggez::event::{self, EventHandler};
use crate::entities::Player;
use crate::game_constants;
use crate::grid::{Grid};

pub struct MyGame {
    grid: Grid,
    turn: usize,
    players: Vec<GameResult<Player>>
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<MyGame> {
        let grid = Grid::create(ctx)?;
        let mut players = Vec::new();
        players.push(Player::from_path(String::from("/pixil-frame-0.png"), ctx));
        players.push(Player::from_path(String::from("/pixil-frame-1.png"), ctx));
        Ok(MyGame {grid,turn: 0, players})
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, 
        button: event::MouseButton, pos_x: f32, pos_y: f32) ->GameResult<()> {
        if button == event::MouseButton::Left && 
        pos_x > game_constants::MARGIN && pos_x < game_constants::SCREEN_WIDTH - game_constants::MARGIN &&
        pos_y > game_constants::MARGIN && pos_y < game_constants::SCREEN_HEIGHT - game_constants::MARGIN {
            let row = ((pos_x - game_constants::MARGIN) / game_constants::CELL_WIDTH) as usize;
            let column = ((pos_y - game_constants::MARGIN) / game_constants::CELL_HEIGHT) as usize;
            self.grid.change_cell_state(row, column, self.players[self.turn].as_ref().expect("No player in this turn "));
            self.turn = (self.turn + 1) % 2;
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::WHITE);
        
        let _ = self.grid.draw_grid(&mut canvas, ctx);
        
        canvas.finish(ctx)?;
        Ok(())
    }
}
