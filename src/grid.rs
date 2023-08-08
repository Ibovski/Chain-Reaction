use ggez::{graphics::{Mesh, DrawMode, Rect, Color, self, DrawParam, Image}, Context, GameResult, mint::{Point2, Vector2}};

use crate::{game_constants, entities::Player};

enum ImageState {
    Default,
    First,
    Second,
    Third,
    Fourth
}

impl ImageState {
    fn to_int(&self) -> u32 {
        match self {
            ImageState::Default => 0,
            ImageState::First => 1,
            ImageState::Second => 2,
            ImageState::Third => 3,
            ImageState::Fourth => 4
        }
    }
}

struct Cell {
    cell_mesh: Mesh,
    state: ImageState,
    player: Option<Player>,
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
            player: None})
    }

    fn draw_cell(&mut self, canvas: &mut graphics::Canvas, ctx: &mut Context, dst: Point2<f32>) -> GameResult<()>{
        
        if let ImageState::Default = self.state{}
        else {
                let temp = self.player.as_mut().unwrap();
                let mut new_rotation = ctx.time.delta().as_secs_f32() * 3.0 + temp.get_rotation();
                let image_dst = Point2{x: dst.x + 38.0, y: dst.y +38.0};
                temp.set_rotation(new_rotation);
                for _i in 0..self.state.to_int() {
                    canvas.draw(
                    temp.get_texture(), 
                    DrawParam::default()
                    .dest(image_dst)
                    .rotation(new_rotation)
                    .scale(Vector2{x: 0.6, y: 0.6}));
                    new_rotation -= 1.57;
                }
        }
        canvas.draw(&self.cell_mesh, DrawParam::default().dest(dst));
        Ok(())
    }

    fn change_state(&mut self, player: &Player) {
        
        match &mut self.state {
            ImageState::Default => {self.state = ImageState::First; self.player = Some(player.clone())}
            ImageState::First => self.state = ImageState::Second,
            ImageState::Second => self.state = ImageState::Third,
            ImageState::Third => self.state = ImageState::Fourth,
            ImageState::Fourth => self.state = ImageState::Default,
        }
    }
}

pub struct Grid {
    grid: Vec<Vec<Cell>>
}

impl Grid {
    pub fn create(ctx: &mut Context) -> GameResult<Grid> {
        let mut grid:Vec<Vec<Cell>>= Vec::new();
        for i in 0..8 {
            grid.push(Vec::new());
            for _j in 0..8 {
                grid[i].push(Cell::new(ctx)?);
            }
        }
        Ok(Grid{grid})
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

    pub fn change_cell_state(&mut self, row: usize, column: usize, player: &Player) {
        self.grid[row][column].change_state(player);
    }
}