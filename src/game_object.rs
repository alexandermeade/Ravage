use macroquad::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum GameObjectKind {
    Player = 0,
    Rock = 2,
    Tree = 3,
}

pub const SPRITE_SIZE: f32 = 32.0;

#[derive(Debug)]
pub struct GameObject {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub collider_dim: (f32, f32),
    pub collider_offset: Vec2,
    pub solid: bool,
    pub z_layer: u32,
    pub object_kind: GameObjectKind,
}

impl GameObject {
    pub fn new(kind: GameObjectKind, pos: Vec2) -> Self {
        Self {
            pos,
            velocity: Vec2::ZERO,
            collider_dim: (SPRITE_SIZE, SPRITE_SIZE),
            collider_offset: Vec2::ZERO,
            solid: true,
            z_layer: 0,
            object_kind: kind,
        }
    }

    pub fn collider(&self) -> Rect {
        Rect::new(
            self.pos.x + self.collider_offset.x,
            self.pos.y + self.collider_offset.y,
            self.collider_dim.0,
            self.collider_dim.1,
        )
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
                dest_size: Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
                ..Default::default()
            },
        );
    }

    pub fn set_collider(&mut self, w: f32, h: f32) {
        self.collider_dim = (w, h);
        self.collider_offset = vec2((SPRITE_SIZE - w) / 2.0, (SPRITE_SIZE - h) / 2.0);
    }

    pub fn draw_collider(&self) {
        let c = self.collider();
        draw_rectangle_lines(c.x, c.y, c.w, c.h, 2.0, RED);
    }
}

// This function basically exists as a port of Macroquad's Rect::overlaps
// function but with some give to make collisions smoother
fn overlaps(a: &Rect, b: &Rect) -> bool {
    const EPS: f32 = 0.01;
    a.left() < b.right() - EPS
        && a.right() > b.left() + EPS
        && a.top() < b.bottom() - EPS
        && a.bottom() > b.top() + EPS
}

pub fn move_and_collide(mover: &mut GameObject, others: &[Rc<RefCell<GameObject>>], dt: f32) {
    let delta = mover.velocity * dt;

    let walls: Vec<Rect> = if mover.solid {
        others
            .iter()
            .map(|o| o.borrow())
            .filter(|o| o.solid)
            .map(|o| o.collider())
            .collect()
    } else {
        Vec::new()
    };

    mover.pos.x += delta.x;
    for wall in &walls {
        if overlaps(&mover.collider(), wall) {
            if delta.x > 0.0 {
                mover.pos.x = wall.left() - mover.collider_dim.0 - mover.collider_offset.x;
            } else if delta.x < 0.0 {
                mover.pos.x = wall.right() - mover.collider_offset.x;
            }
            mover.velocity.x = 0.0;
        }
    }

    mover.pos.y += delta.y;
    for wall in &walls {
        if overlaps(&mover.collider(), wall) {
            if delta.y > 0.0 {
                mover.pos.y = wall.top() - mover.collider_dim.1 - mover.collider_offset.y;
            } else if delta.y < 0.0 {
                mover.pos.y = wall.bottom() - mover.collider_offset.y;
            }
            mover.velocity.y = 0.0;
        }
    }
}
