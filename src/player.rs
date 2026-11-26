use macroquad::texture::Texture2D;
use macroquad::prelude::*;

pub struct Player {
    pub pos: Vec2,
    pub texture: Texture2D,
    pub speed: f32,
}

impl Player {
    pub fn render(&self) {
        draw_texture_ex(
            &self.texture,
            self.pos.x.round(), 
            self.pos.y.round(),
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(32.0, 32.0)),
                ..Default::default()
            },
        );
    }
}


