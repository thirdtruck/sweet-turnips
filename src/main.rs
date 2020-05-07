mod bitter;
mod config;
mod renderer;

use ggez;
use ggez::event;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics;
use ggez::graphics::{Color, DrawParam};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

use bitter::{Coords, Direction, EntityKey, Ticks, World, GRID_HEIGHT, GRID_WIDTH};

use config::{GameConfig, WorldConfig};

use renderer::sprite_grid_from_world;

use sweet_turnips::sprites::{SpriteGrid, SpriteType, Sprites};

use std::convert::From;
use std::path;

const SPRITE_SCALE: f32 = 4.0;
const SPRITE_SIZE: f32 = 8.0 * SPRITE_SCALE;

const RED: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

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

struct GridParam {
    draw_param: DrawParam,
}

impl GridParam {
    fn new() -> Self {
        let draw_param = graphics::DrawParam::new();

        GridParam { draw_param }
    }

    fn at(&self, x: u8, y: u8) -> Self {
        GridParam {
            draw_param: self.draw_param.dest(grid_point(x, y)),
        }
    }

    fn color(&self, color: Color) -> Self {
        GridParam {
            draw_param: self.draw_param.color(color),
        }
    }
}

fn grid_point(x: u8, y: u8) -> na::Point2<f32> {
    let x = x as f32;
    let y = y as f32;
    let segment_size = 8.0;

    na::Point2::new(segment_size * x, segment_size * y)
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

    fn draw_all_spritebatches(&mut self, ctx: &mut Context) -> GameResult {
        let origin_param = graphics::DrawParam::new()
            .dest(na::Point2::new(0.0, 0.0))
            .scale(na::Vector2::new(SPRITE_SCALE, SPRITE_SCALE));

        graphics::draw(ctx, &self.sprites.lines, origin_param)?;
        graphics::draw(ctx, &self.sprites.curves, origin_param)?;
        graphics::draw(ctx, &self.sprites.crosses, origin_param)?;
        graphics::draw(ctx, &self.sprites.corner_triangles, origin_param)?;
        graphics::draw(ctx, &self.sprites.small_circles, origin_param)?;
        graphics::draw(ctx, &self.sprites.big_circles, origin_param)?;
        graphics::draw(ctx, &self.sprites.diamonds, origin_param)?;
        graphics::draw(ctx, &self.sprites.dashes, origin_param)?;
        graphics::draw(ctx, &self.sprites.dots, origin_param)?;
        graphics::draw(ctx, &self.sprites.booms, origin_param)?;
        graphics::draw(ctx, &self.sprites.skulls, origin_param)?;
        graphics::draw(ctx, &self.sprites.side_triangles, origin_param)?;
        graphics::draw(ctx, &self.sprites.ships, origin_param)?;
        graphics::draw(ctx, &self.sprites.hearts, origin_param)?;
        graphics::draw(ctx, &self.sprites.cursors, origin_param)?;
        graphics::draw(ctx, &self.sprites.turnips, origin_param)?;
        graphics::draw(ctx, &self.sprites.squids, origin_param)?;
        graphics::draw(ctx, &self.sprites.lizards, origin_param)?;
        graphics::draw(ctx, &self.sprites.balls, origin_param)?;
        graphics::draw(ctx, &self.sprites.crabs, origin_param)?;
        graphics::draw(ctx, &self.sprites.altars, origin_param)?;

        self.sprites.lines.clear();
        self.sprites.curves.clear();
        self.sprites.crosses.clear();
        self.sprites.corner_triangles.clear();
        self.sprites.small_circles.clear();
        self.sprites.big_circles.clear();
        self.sprites.diamonds.clear();
        self.sprites.dashes.clear();
        self.sprites.dots.clear();
        self.sprites.booms.clear();
        self.sprites.skulls.clear();
        self.sprites.side_triangles.clear();
        self.sprites.ships.clear();
        self.sprites.hearts.clear();
        self.sprites.cursors.clear();
        self.sprites.turnips.clear();
        self.sprites.squids.clear();
        self.sprites.lizards.clear();
        self.sprites.balls.clear();
        self.sprites.crabs.clear();
        self.sprites.altars.clear();

        Ok(())
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

    fn render_sprite_grid(&mut self, sprite_grid: SpriteGrid) {
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                let sprite_type = sprite_grid.sprite_type_at(x, y);
                self.render_sprite_at(sprite_type, x, y);
            }
        }
    }

    fn render_sprite_at(&mut self, sprite_type: SpriteType, x: u8, y: u8) {
        let gp = GridParam::new().at(x, y);

        if sprite_type == SpriteType::Empty {
            return;
        }

        match sprite_type {
            SpriteType::BigCircle => self.sprites.big_circles.add(gp.draw_param),
            SpriteType::Lizard(color) => self.sprites.lizards.add(gp.draw_param.color(color)),
            SpriteType::Turnip => self.sprites.turnips.add(gp.color(RED).draw_param),
            SpriteType::Skull => self.sprites.skulls.add(gp.draw_param),
            SpriteType::Cursor => self.sprites.cursors.add(gp.draw_param),
            _ => unimplemented!("Unimplemented sprite type: {:?}", sprite_type),
        };
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
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        let mut sprite_grid = sprite_grid_from_world(&self.world, self.selected_villager_key);

        sprite_grid.cursor_at(self.cursor.x + 1, self.cursor.y + 1);

        self.render_sprite_grid(sprite_grid);

        self.draw_all_spritebatches(ctx)?;

        graphics::present(ctx)?;

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
