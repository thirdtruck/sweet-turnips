mod bitter;
mod config;
mod render;

use bitter::{Coords, Direction, EntityKey, Ticks, World, GRID_HEIGHT, GRID_WIDTH};

use config::{GameConfig, WorldConfig};

use sweet_turnips;
use sweet_turnips::event;
use sweet_turnips::event::{KeyCode, KeyMods};
use sweet_turnips::sprites::SpriteContext;
use sweet_turnips::AppConfig;
use sweet_turnips::{Context, GameResult};

use std::convert::From;

const GAME_NAME: &str = "bitter-turnips";
const AUTHOR_NAME: &str = "JC Holder";

struct MainState {
    world: World,
    sprite_context: SpriteContext,
    selected_villager_key: Option<EntityKey>,
    ticks: Ticks,
}

impl MainState {
    fn new(ctx: &mut Context, game_config: GameConfig) -> GameResult<MainState> {
        let sprite_context = SpriteContext::new(ctx)?;

        let ticks: Ticks = 0;

        let s = MainState {
            world: game_config.world.into(),
            sprite_context,
            selected_villager_key: None,
            ticks,
        };
        Ok(s)
    }

    fn move_cursor(&mut self, direction: Direction) {
        self.world = self.world.with_cursor_moved(direction);
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
        } else {
            self.world = self.world.events_processed();
        }

        self.selected_villager_key = self.world.villager_key_at(self.world.cursor_coords());

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
            KeyCode::Space => self.spawn_egg(self.world.cursor_coords()),
            _ => (),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let sprite_grid = render::sprite_grid_from_world(&self.world, self.selected_villager_key);

        self.sprite_context.render_sprite_grid(sprite_grid, ctx)?;

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

        let c = config.starting_cursor;
        world.add_cursor_at((c.x, c.y));

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

    let state = &mut MainState::new(ctx, game_config)?;

    event::run(ctx, event_loop, state)
}
