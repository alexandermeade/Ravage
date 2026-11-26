use macroquad::prelude::*;
use std::fs::*;
use std::io::*;
use serde_json;
use anyhow::Result;
mod player;
mod game; 
mod game_config;

#[macroquad::main("Top Down Movement")]
async fn main() -> Result<()> {
    let file = File::open("./config/game_config.json")?;
    let reader = BufReader::new(file);
    let config:game_config::GameConfig = serde_json::from_reader(reader)?;
    game::Game::start(config).await;
    Ok(())
}
