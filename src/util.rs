
use anyhow::Result; 
use macroquad::prelude::*;
use log::{info};

pub async fn load_texture_near(path: &str) -> Result<Texture2D> {
    let texture = load_texture(path).await?;
    texture.set_filter(FilterMode::Nearest);
    info!("loaded texture: {}", path);
    Ok(texture)
}
