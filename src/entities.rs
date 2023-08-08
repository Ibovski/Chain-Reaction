use ggez::{graphics::{Image, Transform}, Context, GameResult};

#[derive(Clone)]
pub struct Player {
    texture: Image,
    rotation: f32,
    transform: Transform

}

impl Player {
    pub fn from_path(path: String, ctx: &mut Context) -> GameResult<Player> {
        let image = Image::from_path(ctx, 
            path)?;
            Ok(Player{texture: image, rotation: 0.0, transform: Transform::default()})  
    }
    
    pub fn get_texture(&self) -> &Image {
        &self.texture
    }
    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }
    pub fn get_transform(&self) -> Transform {
        self.transform
    }
    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }
}