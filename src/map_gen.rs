
use macroquad::prelude::*;

use macroquad::rand::gen_range;
use crate::game::Game;

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
        (0..height)
            .map(|_| {
                (0..width)
                    .map(|_| match gen_range(0, 100) {
                        0..=9 => MapTile::Rock,    // 10%
                        10..=24 => MapTile::Bush,  // 15%
                        _ => MapTile::Grass,       // 75%
                    })
                    .collect()
            })
            .collect()
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
                        dest_size: Some(vec2(TILE_WIDTH* 3.0, TILE_HEIGHT * 3.0)),
                        ..Default::default()
                    },
                );
            }
        } 
    }
}
