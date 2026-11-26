use serde::Deserialize;
use crate::*;

#[derive(Deserialize)]
pub struct GameConfig {
    pub player_sprite: String,
    pub virtual_dim: VirtualDim,
}

#[derive(Deserialize)]
pub struct VirtualDim {
    pub width: f32, 
    pub height: f32,
}
