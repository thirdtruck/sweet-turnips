
use ggez;
use ggez::event;
use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use std::path;

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
    default_sprite_param: DrawParam,
}

fn grid_point(x: u8, y: u8) -> na::Point2<f32> {
    na::Point2::new((30 * x) as f32, (30 * y) as f32)
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

        let default_sprite_param = graphics::DrawParam::new()
            .scale(na::Vector2::new(4.0, 4.0));

        let s = MainState {
            sprites,
            default_sprite_param,
        };
        Ok(s)
    }

    fn curve_at(&mut self, x: u8, y: u8) {
        self.sprites.curves.add(self.default_sprite_param.dest(grid_point(x, y)));
    }

    fn line_at(&mut self, x: u8, y: u8) {
        self.sprites.lines.add(self.default_sprite_param.dest(grid_point(x, y)));
    }

    fn cross_at(&mut self, x: u8, y: u8) {
        self.sprites.crosses.add(self.default_sprite_param.dest(grid_point(x, y)));
    }

    fn corner_triangle_at(&mut self, x: u8, y: u8) {
        self.sprites.corner_triangles.add(self.default_sprite_param.dest(grid_point(x, y)));
    }

    fn small_circle_at(&mut self, x: u8, y: u8) {
        self.sprites.small_circles.add(self.default_sprite_param.dest(grid_point(x, y)));
    }

    fn big_circle_at(&mut self, x: u8, y: u8) {
        self.sprites.big_circles.add(self.default_sprite_param.dest(grid_point(x, y)));
    }

    fn diamond_at(&mut self, x: u8, y: u8) {
        self.sprites.diamonds.add(self.default_sprite_param.dest(grid_point(x, y)));
    }

    fn dash_at(&mut self, x: u8, y: u8) {
        self.sprites.dashes.add(self.default_sprite_param.dest(grid_point(x, y)));
    }

    fn boom_at(&mut self, x: u8, y: u8) {
        self.sprites.booms.add(self.default_sprite_param.dest(grid_point(x, y)));
    }

    fn skull_at(&mut self, x: u8, y: u8) {
        self.sprites.skulls.add(self.default_sprite_param.dest(grid_point(x, y)));
    }

    fn side_triangle_at(&mut self, x: u8, y: u8) {
        self.sprites.side_triangles.add(self.default_sprite_param.dest(grid_point(x, y)));
    }

    fn ship_at(&mut self, x: u8, y: u8) {
        self.sprites.ships.add(self.default_sprite_param.dest(grid_point(x, y)));
    }

    fn heart_at(&mut self, x: u8, y: u8) {
        self.sprites.hearts.add(self.default_sprite_param.dest(grid_point(x, y)));
    }

    fn cursor_at(&mut self, x: u8, y: u8) {
        self.sprites.cursors.add(self.default_sprite_param.dest(grid_point(x, y)));
    }

    fn turnip_at(&mut self, x: u8, y: u8) {
        self.sprites.turnips.add(self.default_sprite_param.dest(grid_point(x, y)));
    }

    fn squid_at(&mut self, x: u8, y: u8) {
        self.sprites.squids.add(self.default_sprite_param.dest(grid_point(x, y)));
    }

    fn lizard_at(&mut self, x: u8, y: u8) {
        self.sprites.lizards.add(self.default_sprite_param.dest(grid_point(x, y)));
    }

    fn ball_at(&mut self, x: u8, y: u8) {
        self.sprites.balls.add(self.default_sprite_param.dest(grid_point(x, y)));
    }

    fn crab_at(&mut self, x: u8, y: u8) {
        self.sprites.crabs.add(self.default_sprite_param.dest(grid_point(x, y)));
    }

    fn altar_at(&mut self, x: u8, y: u8) {
        self.sprites.altars.add(self.default_sprite_param.dest(grid_point(x, y)));
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        self.curve_at(0, 0);

        self.line_at(0, 1);

        self.cross_at(0, 2);

        let origin_param = graphics::DrawParam::new().dest(na::Point2::new(0.0, 0.0));

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

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./resources");

    let cb = ggez::ContextBuilder::new("bitter-jam-entry", "ggez").add_resource_path(resource_dir);
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
