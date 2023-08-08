use ggez::{graphics::{Mesh, DrawMode, Rect, Color, self, DrawParam, Transform}, Context, GameResult, mint::{Point2, Vector2}};

use crate::{game_constants, entities::{Player, Direction}};
enum ImageState {
    Default,
    First,
    Second,
    Third,
    Fourth,
}

impl ImageState {
    fn to_int(&self) -> u32 {
        match self {
            ImageState::Default => 0,
            ImageState::First => 1,
            ImageState::Second => 2,
            ImageState::Third => 3,
            ImageState::Fourth => 4,
        }
    }
    fn next(&self) -> Self {
        match self {
            ImageState::Default => ImageState::First,
            ImageState::First => ImageState::Second,
            ImageState::Second => ImageState::Third,
            ImageState::Third =>  ImageState::Fourth,
            ImageState::Fourth => ImageState::Default,
        }
    }
}

struct Cell {
    cell_mesh: Mesh,
    state: ImageState,
    players: Vec<Player>,
    triggered: bool,
}

impl Cell {
    fn new(ctx: &mut Context) -> GameResult<Cell> {
        
        let cell_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(1.0),
            Rect::new(0.0,0.0,game_constants::CELL_WIDTH, game_constants::CELL_HEIGHT),
            Color::BLACK,
        )?;
        Ok(Cell{
            cell_mesh, 
            state: ImageState::Default,
            players: Vec::new(),
            triggered: false})
    }
    fn update(&mut self) {
       
       if self.triggered == true {
            let temp = &mut self.players;
            for player in temp {
                player.set_speed(1.1);
                player.update(5.0, &mut self.triggered);
            }
        }
        
    }
    fn draw_cell(&mut self, canvas: &mut graphics::Canvas, ctx: &mut Context, dst: Point2<f32>) -> GameResult<()>{
        if self.state.to_int() != 0 {
            
           // temp.set_rotation(new_rotation);
            let temp = &self.players;
          //  let mut new_rotation = ctx.time.delta().as_secs_f32() * 3.0 + player.get_rotation();
            for player in temp {
                canvas.draw(
                    player.get_texture(), 
                DrawParam::default()
                .dest(player.get_position())
                .rotation(player.get_rotation())
                .scale(Vector2{x: 0.6, y: 0.6}));   
            }
        }
        canvas.draw(&self.cell_mesh, DrawParam::default().dest(dst));
        Ok(())
    }

    fn change_state(&mut self, player: Player) -> bool{
        if self.players.is_empty(){
            self.players.push(player);
        }
        else if self.players[0].get_id() != player.get_id() {
            return false;
        }
        else {
            self.players.push(player);
        }
        
        self.state = self.state.next();
        true
    }

    fn trigger(&mut self, row: usize, column: usize) {
        match (row, column) {
            (0, 0) | (0, game_constants::LAST_ROWCOL_INDEX) | 
            (game_constants::LAST_ROWCOL_INDEX, 0) | (game_constants::LAST_ROWCOL_INDEX, game_constants::LAST_ROWCOL_INDEX) => {
                if let ImageState::Second = self.state {
                    self.triggered = true;
                }
            }
            (0, _) | (game_constants::LAST_ROWCOL_INDEX, _) |
            (_, 0) | (_, game_constants::LAST_ROWCOL_INDEX) => {
                if let ImageState::Third = self.state {
                    self.triggered = true;
                }
            }
            (_, _) => {
                if let ImageState::Fourth = self.state {
                    self.triggered = true;
                }
            }
        }
    }
}

pub struct Grid {
    grid: Vec<Vec<Cell>>
}

impl Grid {
    pub fn create(ctx: &mut Context) -> GameResult<Grid> {
        let mut grid:Vec<Vec<Cell>>= Vec::new();
        for i in 0..game_constants::TABLE_ROWS {
            grid.push(Vec::new());
            for _j in 0..game_constants::TABLE_COLUMNS {
                grid[i].push(Cell::new(ctx)?);
            }
        }
        Ok(Grid{grid})
    }

    pub fn update(&mut self ) {
        for row in 0..game_constants::TABLE_ROWS{
            for column in 0..game_constants::TABLE_COLUMNS {
                self.grid[row][column].update();
            }
        } 
    }

    pub fn draw_grid(&mut self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult<()> {
        for row in 0..game_constants::TABLE_ROWS{
            for column in 0..game_constants::TABLE_COLUMNS {
                let pos_x = (row  as f32) * game_constants::CELL_WIDTH + game_constants::MARGIN;
                let pos_y = (column as f32) * game_constants::CELL_HEIGHT + game_constants::MARGIN;
                
                let _ = self.grid[row][column].draw_cell(canvas, ctx, Point2{x: pos_x, y: pos_y});
            }
        }
        Ok(())
    }

    pub fn change_cell_state(&mut self, row: usize, column: usize, players: &Player) -> bool{
        let pos_x = (row  as f32) * game_constants::CELL_WIDTH + game_constants::MARGIN;
        let pos_y = (column as f32) * game_constants::CELL_HEIGHT + game_constants::MARGIN;
        let mut new_player = players.clone();
        new_player.set_pos(Point2{x: pos_x + 38.0, y: pos_y + 38.0});
        let current_state = &self.grid[row][column].state;
        match current_state {
            ImageState::Default => {
                new_player.set_rotation(0.775); 
                new_player.set_dir(Direction::DOWN);
                new_player.set_go_to_pos(Point2
                    {x: new_player.get_position().x, 
                    y: new_player.get_position().y - game_constants::CELL_HEIGHT})
            },
            ImageState::First => {
                new_player.set_rotation(0.775 + 1.57); 
                new_player.set_dir(Direction::LEFT);
                new_player.set_go_to_pos(Point2
                    {x: new_player.get_position().x - game_constants::CELL_WIDTH, 
                    y: new_player.get_position().y})
            },
            ImageState::Second => {
                new_player.set_rotation(0.775 + 3.14); 
                new_player.set_dir(Direction::UP);
                new_player.set_go_to_pos(Point2
                    {x: new_player.get_position().x, 
                    y: new_player.get_position().y + game_constants::CELL_HEIGHT})
            },
            ImageState::Third => {
                new_player.set_rotation(0.775 + 4.71); 
                new_player.set_dir(Direction::RIGHT);
                new_player.set_go_to_pos(Point2
                    {x: new_player.get_position().x + game_constants::CELL_WIDTH, 
                    y: new_player.get_position().y})
            }
            _ => {}
        }
        let success_turn = self.grid[row][column].change_state(new_player);
        if success_turn == true {
            self.grid[row][column].trigger(row, column);
        }
        success_turn
    }
}