use macroquad::prelude::*;

use crate::game_config::GameConfig;
use crate::game_object::GameObject;
use crate::game_object::GameObjectKind;
use std::cell::RefCell;
use std::rc::Rc;

pub struct GameMap {
    pub cells: Vec<Vec<MapTile>>,
    pub cell_size: f32,
    pub sprite_sheet: Texture2D,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum MapTile {
    Bush = 0,
    Grass = 1,
    Rock = 2,
    PlowedGrass = 3,
}

impl GameMap {
    pub fn generate_grid(width: u32, height: u32) -> Vec<Vec<MapTile>> {
        vec![vec![MapTile::Grass; width as usize]; height as usize]
    }

    pub fn populate_terrain(config: &GameConfig) -> Vec<Rc<RefCell<GameObject>>> {
        let tile_size = config.cell_size;
        let target: usize = 15;

        let min_dist = ((config.map_width * config.map_height) as f32 / target as f32).sqrt() * 0.7;

        let mut points: Vec<Vec2> = Vec::new();
        let mut result = Vec::new();
        let mut attempts = 0;

        while result.len() < target && attempts < target * 100 {
            attempts += 1;

            let tile_pos = vec2(
                rand::gen_range(0, config.map_width) as f32,
                rand::gen_range(0, config.map_height) as f32,
            );

            if points.iter().any(|p| p.distance(tile_pos) < min_dist) {
                continue;
            }
            points.push(tile_pos);

            let kind = if rand::gen_range(0, 100) < 50 {
                GameObjectKind::Rock
            } else {
                GameObjectKind::Tree
            };

            let mut object = GameObject::new(kind, tile_pos * tile_size);
            match kind {
                GameObjectKind::Rock => object.set_collider(28.0, 28.0),
                _ => object.set_collider(20.0, 20.0),
            }
            result.push(Rc::new(RefCell::new(object)));
        }
        result
    }

    pub fn render(&self) {
        for (i, tiles) in self.cells.iter().enumerate() {
            for (j, tile) in tiles.iter().enumerate() {
                let source_rect = Rect::new(
                    (*tile as u32) as f32 * self.cell_size,
                    0.0,
                    self.cell_size,
                    self.cell_size,
                );

                draw_texture_ex(
                    &self.sprite_sheet,
                    self.cell_size * (i as f32),
                    self.cell_size * (j as f32),
                    WHITE,
                    DrawTextureParams {
                        source: Some(source_rect),
                        dest_size: Some(vec2(32.0, 32.0)),
                        ..Default::default()
                    },
                );
            }
        }
    }
}
