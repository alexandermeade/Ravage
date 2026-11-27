
use macroquad::prelude::*;

use macroquad::rand::gen_range;
use crate::game::Game;
use crate::game_config::GameConfig;
use std::cell::RefCell;
use std::rc::Rc;
use crate::game_object::GameObject;
use crate::game_object::GameObjectKind;

const TILE_WIDTH: f32 = 32.0;
const TILE_HEIGHT: f32 = 32.0;

pub struct GameMap {
    pub cells: Vec<Vec<MapTile>>,
    pub sprite_sheet: Texture2D
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
        let target: usize = 10;
        
        let tile_size = 32.0;
        let target: usize = 50;

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

            result.push(Rc::new(RefCell::new(GameObject {
                pos: tile_pos * tile_size,   
                velocity: Vec2::ZERO,
                z_layer: 0,
                scale: rand::gen_range(0.75, 1.5),
                object_kind: if rand::gen_range(0, 100) < 50 {
                    GameObjectKind::Rock
                } else {
                    GameObjectKind::Tree
                },
            })));
        }
        result        
    }
    pub fn render(&self) {
        for (i, tiles) in self.cells.iter().enumerate() {
                for (j, tile) in tiles.iter().enumerate() {
                let source_rect = Rect::new((*tile as u32) as f32 * TILE_WIDTH, 0.0, TILE_WIDTH, TILE_HEIGHT);

                draw_texture_ex(
                    &self.sprite_sheet,
                    TILE_WIDTH * (i as f32),
                    TILE_HEIGHT * (j as f32),
                    WHITE,
                    DrawTextureParams {
                        source: Some(source_rect),
                        dest_size: Some(vec2(TILE_WIDTH, TILE_HEIGHT )),
                        ..Default::default()
                    },
                );
            }
        } 
    }
}
