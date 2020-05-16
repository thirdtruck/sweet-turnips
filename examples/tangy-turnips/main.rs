mod config;
mod render;
mod tangy;

use tangy::{Direction, Ticks, World, GRID_HEIGHT, GRID_WIDTH};

use config::{GameConfig, WorldConfig};

use sweet_turnips;
use sweet_turnips::event;
use sweet_turnips::event::{KeyCode, KeyMods};
use sweet_turnips::midi::{connect_to_midi, MidiReceiver};
use sweet_turnips::sprites::Sprites;
use sweet_turnips::{Context, GameResult};

use std::convert::From;
use std::fs;
use std::path;
use std::sync::mpsc;

const GAME_NAME: &str = "tangy-turnips";

const SPRITE_SCALE: f32 = 4.0;
const SPRITE_SIZE: f32 = 8.0 * SPRITE_SCALE;

struct MainState {
    world: World,
    sprites: Sprites,
    ticks: Ticks,
    rx: Option<MidiReceiver>,
    tick_speed: usize,
}

impl MainState {
    fn new(
        ctx: &mut Context,
        game_config: GameConfig,
        rx: Option<MidiReceiver>,
    ) -> GameResult<MainState> {
        let sprites = Sprites::new(ctx)?;

        let ticks: Ticks = 0;

        let s = MainState {
            world: game_config.world.into(),
            sprites,
            ticks,
            rx,
            tick_speed: 40,
        };
        Ok(s)
    }

    fn move_player_ship(&mut self, direction: Direction) {
        self.world = self.world.with_player_ship_move_requested(direction);
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let mut tick_speed: usize = self.tick_speed;
        let mut dir: Option<Direction> = None;

        if let Some(receiver) = &self.rx {
            // Mapped to a Korg nanoKONTROL2
            // TODO: Add mapping options to config

            while let Ok(key_value) = receiver.try_recv() {
                let (key, value) = key_value;

                if key == 43 && value == 127 {
                    // rewind
                    dir = Some(Direction::Left);
                }

                if key == 44 && value == 127 {
                    // fast forward
                    dir = Some(Direction::Right);
                }

                if key == 16 {
                    if value > 0 {
                        tick_speed = value as usize;
                    } else {
                        tick_speed = 1;
                    }
                }
            }
        }

        self.tick_speed = tick_speed;
        if let Some(d) = dir {
            self.move_player_ship(d);
        }

        self.ticks += 1;

        if (self.ticks + 1) % self.tick_speed == 0 {
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
    fn from(config: WorldConfig) -> Self {
        let mut world = Self::new();

        let player_ship_coords = (config.starting_player_ship.x, config.starting_player_ship.y);

        world = world.with_player_ship_added_at(player_ship_coords);

        for enemy_config in config.starting_enemy_ships.iter() {
            let coords = (enemy_config.x, enemy_config.y);
            world = world.with_enemy_ship_added_at(coords);
        }

        world
    }
}

pub fn main() -> GameResult {
    let (tx, rx) = mpsc::channel();

    connect_to_midi(tx);

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

    let state = &mut MainState::new(ctx, game_config, Some(rx))?;

    event::run(ctx, event_loop, state)
}
