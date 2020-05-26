mod config;
mod render;
mod tangy;

use tangy::{Direction, Ticks, World, GRID_HEIGHT, GRID_WIDTH};

use config::{GameConfig, WorldConfig};

use sweet_turnips;
use sweet_turnips::event;
use sweet_turnips::event::{KeyCode, KeyMods};
use sweet_turnips::midi::{connect_to_midi, MidiReceiver};
use sweet_turnips::sprites::SpriteContext;
use sweet_turnips::AppConfig;
use sweet_turnips::{Context, GameResult};

use std::convert::From;
use std::sync::mpsc;

const GAME_NAME: &str = "tangy-turnips";
const AUTHOR_NAME: &str = "JC Holder";

struct MainState {
    world: World,
    sprite_context: SpriteContext,
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
        let sprite_context = SpriteContext::new(ctx)?;

        let ticks: Ticks = 0;

        let s = MainState {
            world: game_config.world.into(),
            sprite_context,
            ticks,
            rx,
            tick_speed: 20,
        };
        Ok(s)
    }

    fn move_player_ship(&mut self, direction: Direction) {
        self.world = self.world.with_player_ship_move_requested(direction);
    }

    fn fire_bullets(&mut self) {
        self.world = self.world.with_player_bullets_fired();
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
            KeyCode::Space => self.fire_bullets(),
            _ => (),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let sprite_grid = render::sprite_grid_from_world(&self.world);

        self.sprite_context.render_sprite_grid(sprite_grid, ctx)?;

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
    let app_config = AppConfig::new((GRID_WIDTH, GRID_HEIGHT))
        .game_name(GAME_NAME)
        .author_name(AUTHOR_NAME);

    let config_path = sweet_turnips::prep_config_path(&app_config)?;

    let game_config = config::setup_game_config(config_path);

    let (ctx, event_loop) = &mut sweet_turnips::build_context_and_event_loop(&app_config)?;

    let (tx, rx) = mpsc::channel();

    connect_to_midi(tx);

    let state = &mut MainState::new(ctx, game_config, Some(rx))?;

    event::run(ctx, event_loop, state)
}
