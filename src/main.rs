use macroquad::prelude::*;
use log::{info, warn, error};
use std::fs::*;
use std::io::*;
use serde_json;
use anyhow::Result;
mod player;
mod game; 
mod game_config;
mod map_gen;
mod util;
mod game_object;

#[macroquad::main("Top Down Movement")]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info")
    ).init();
    let path = "./config/game_config.json";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let config:game_config::GameConfig = serde_json::from_reader(reader)?;
    
    info!("Loaded log file {}", path);
    if let Err(e) = game::Game::start(config).await {
        error!("Game threw error {}:\n\t", e);
    }
    Ok(())
}
