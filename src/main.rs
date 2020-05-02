
use ggez;
use ggez::event;
use ggez::graphics;
use ggez::graphics::Image;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use std::path;

struct Sprites {
    curves: graphics::spritebatch::SpriteBatch,
    curves_i: graphics::spritebatch::SpriteBatch,
    /*
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
    */
}

struct MainState {
    sprites: Sprites,
}

fn invert(ctx: &mut Context, image: &graphics::Image) -> GameResult<graphics::Image> {
    let image_u8 = image.to_rgba8(ctx)?;

    let image_u8_i: Vec<u8> = image_u8.iter().enumerate().map(|(i, p)| {
        if (i + 1) % 4 == 0 {
            *p
        } else {
            if *p == 0 { 255 } else { 0 }
        }
    }).collect();

    graphics::Image::from_rgba8(ctx, 8, 8, &image_u8_i)
}

fn prep_sprites(ctx: &mut Context, sprite_number: usize) -> GameResult<(SpriteBatch, SpriteBatch)> {
    let filepath = format!("/separate/{}.png", sprite_number);

    let original = graphics::Image::new(ctx, filepath).unwrap();
    let inverted = invert(ctx, &original)?;

    let mut original_batch = graphics::spritebatch::SpriteBatch::new(original);
    original_batch.set_filter(ggez::graphics::FilterMode::Nearest);

    let mut inverted_batch = graphics::spritebatch::SpriteBatch::new(inverted);
    inverted_batch.set_filter(ggez::graphics::FilterMode::Nearest);

    // Source images are "inverted" by our standard, hence the reverse positioning
    Ok((inverted_batch, original_batch))
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let (curves, curves_i) = prep_sprites(ctx, 1)?;

        let sprites: Sprites = Sprites {
            curves,
            curves_i,
        };

        let s = MainState { sprites };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        self.sprites.curves.add(graphics::DrawParam::new()
                               .scale(na::Vector2::new(4.0, 4.0)));

        let param = graphics::DrawParam::new()
            .dest(na::Point2::new(0.0, 0.0));

        graphics::draw(ctx, &self.sprites.curves, param)?;

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
