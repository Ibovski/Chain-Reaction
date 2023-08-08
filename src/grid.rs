use ggez::{graphics::{Mesh, DrawMode, Rect, Color, self, DrawParam, Image}, Context, GameResult, mint::{Point2, Vector2}};

use crate::game_constants;

enum ImageState {
    Default,
    First,
    Second,
    Third,
    Fourth
}

struct Cell {
    cell_width: f32,
    cell_height: f32,
    cell_mesh: Mesh,
    state: ImageState,
    rotation: f32,
}

impl Cell {
    fn new(ctx: &mut Context) -> GameResult<Cell> {
        
        let cell_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(1.0),
            Rect::new(0.0,0.0,game_constants::CELL_WIDTH, game_constants::CELL_HEIGHT),
            Color::BLACK,
        )?;
        Ok(Cell{cell_width: game_constants::CELL_WIDTH, 
            cell_height: game_constants::CELL_HEIGHT, 
            cell_mesh, 
            state: ImageState::Default,
            rotation: 0.0})
    }

    fn draw_cell(&mut self, canvas: &mut graphics::Canvas, ctx: &mut Context, dst: Point2<f32>, rotation: f32) -> GameResult<()>{
        let image = graphics::Image::from_path(ctx, 
            "/pixil-frame-1.png")?;
        if let ImageState::First = self.state {
                self.rotation += f32::abs(rotation);
                canvas.draw(&image, DrawParam::default().dest(Point2{x: dst.x + 38.0, y: dst.y + 38.0}).rotation(self.rotation).scale(Vector2{x: 0.6, y: 0.6}));
        } else if let ImageState::Second = self.state {
            self.rotation += f32::abs(rotation);
            canvas.draw(&image, DrawParam::default().dest(Point2{x: dst.x + 38.0, y: dst.y + 38.0}).rotation(self.rotation - 1.57).scale(Vector2{x: 0.6, y: 0.6}));
            canvas.draw(&image, DrawParam::default().dest(Point2{x: dst.x + 38.0, y: dst.y + 38.0}).rotation(self.rotation).scale(Vector2{x: 0.6, y: 0.6}));
        } else if let ImageState::Third = self.state {
            self.rotation += f32::abs(rotation);
            canvas.draw(&image, DrawParam::default().dest(Point2{x: dst.x + 38.0, y: dst.y + 38.0}).rotation(self.rotation - 3.14).scale(Vector2{x: 0.6, y: 0.6}));
            canvas.draw(&image, DrawParam::default().dest(Point2{x: dst.x + 38.0, y: dst.y + 38.0}).rotation(self.rotation - 1.57).scale(Vector2{x: 0.6, y: 0.6}));
            canvas.draw(&image, DrawParam::default().dest(Point2{x: dst.x + 38.0, y: dst.y + 38.0}).rotation(self.rotation).scale(Vector2{x: 0.6, y: 0.6}));
        } else if let ImageState::Fourth = self.state {
            self.rotation += f32::abs(rotation);
            canvas.draw(&image, DrawParam::default().dest(Point2{x: dst.x + 38.0, y: dst.y + 38.0}).rotation(self.rotation - 4.71).scale(Vector2{x: 0.6, y: 0.6}));
            canvas.draw(&image, DrawParam::default().dest(Point2{x: dst.x + 38.0, y: dst.y + 38.0}).rotation(self.rotation - 3.14).scale(Vector2{x: 0.6, y: 0.6}));
            canvas.draw(&image, DrawParam::default().dest(Point2{x: dst.x + 38.0, y: dst.y + 38.0}).rotation(self.rotation - 1.57).scale(Vector2{x: 0.6, y: 0.6}));
            canvas.draw(&image, DrawParam::default().dest(Point2{x: dst.x + 38.0, y: dst.y + 38.0}).rotation(self.rotation).scale(Vector2{x: 0.6, y: 0.6}));
        }
        canvas.draw(&self.cell_mesh, DrawParam::default().dest(dst));
        Ok(())
    }

    fn change_state(&mut self) {
        match self.state {
            ImageState::Default => self.state = ImageState::First,
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

    pub fn draw_grid(&mut self, canvas: &mut graphics::Canvas, ctx: &mut Context, rotation: f32) -> GameResult<()> {
        for row in 0..game_constants::TABLE_ROWS{
            for column in 0..game_constants::TABLE_COLUMNS {
                let pos_x = (row  as f32) * game_constants::CELL_WIDTH + game_constants::MARGIN;
                let pos_y = (column as f32) * game_constants::CELL_HEIGHT + game_constants::MARGIN;
                
                let _ = self.grid[row][column].draw_cell(canvas, ctx, Point2{x: pos_x, y: pos_y}, rotation);
            }
        }
        Ok(())
    }

    pub fn change_cell_state(&mut self, row: usize, column: usize) {
        self.grid[row][column].change_state();
    }
}