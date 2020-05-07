mod bitter;
mod config;
mod renderer;

use ggez;
use ggez::event;
use ggez::event::{KeyCode, KeyMods};
use ggez::{Context, GameResult};

use bitter::{Coords, Direction, EntityKey, Ticks, World, GRID_HEIGHT, GRID_WIDTH};

use config::{GameConfig, WorldConfig};

use renderer::WorldRenderer;

use sweet_turnips::sprites::{Sprites, SpriteGridRenderer};

use std::convert::From;
use std::path;

const SPRITE_SCALE: f32 = 4.0;
const SPRITE_SIZE: f32 = 8.0 * SPRITE_SCALE;

#[derive(Copy, Clone)]
struct Cursor {
    x: u8,
    y: u8,
}

impl Cursor {
    fn new() -> Self {
        Self { x: 2, y: 2 }
    }
}

struct MainState {
    world: World,
    sprites: Sprites,
    cursor: Cursor,
    selected_villager_key: Option<EntityKey>,
    ticks: Ticks,
}

impl MainState {
    fn new(ctx: &mut Context, game_config: GameConfig) -> GameResult<MainState> {
        let sprites = Sprites::new(ctx)?;

        let ticks: Ticks = 0;

        let s = MainState {
            world: game_config.world.into(),
            sprites,
            cursor: Cursor::new(),
            selected_villager_key: None,
            ticks,
        };
        Ok(s)
    }

    fn move_cursor(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                if self.cursor.y > 1 {
                    self.cursor.y -= 1
                }
            }
            Direction::Down => {
                if self.cursor.y < GRID_HEIGHT - 2 {
                    self.cursor.y += 1
                }
            }
            Direction::Left => {
                if self.cursor.x > 1 {
                    self.cursor.x -= 1
                }
            }
            Direction::Right => {
                if self.cursor.x < GRID_WIDTH - 2 {
                    self.cursor.x += 1
                }
            }
        }
    }

    fn spawn_egg(&mut self, coords: Coords) {
        self.world = self.world.with_egg_spawn_requested_at(coords);
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.ticks += 1;

        if (self.ticks + 1) % 40 == 0 {
            self.world = self.world.ticked();
        }

        self.selected_villager_key = self.world.villager_key_at(self.cursor.x, self.cursor.y);

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
            KeyCode::W => self.move_cursor(Direction::Up),
            KeyCode::A => self.move_cursor(Direction::Left),
            KeyCode::S => self.move_cursor(Direction::Down),
            KeyCode::D => self.move_cursor(Direction::Right),
            KeyCode::Space => self.spawn_egg((self.cursor.x, self.cursor.x)),
            _ => (),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let renderer = WorldRenderer {
            world: self.world.clone(),
            selected_villager_key: self.selected_villager_key,
        };
        let mut sprite_grid = renderer.render_grid();

        sprite_grid.cursor_at(self.cursor.x + 1, self.cursor.y + 1);

        self.sprites.render_sprite_grid(sprite_grid);

        self.sprites.draw_all_sprites(ctx)?;

        Ok(())
    }
}

impl From<WorldConfig> for World {
    fn from(config: WorldConfig) -> Self {
        let mut world = Self::new();

        for v in config.starting_villagers {
            world.add_villager_at(v.x, v.y);
        }

        for f in config.starting_farms {
            world.add_farm_at(f.x, f.y);
        }

        world
    }
}

pub fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./resources");

    let config_path = resource_dir.join("config.yaml");

    let game_config = config::setup_game_config(config_path);

    let cb = ggez::ContextBuilder::new("bitter-turnips", "ggez")
        .add_resource_path(resource_dir)
        .window_mode(ggez::conf::WindowMode::default().dimensions(
            GRID_WIDTH as f32 * SPRITE_SIZE,
            GRID_HEIGHT as f32 * SPRITE_SIZE,
        ));

    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new(ctx, game_config)?;

    event::run(ctx, event_loop, state)
}
