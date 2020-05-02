
use ggez;
use ggez::event;
use ggez::graphics;
use ggez::graphics::{Color, DrawParam};
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use std::path;
use std::f32::consts::PI;

const SPRITE_SCALE: f32 = 4.0;
const SPRITE_SIZE: f32 = 8.0 * SPRITE_SCALE;
const GRID_WIDTH: f32 = 8.0;
const GRID_HEIGHT: f32 = 8.0;

const RED: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

const D0: f32 = 0.0;
const D90: f32 = PI / 2.0;
const D180: f32 = PI;
const D270: f32 = PI * 1.5;

struct Sprites {
    curves: graphics::spritebatch::SpriteBatch,
    lines: graphics::spritebatch::SpriteBatch,
    crosses: graphics::spritebatch::SpriteBatch,
    corner_triangles: graphics::spritebatch::SpriteBatch,
    small_circles: graphics::spritebatch::SpriteBatch,
    big_circles: graphics::spritebatch::SpriteBatch,
    diamonds: graphics::spritebatch::SpriteBatch,
    dashes: graphics::spritebatch::SpriteBatch,
    dots: graphics::spritebatch::SpriteBatch,
    booms: graphics::spritebatch::SpriteBatch,
    skulls: graphics::spritebatch::SpriteBatch,
    side_triangles: graphics::spritebatch::SpriteBatch,
    ships: graphics::spritebatch::SpriteBatch,
    hearts: graphics::spritebatch::SpriteBatch,
    cursors: graphics::spritebatch::SpriteBatch,
    turnips: graphics::spritebatch::SpriteBatch,
    squids: graphics::spritebatch::SpriteBatch,
    lizards: graphics::spritebatch::SpriteBatch,
    balls: graphics::spritebatch::SpriteBatch,
    crabs: graphics::spritebatch::SpriteBatch,
    altars: graphics::spritebatch::SpriteBatch,
}

struct MainState {
    sprites: Sprites,
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

    fn rotated(&self, radians: f32) -> Self {
        let draw_param = self.draw_param
            .offset(na::Point2::new(0.0, 0.0))
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

        let s = MainState {
            sprites,
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
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        let gp = GridParam::new();


        self.curve(gp.at(0, 0));

        for x in 1..7 {
            self.line(gp.at(x, 0));
        }

        self.curve(gp.at(0, 7));

        for y in 1..7 {
            self.line(gp.at(7, y).rotated(D90));
        }

        self.curve(gp.at(7, 7));

        for x in 1..7 {
            self.line(gp.at(x, 7));
        }

        self.curve(gp.at(7, 0));

        for y in 1..7 {
            self.line(gp.at(0, y).rotated(D90));
        }


        self.line(gp.at(0, 1));

        self.cross(gp.at(0, 2));

        self.altar(gp.at(2, 2).rotated(D0));
        self.altar(gp.at(2, 2).rotated(D180).color(RED));

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
                     .dimensions(GRID_WIDTH * SPRITE_SIZE, GRID_HEIGHT * SPRITE_SIZE));

    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new(ctx)?;

    event::run(ctx, event_loop, state)
}
