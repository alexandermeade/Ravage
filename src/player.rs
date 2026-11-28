use macroquad::texture::Texture2D;
use macroquad::prelude::*;

use crate::game_object::GameObject;

pub struct Player {
    pub game_object: GameObject,
    pub texture: Texture2D,
    pub speed: f32,
}

impl Player {
    pub fn render(&self) {
        draw_texture_ex(
            &self.texture,
            self.game_object.pos.x.round(), 
            self.game_object.pos.y.round(),
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(32.0, 32.0)),
                ..Default::default()
            },
        );
    }
}


