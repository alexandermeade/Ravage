use macroquad::prelude::*;
use anyhow::Result;
use log::{info, warn};
use crate::player::{Player};
use crate::game_config;
use crate::map_gen::*;
use crate::util;

pub struct Game {
    pub render_target: RenderTarget,
    pub camera: Camera2D,
    pub player: Player,
    pub config: game_config::GameConfig,
    pub map: GameMap,
}

struct Viewport {
    scale: f32,
    offset: Vec2,
}

impl Viewport {
    fn current(config: &game_config::GameConfig) -> Self {
        let scale = (screen_width() / config.virtual_dim.width)
            .max(screen_height() / config.virtual_dim.height);
        let dest = vec2(config.virtual_dim.width, config.virtual_dim.height) * scale;
        let offset = (vec2(screen_width(), screen_height()) - dest) / 2.0;
        Self { scale, offset }
    }
    fn dest_size(&self, config: &game_config::GameConfig) -> Vec2 {
        vec2(config.virtual_dim.width, config.virtual_dim.height) * self.scale
    }

    fn screen_to_virtual(&self, p: Vec2) -> Vec2 {
        (p - self.offset) / self.scale
    }
}


impl Game {
    async fn update(&mut self) {

        let dt = get_frame_time();

        let mut dir = Vec2::ZERO;
        if is_key_down(KeyCode::W) {
            dir.y -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            dir.y += 1.0;
        }
        if is_key_down(KeyCode::A) {
            dir.x -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            dir.x += 1.0;
        }

        if dir != Vec2::ZERO {
            self.player.pos += dir.normalize() * self.player.speed * dt;
        }
        let vp = Viewport::current(&self.config);
        self.camera.target = self.player.pos.round();
    }

    async fn render(&mut self) {

        let vp = Viewport::current(&self.config);

        set_camera(&self.camera);
        clear_background(DARKGRAY);
        self.map.render();
        self.player.render();

        let half = vec2(self.config.virtual_dim.width, self.config.virtual_dim.height) / 2.0;
        let mouse_world = vp.screen_to_virtual(mouse_position().into()) + self.camera.target - half;
        draw_circle(mouse_world.x, mouse_world.y, 2.0, RED);

        set_default_camera();
        clear_background(BLACK);

        draw_texture_ex(
            &self.render_target.texture,
            vp.offset.x,
            vp.offset.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vp.dest_size(&self.config)),
                flip_y: true,
                ..Default::default()
            },
        );

        next_frame().await;
    }    
    
    pub async fn start(config: game_config::GameConfig) -> Result<()> { 
        let player_texture = util::load_texture_near(&config.player_sprite).await?; 
        let terrain_texture = util::load_texture_near(&config.terrain_sprite).await?;

        info!("loading render target");
        let render_target = render_target(config.virtual_dim.width as u32, config.virtual_dim.height as u32);
        render_target.texture.set_filter(FilterMode::Nearest);
        
        let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, config.virtual_dim.width, config.virtual_dim.height));
        camera.render_target = Some(render_target.clone());
        
        let player = Player {
            pos: vec2(config.virtual_dim.width / 2.0, config.virtual_dim.height / 2.0),
            texture: player_texture,
            speed: 100.0,
        };

        let game_map = GameMap {
            cells: GameMap::generate_grid(config.map_width, config.map_height),
            sprite_sheet: terrain_texture,
        };

        let mut game = Game {
            player, 
            camera,
            render_target,
            config, 
            map: game_map,
        };
        info!("starting game loop");
        loop {
            game.update().await;
            game.render().await;
        }
        Ok(())
    }
}
