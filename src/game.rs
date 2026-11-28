use macroquad::prelude::*;
use anyhow::Result;
use log::{info, warn};
use crate::player::{Player};
use crate::game_config;
use crate::map_gen::*;
use crate::util;
use crate::game_object::*;
use std::rc::Rc;
use std::cell::RefCell;
use crate::game_config::Assets;


pub struct Game {
    pub render_target: RenderTarget,
    pub camera: Camera2D,
    pub player: Player,
    pub objects: Vec<Rc<RefCell<GameObject>>>,
    pub config: game_config::GameConfig,
    pub assets: LoadedAssets,
    pub map: GameMap,
}

pub struct LoadedAssets {
    pub player_sprite: Texture2D,
    pub terrain_sprites: Texture2D,
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

        self.player.game_object.velocity = dir.normalize_or_zero() * self.player.speed;
        move_and_collide(&mut self.player.game_object, &self.objects, dt);

        self.camera.target = (self.player.game_object.pos + Vec2::new(16.0, 16.0)).round();
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

        let cell_size = self.config.cell_size;
        let hovered = (mouse_world / cell_size).floor();
        for (y, row) in self.map.cells.iter().enumerate() {
            for (x, _cell) in row.iter().enumerate() {
                if x as f32 == hovered.x && y as f32 == hovered.y {
                    draw_rectangle(x as f32 * cell_size, y as f32 * cell_size, cell_size, cell_size,
                        Color::new(1.0, 0.0, 0.0, 0.4));
                }
            }
        }
        let terrain_sprites = &self.assets.terrain_sprites;
        self.objects.iter().for_each(|obj_ref| obj_ref.borrow().render(terrain_sprites));
        
        if self.config.draw_colliders {
            self.player.game_object.draw_collider();
        }
        self.objects.iter().for_each(|obj_ref| obj_ref.borrow().draw_collider());

        let pos = self.player.game_object.pos;
        draw_text(&format!("player pos x: {}, y: {}", pos.x, pos.y), pos.x.round(), pos.y.round(), 12.0, RED);
        let cell = (mouse_world / cell_size).floor();
        draw_text(
            &format!(
                "cell {},{}  map {}x{}  cells {}x{}",
                cell.x, cell.y,
                self.config.map_width, self.config.map_height,
                self.map.cells.first().map_or(0, |r| r.len()),
                self.map.cells.len()
            ),
            mouse_world.x + 8.0,
            mouse_world.y - 8.0,
            12.0,
            WHITE,
        );
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
        info!("loading render target");
        let render_target = render_target(config.virtual_dim.width as u32, config.virtual_dim.height as u32);
        render_target.texture.set_filter(FilterMode::Nearest);

        let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, config.virtual_dim.width, config.virtual_dim.height));
        camera.render_target = Some(render_target.clone());

        let loaded_assets = config.assets.load_all().await?;
        let cell_size = config.cell_size;

        let mut player_object = GameObject::new(
            GameObjectKind::Player,
            vec2(config.virtual_dim.width / 2.0, config.virtual_dim.height / 2.0),
        );
        player_object.set_collider(20.0, 20.0);
        player_object.collider_offset = vec2(6.0, 10.0);
        player_object.collider_dim = (cell_size, cell_size);

        let player = Player {
            game_object: player_object,
            texture: loaded_assets.player_sprite.weak_clone(),
            speed: 100.0,
        };

        let game_map = GameMap {
            cells: GameMap::generate_grid(config.map_width, config.map_height),
            cell_size: config.cell_size,
            sprite_sheet: loaded_assets.terrain_sprites.weak_clone(),
        };

        let terrain = GameMap::populate_terrain(&config);
        //terrain.iter().for_each(|o| info!("obj: {:#?}", o.borrow()));
        let mut game = Game {
            player,
            camera,
            render_target,
            config,
            map: game_map,
            objects: terrain,
            assets: loaded_assets,
        };
        info!("starting game loop");
        loop {
            game.update().await;
            game.render().await;
        }
    }
}
