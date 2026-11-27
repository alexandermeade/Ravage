use serde::Deserialize;
use crate::*;

#[derive(Deserialize)]
pub struct GameConfig {
    pub player_sprite: String,
    pub terrain_sprite: String,
    pub map_height: u32,
    pub map_width: u32,
    pub virtual_dim: VirtualDim,
}

#[derive(Deserialize)]
pub struct VirtualDim {
    pub width: f32, 
    pub height: f32,
}
