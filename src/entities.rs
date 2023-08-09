use ggez::{graphics::{Image}, Context, GameResult, mint::Point2};

use crate::{game_constants::{self}, helper};

static mut PLAYERS_ID: i32 = -1;

#[derive(Clone)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Clone)]
pub struct Player {
    texture: Image,
    rotation: f32,
    pos: Point2<f32>,
    go_to_pos: Point2<f32>,
    speed: f32,
    dir: Direction,
    id: i32
}

impl Player {
    pub fn from_path(path: String, ctx: &mut Context) -> GameResult<Player> {
        let image = Image::from_path(ctx, 
            path)?;
            unsafe { PLAYERS_ID += 1 };
            Ok(Player{texture: image, rotation: 0.775, 
                pos: Point2{x: 0.0, y: 0.0},
                go_to_pos: Point2{x: 0.0, y: 0.0},
                speed: 0.0,
                dir: Direction::UP,
                id: unsafe { PLAYERS_ID }})  
    }

    pub fn update(&mut self, triggered: &mut bool) {
        match self.dir {
            Direction::LEFT => self.pos.x += game_constants::PLAYER_SPEED,
            Direction::RIGHT => self.pos.x -= game_constants::PLAYER_SPEED,
            Direction::UP => self.pos.y += game_constants::PLAYER_SPEED,
            Direction::DOWN => self.pos.y -= game_constants::PLAYER_SPEED,
        }
       
         if helper::approximately_eq(self.pos.x, self.go_to_pos.x, 1.0) && 
         helper::approximately_eq(self.pos.y, self.go_to_pos.y, 1.0)  {
            self.speed = 0.0;
            *triggered = false;
        }
        
    }

    pub fn get_texture(&self) -> &Image {
        &self.texture
    }
    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }
    pub fn get_position(&self) -> Point2<f32> {
        self.pos
    }
    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }
    pub fn set_pos(&mut self, pos: Point2<f32>) {
        self.pos = pos;
    }
    pub fn set_dir(&mut self, dir: Direction) {
        self.dir = dir;
    }
    pub fn set_go_to_pos(&mut self, go_to_pos: Point2<f32>) {
        self.go_to_pos = go_to_pos;
    }
}