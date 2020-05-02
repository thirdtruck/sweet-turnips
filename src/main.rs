mod bitter;
mod sprites;

use ggez;
use ggez::event;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics;
use ggez::graphics::{Color, DrawParam};
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

use bitter::{
    Direction,
    EntityId,
    GRID_WIDTH,
    GRID_HEIGHT,
    Ticks,
    Villager,
    World,
};

use sprites::{Sprites, SpriteType};

use std::path;
use std::f32::consts::PI;

const SPRITE_SCALE: f32 = 4.0;
const SPRITE_SIZE: f32 = 8.0 * SPRITE_SCALE;

const SPRITE_GRID_LENGTH: usize = (GRID_WIDTH * GRID_HEIGHT) as usize;

const RED: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

#[allow(dead_code)]
const D0: f32 = 0.0;
#[allow(dead_code)]
const D90: f32 = PI / 2.0;
#[allow(dead_code)]
const D180: f32 = (PI / 2.0) * 2.0;
#[allow(dead_code)]
const D270: f32 = (PI / 2.0) * 3.0;

struct SpriteGrid {
    sprite_types: [SpriteType; SPRITE_GRID_LENGTH],
}

impl SpriteGrid {
    fn new() -> Self {
        SpriteGrid {
            sprite_types: [SpriteType::Empty; SPRITE_GRID_LENGTH],
        }
    }

    fn big_circle_at(&mut self, x: u8, y: u8) {
        let index = (y * GRID_WIDTH + x) as usize;
        self.sprite_types[index] = SpriteType::BigCircle;
    }
}

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

    fn overlaps(&self, x: u8, y: u8) -> bool {
        self.x + 1 == x && self.y + 1 == y
    }
}

struct MainState {
    world: World,
    sprites: Sprites,
    cursor: Cursor,
    selected_villager_id: Option<EntityId>,
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

    // TODO: Figure out why this is broken
    #[allow(dead_code)]
    fn rotated(&self, radians: f32) -> Self {
        let draw_param = self.draw_param
            .offset(na::Point2::new(0.5, 0.5))
            .rotation(radians)
            ;

        GridParam { draw_param }
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

#[allow(dead_code)] // For all the  [sprite]_at methods, at least until they have all been used somewhere
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
            world: World::new(),
            sprites,
            cursor: Cursor::new(),
            selected_villager_id: None,
            ticks,
        };
        Ok(s)
    }

    // TODO: Fix whatever borrow issues made it necessary to make possible_id into an Option
    fn find_villager(&mut self, id: EntityId) -> Option<Villager> {
        for villager in self.world.villagers.iter() {
            if villager.id == id {
                return Some(villager.clone());
            }
        }

        None
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

    fn curve(&mut self, gp: GridParam) {
        self.sprites.curves.add(gp.draw_param);
    }

    fn line(&mut self, gp: GridParam) {
        self.sprites.lines.add(gp.draw_param);
    }

    fn cross(&mut self, gp: GridParam) {
        self.sprites.crosses.add(gp.draw_param);
    }

    fn corner_triangle(&mut self, gp: GridParam) {
        self.sprites.corner_triangles.add(gp.draw_param);
    }

    fn small_circle(&mut self, gp: GridParam) {
        self.sprites.small_circles.add(gp.draw_param);
    }

    fn big_circle(&mut self, gp: GridParam) {
        self.sprites.big_circles.add(gp.draw_param);
    }

    fn diamond(&mut self, gp: GridParam) {
        self.sprites.diamonds.add(gp.draw_param);
    }

    fn dash(&mut self, gp: GridParam) {
        self.sprites.dashes.add(gp.draw_param);
    }

    fn boom(&mut self, gp: GridParam) {
        self.sprites.booms.add(gp.draw_param);
    }

    fn skull(&mut self, gp: GridParam) {
        self.sprites.skulls.add(gp.draw_param);
    }

    fn side_triangle(&mut self, gp: GridParam) {
        self.sprites.side_triangles.add(gp.draw_param);
    }

    fn ship(&mut self, gp: GridParam) {
        self.sprites.ships.add(gp.draw_param);
    }

    fn heart(&mut self, gp: GridParam) {
        self.sprites.hearts.add(gp.draw_param);
    }

    fn cursor(&mut self, gp: GridParam) {
        self.sprites.cursors.add(gp.draw_param);
    }

    fn turnip(&mut self, gp: GridParam) {
        self.sprites.turnips.add(gp.draw_param);
    }

    fn squid(&mut self, gp: GridParam) {
        self.sprites.squids.add(gp.draw_param);
    }

    fn lizard(&mut self, gp: GridParam) {
        self.sprites.lizards.add(gp.draw_param);
    }

    fn ball(&mut self, gp: GridParam) {
        self.sprites.balls.add(gp.draw_param);
    }

    fn crab(&mut self, gp: GridParam) {
        self.sprites.crabs.add(gp.draw_param);
    }

    fn altar(&mut self, gp: GridParam) {
        self.sprites.altars.add(gp.draw_param);
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
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.ticks += 1;

        self.world.tick();

        self.selected_villager_id = self.world.villager_id_at(self.cursor.x, self.cursor.y);

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
            _ => (),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        let mut sprite_grid = SpriteGrid::new();

        let gp = GridParam::new();

        let selected_villager = match self.selected_villager_id {
            Some(id) => self.find_villager(id),
            None => None,
        };

        sprite_grid.big_circle_at(0, 0);

        self.big_circle(gp.at(0, 0));

        for x in 1..7 {
            self.big_circle(gp.at(x, 0));
        }

        self.big_circle(gp.at(0, 7));

        for y in 1..7 {
            if ! self.cursor.overlaps(7, y) {
                self.big_circle(gp.at(7, y));
            }
        }

        if ! self.cursor.overlaps(7, 7) {
            self.big_circle(gp.at(7, 7));
        }

        if let Some(villager) = selected_villager {
            for x in 1..7 {
                if villager.satiation >= x {
                    self.turnip(gp.at(x, 7).color(RED));
                }
            }
        } else {
            for x in 1..7 {
                if ! self.cursor.overlaps(x, 7) {
                    self.big_circle(gp.at(x, 7));
                }
            }
        }

        self.big_circle(gp.at(7, 0));

        for y in 1..7 {
            self.big_circle(gp.at(0, y));
        }

        let death_marker_coords: Vec<(u8, u8)> = self.world.death_markers.iter().map(|dm| (dm.x, dm.y)).collect();

        for (x, y) in death_marker_coords {
            self.skull(gp.at(x, y));
        }

        let villager_coords: Vec<(u8, u8)> = self.world.villagers.iter().map(|v| (v.x, v.y)).collect();

        for (x, y) in villager_coords {
            self.lizard(gp.at(x, y));
        }

        self.cursor(gp.at(self.cursor.x + 1, self.cursor.y + 1));

        self.draw_all_spritebatches(ctx)?;

        graphics::present(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./resources");

    let cb = ggez::ContextBuilder::new("bitter-jam-entry", "ggez")
        .add_resource_path(resource_dir)
        .window_mode(ggez::conf::WindowMode::default()
                     .dimensions(GRID_WIDTH as f32 * SPRITE_SIZE, GRID_HEIGHT as f32 * SPRITE_SIZE));

    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new(ctx)?;

    event::run(ctx, event_loop, state)
}
