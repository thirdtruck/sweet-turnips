mod tangy;
mod config;
mod render;

use tangy::{Direction, Ticks, World, GRID_HEIGHT, GRID_WIDTH};

use config::{GameConfig, WorldConfig};

use sweet_turnips;
use sweet_turnips::{Context, GameResult};
use sweet_turnips::event;
use sweet_turnips::event::{KeyCode, KeyMods};
use sweet_turnips::sprites::{Sprites};

use std::convert::From;
use std::fs;
use std::path;

const GAME_NAME: &str = "tangy-turnips";

const SPRITE_SCALE: f32 = 4.0;
const SPRITE_SIZE: f32 = 8.0 * SPRITE_SCALE;

struct MainState {
    world: World,
    sprites: Sprites,
    ticks: Ticks,
}

impl MainState {
    fn new(ctx: &mut Context, game_config: GameConfig) -> GameResult<MainState> {
        let sprites = Sprites::new(ctx)?;

        let ticks: Ticks = 0;

        let s = MainState {
            world: game_config.world.into(),
            sprites,
            ticks,
        };
        Ok(s)
    }

    fn move_player_ship(&mut self, direction: Direction) {
        self.world = self.world.with_player_ship_move_requested(direction);
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.ticks += 1;

        if (self.ticks + 1) % 40 == 0 {
            self.world = self.world.ticked();
        } else {
            self.world = self.world.with_events_processed();
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Escape => event::quit(ctx),
            KeyCode::W => self.move_player_ship(Direction::Up),
            KeyCode::A => self.move_player_ship(Direction::Left),
            KeyCode::S => self.move_player_ship(Direction::Down),
            KeyCode::D => self.move_player_ship(Direction::Right),
            _ => (),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let sprite_grid = render::sprite_grid_from_world(&self.world);

        self.sprites.render_sprite_grid(sprite_grid);

        self.sprites.draw_all_sprites(ctx)?;

        Ok(())
    }
}

impl From<WorldConfig> for World {
    fn from(_config: WorldConfig) -> Self {
        let world = Self::new().with_player_ship_added_at((2, 5));

        world
    }
}

pub fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./resources");

    let config_dir = resource_dir.join(GAME_NAME);
    let config_path = config_dir.join("config.yaml");

    fs::create_dir_all(config_dir)?;
    let game_config = config::setup_game_config(config_path);

    let cb = sweet_turnips::ContextBuilder::new(GAME_NAME, "JC Holder")
        .add_resource_path(resource_dir)
        .window_mode(sweet_turnips::conf::WindowMode::default().dimensions(
            GRID_WIDTH as f32 * SPRITE_SIZE,
            GRID_HEIGHT as f32 * SPRITE_SIZE,
        ));

    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new(ctx, game_config)?;

    event::run(ctx, event_loop, state)
}
