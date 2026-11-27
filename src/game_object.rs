use macroquad::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum GameObjectKind {
    Rock = 2, 
    Tree = 3, 
}
pub const SPRITE_SIZE: f32 = 32.0;

#[derive(Debug)]
pub struct GameObject {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub scale: f32,            
    pub z_layer: u32,
    pub object_kind: GameObjectKind,
}

impl GameObject {
    pub fn size(&self) -> Vec2 {
        Vec2::splat(SPRITE_SIZE * self.scale)
    }

    pub fn bounding_box(&self) -> Rect {
        let s = self.size();
        Rect::new(self.pos.x, self.pos.y, s.x, s.y)
    }

    pub fn render(&self, sprite_sheet: &Texture2D) {
        let index = self.object_kind as u32 as f32;
        let source = Rect::new(index * SPRITE_SIZE, 0.0, SPRITE_SIZE, SPRITE_SIZE);

        draw_texture_ex(
            sprite_sheet,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(source),
                dest_size: Some(self.size()),
                ..Default::default()
            },
        );
    }
}
