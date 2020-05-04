mod bitter;
mod renderer;
mod sprites;
mod config;

use ggez;
use ggez::event;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics;
use ggez::graphics::{Color, DrawParam};
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

use bitter::{
    Coords,
    Direction,
    EntityKey,
    GRID_WIDTH,
    GRID_HEIGHT,
    Ticks,
    World,
};

use renderer::sprite_grid_from_world;

use sprites::{SpriteGrid, Sprites, SpriteType};

use std::path;

const SPRITE_SCALE: f32 = 4.0;
const SPRITE_SIZE: f32 = 8.0 * SPRITE_SCALE;

const RED: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

#[derive(Copy,Clone)]
struct Cursor {
    x: u8,
    y: u8,
}

impl Cursor {
    fn new() -> Self {
        Self {
            x: 2,
            y: 2,
        }
    }

    fn coords(&self) -> Coords {
        (self.x, self.y)
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
        let draw_param = graphics::DrawParam::new()
            //.offset(na::Point2::new(0.0, 0.0))
            //.scale(na::Vector2::new(SPRITE_SCALE, SPRITE_SCALE))
            ;

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

fn invert(ctx: &mut Context, image: &graphics::Image) -> GameResult<graphics::Image> {
    let image_u8 = image.to_rgba8(ctx)?;

    let image_u8_i: Vec<u8> = image_u8.iter().enumerate().map(|(i, p)| {
        if (i + 1) % 4 == 0 {
            if image_u8[i - 1] == 255 {
                0 // transparent if the pixel is white)
            } else {
                255
            }
        } else {
            if *p == 0 { 255 } else { 0 }
        }
    }).collect();

    graphics::Image::from_rgba8(ctx, 8, 8, &image_u8_i)
}

fn prep_sprites(ctx: &mut Context, sprite_number: usize) -> GameResult<SpriteBatch> {
    let filepath = format!("/separate/{}.png", sprite_number);

    let original = graphics::Image::new(ctx, filepath).unwrap();
    let inverted = invert(ctx, &original)?;

    let mut inverted_batch = graphics::spritebatch::SpriteBatch::new(inverted);
    inverted_batch.set_filter(ggez::graphics::FilterMode::Nearest);

    // Source images are "inverted" by our standard, hence the reverse positioning
    Ok(inverted_batch)
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let sprites: Sprites = Sprites {
            curves: prep_sprites(ctx, 1)?,
            lines: prep_sprites(ctx, 2)?,
            crosses: prep_sprites(ctx, 3)?,
            corner_triangles: prep_sprites(ctx, 4)?,
            small_circles: prep_sprites(ctx, 5)?,
            big_circles: prep_sprites(ctx, 6)?,
            diamonds: prep_sprites(ctx, 7)?,
            dashes: prep_sprites(ctx, 8)?,
            dots: prep_sprites(ctx, 9)?,
            booms: prep_sprites(ctx, 10)?,
            skulls: prep_sprites(ctx, 11)?,
            side_triangles: prep_sprites(ctx, 12)?,
            ships: prep_sprites(ctx, 13)?,
            hearts: prep_sprites(ctx, 14)?,
            cursors: prep_sprites(ctx, 15)?,
            turnips: prep_sprites(ctx, 16)?,
            squids: prep_sprites(ctx, 17)?,
            lizards: prep_sprites(ctx, 18)?,
            balls: prep_sprites(ctx, 19)?,
            crabs: prep_sprites(ctx, 20)?,
            altars: prep_sprites(ctx, 21)?,
        };

        let ticks: Ticks = 0;

        let s = MainState {
            world: build_example_world(),
            sprites,
            cursor: Cursor::new(),
            selected_villager_key: None,
            ticks,
        };
        Ok(s)
    }

    fn draw_all_spritebatches(&mut self, ctx: &mut Context) -> GameResult {
        let origin_param = graphics::DrawParam::new().dest(na::Point2::new(0.0, 0.0)).scale(na::Vector2::new(SPRITE_SCALE, SPRITE_SCALE));

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
            },
            Direction::Down => {
                if self.cursor.y < GRID_HEIGHT - 2 {
                    self.cursor.y += 1
                }
            },
            Direction::Left => {
                if self.cursor.x > 1 {
                    self.cursor.x -= 1
                }
            },
            Direction::Right => {
                if self.cursor.x < GRID_WIDTH - 2 {
                    self.cursor.x += 1
                }
            },
        }
    }

    fn spawn_egg(&mut self, coords: Coords) {
        self.world.request_egg_spawn(coords);
    }

    fn render_sprite_grid(&mut self, sprite_grid: SpriteGrid) {
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                let sprite_type =  sprite_grid.sprite_type_at(x, y);
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
            SpriteType::Lizard(color) => {
                self.sprites.lizards.add(gp.draw_param.color(color))
            },
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
            let new_world = self.world.tick();
            self.world = new_world;
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
            KeyCode::Space => self.spawn_egg(self.cursor.coords()),
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

fn build_example_world() -> World {
    let mut world = World::new();

    world.add_villager_at(4, 4);
    world.add_villager_at(4, 5);

    world.add_farm_at(5, 4);
    world.add_farm_at(5, 5);
    world.add_farm_at(5, 6);

    world
}

pub fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./resources");

    config::initialize_config_file().unwrap();

    let cb = ggez::ContextBuilder::new("bitter-jam-entry", "ggez")
        .add_resource_path(resource_dir)
        .window_mode(ggez::conf::WindowMode::default()
                     .dimensions(GRID_WIDTH as f32 * SPRITE_SIZE, GRID_HEIGHT as f32 * SPRITE_SIZE));

    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new(ctx)?;

    event::run(ctx, event_loop, state)
}
