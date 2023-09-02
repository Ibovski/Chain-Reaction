use ggez::{graphics::Image, Context, GameResult, mint::Point2};
use crate::{game_constants::{self}, helper::{self, image_from_path}};

/// Represents the player's ID, starting from 0
static mut PLAYERS_ID: i32 = -1;

/// Represents a direction in which a player can move.
#[derive(Clone, Debug)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

/// Represents a player in the game.
#[derive(Clone)]
pub struct Player {
    texture: Image,
    rotation: f32,
    pos: Point2<f32>,
    go_to_pos: Point2<f32>,
    speed: f32,
    dir: Direction,
    id: i32,
}

impl Player {
    /// Creates a new player from an image file located at the specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the image file representing the player.
    /// * `ctx` - A mutable reference to the game context.
    ///
    /// # Returns
    ///
    /// A `Result` containing the created `Player` or a `GameError` if there was an issue loading the image.
    pub fn from_path(path: String, ctx: &mut Context) -> GameResult<Player> {
        let image = image_from_path(ctx, path)?;
        unsafe { PLAYERS_ID += 1 };
        Ok(Player {
            texture: image,
            rotation: 0.0,
            pos: Point2 { x: 0.0, y: 0.0 },
            go_to_pos: Point2 { x: 0.0, y: 0.0 },
            speed: 0.0,
            dir: Direction::UP,
            id: unsafe { PLAYERS_ID },
        })
    }

    /// Updates the player's position based on its current direction and speed.
    ///
    /// # Arguments
    ///
    /// * `triggered` - A mutable reference to a boolean flag indicating if the player is triggered.
    pub fn update(&mut self, triggered: &mut bool) {
        match self.dir {
            Direction::LEFT => self.pos.x += game_constants::PLAYER_SPEED,
            Direction::RIGHT => self.pos.x -= game_constants::PLAYER_SPEED,
            Direction::UP => self.pos.y -= game_constants::PLAYER_SPEED,
            Direction::DOWN => self.pos.y += game_constants::PLAYER_SPEED,
        }

        if helper::approximately_eq(self.pos.x, self.go_to_pos.x, 1.0)
            && helper::approximately_eq(self.pos.y, self.go_to_pos.y, 1.0)
        {
            self.speed = 0.0;
            *triggered = false;
        }
    }

    /// Retrieves a reference to the player's texture (image).
    ///
    /// # Returns
    ///
    /// A reference to the player's texture.
    pub fn get_texture(&self) -> &Image {
        &self.texture
    }

    /// Retrieves the player's rotation angle.
    ///
    /// # Returns
    ///
    /// The rotation angle of the player.
    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    /// Retrieves the player's current position.
    ///
    /// # Returns
    ///
    /// The current position of the player as a `Point2<f32>`.
    pub fn get_position(&self) -> Point2<f32> {
        self.pos
    }

    /// Retrieves the unique identifier (ID) of the player.
    ///
    /// # Returns
    ///
    /// The ID of the player as an integer.
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Sets the player's rotation angle.
    ///
    /// # Arguments
    ///
    /// * `rotation` - The new rotation angle for the player.
    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    /// Sets the player's movement speed.
    ///
    /// # Arguments
    ///
    /// * `speed` - The new movement speed for the player.
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    /// Sets the player's position.
    ///
    /// # Arguments
    ///
    /// * `pos` - The new position for the player as a `Point2<f32>`.
    pub fn set_pos(&mut self, pos: Point2<f32>) {
        self.pos = pos;
    }

    /// Sets the player's direction of movement.
    ///
    /// # Arguments
    ///
    /// * `dir` - The new direction of movement for the player.
    pub fn set_dir(&mut self, dir: Direction) {
        self.dir = dir;
    }

    /// Sets the destination position for the player to move to.
    ///
    /// # Arguments
    ///
    /// * `go_to_pos` - The new destination position as a `Point2<f32>`.
    pub fn set_go_to_pos(&mut self, go_to_pos: Point2<f32>) {
        self.go_to_pos = go_to_pos;
    }
}
