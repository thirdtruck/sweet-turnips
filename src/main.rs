
use ggez;
use ggez::event;
use ggez::graphics;
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
                               .scale(na::Vector2::new(4.0, 4.0))
                               .dest(na::Point2::new(0.0, 0.0))
                               );

        self.sprites.lines.add(graphics::DrawParam::new()
                               .scale(na::Vector2::new(4.0, 4.0))
                               .dest(na::Point2::new(0.0, 30.0))
                               );

        self.sprites.crosses.add(graphics::DrawParam::new()
                               .scale(na::Vector2::new(4.0, 4.0))
                               .dest(na::Point2::new(0.0, 60.0))
                               );

        self.sprites.corner_triangles.add(graphics::DrawParam::new()
                               .scale(na::Vector2::new(4.0, 4.0))
                               .dest(na::Point2::new(0.0, 90.0))
                               );

        self.sprites.small_circles.add(graphics::DrawParam::new()
                               .scale(na::Vector2::new(4.0, 4.0))
                               .dest(na::Point2::new(0.0, 120.0))
                               );

        self.sprites.big_circles.add(graphics::DrawParam::new()
                               .scale(na::Vector2::new(4.0, 4.0))
                               .dest(na::Point2::new(30.0, 0.0))
                               );

        self.sprites.diamonds.add(graphics::DrawParam::new()
                               .scale(na::Vector2::new(4.0, 4.0))
                               .dest(na::Point2::new(30.0, 30.0))
                               );

        self.sprites.dashes.add(graphics::DrawParam::new()
                               .scale(na::Vector2::new(4.0, 4.0))
                               .dest(na::Point2::new(30.0, 60.0))
                               );

        self.sprites.dots.add(graphics::DrawParam::new()
                               .scale(na::Vector2::new(4.0, 4.0))
                               .dest(na::Point2::new(30.0, 90.0))
                               );

        self.sprites.booms.add(graphics::DrawParam::new()
                               .scale(na::Vector2::new(4.0, 4.0))
                               .dest(na::Point2::new(30.0, 120.0))
                               );

        let param = graphics::DrawParam::new()
            .dest(na::Point2::new(0.0, 0.0));

        graphics::draw(ctx, &self.sprites.lines, param)?;
        graphics::draw(ctx, &self.sprites.curves, param)?;
        graphics::draw(ctx, &self.sprites.crosses, param)?;
        graphics::draw(ctx, &self.sprites.corner_triangles, param)?;
        graphics::draw(ctx, &self.sprites.small_circles, param)?;
        graphics::draw(ctx, &self.sprites.big_circles, param)?;
        graphics::draw(ctx, &self.sprites.diamonds, param)?;
        graphics::draw(ctx, &self.sprites.dashes, param)?;
        graphics::draw(ctx, &self.sprites.dots, param)?;
        graphics::draw(ctx, &self.sprites.booms, param)?;
        graphics::draw(ctx, &self.sprites.skulls, param)?;
        graphics::draw(ctx, &self.sprites.side_triangles, param)?;
        graphics::draw(ctx, &self.sprites.ships, param)?;
        graphics::draw(ctx, &self.sprites.hearts, param)?;
        graphics::draw(ctx, &self.sprites.cursors, param)?;
        graphics::draw(ctx, &self.sprites.turnips, param)?;
        graphics::draw(ctx, &self.sprites.squids, param)?;
        graphics::draw(ctx, &self.sprites.lizards, param)?;
        graphics::draw(ctx, &self.sprites.balls, param)?;
        graphics::draw(ctx, &self.sprites.crabs, param)?;
        graphics::draw(ctx, &self.sprites.altars, param)?;

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
