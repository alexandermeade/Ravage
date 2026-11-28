use serde::Deserialize;
use crate::{game::LoadedAssets, *};

#[derive(Deserialize)]
pub struct GameConfig {
    pub assets: Assets,
    pub map_height: u32,
    pub map_width: u32,
    pub virtual_dim: VirtualDim,
    pub cell_size: f32,
    pub draw_colliders: bool,
}

#[derive(Deserialize)]
pub struct Assets {
    pub player_sprite: String, 
    pub terrain_sprite: String,
}

impl Assets {
    pub async fn load_all(&self) -> Result<LoadedAssets> {
        Ok(LoadedAssets { 
            player_sprite: util::load_texture_near(&self.player_sprite).await?,
            terrain_sprites: util::load_texture_near(&self.terrain_sprite).await? 
        })
    }
}

#[derive(Deserialize)]
pub struct VirtualDim {
    pub width: f32, 
    pub height: f32,
}
