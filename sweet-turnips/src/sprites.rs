use ggez;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::{Context, GameResult};
use ggez::nalgebra as na;

pub use ggez::graphics::spritebatch::SpriteBatch;

const GRID_WIDTH: u8 = 8;
const GRID_HEIGHT: u8 = 8;

const SPRITE_GRID_LENGTH: usize = (GRID_WIDTH * GRID_HEIGHT) as usize;

const SPRITE_SCALE: f32 = 4.0;

#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum SpriteType {
    Curve,
    Line,
    Cross,
    CornerTriangle,
    SmallCircle,
    BigCircle,
    Diamond,
    Dash,
    Dot,
    Boom,
    Skull,
    SideTriangle,
    Ship,
    Heart,
    Cursor,
    Turnip,
    Squid,
    Lizard(Color),
    Ball,
    Crab,
    Altar,
    Empty,
}

pub struct Sprites {
    pub curves: SpriteBatch,
    pub lines: SpriteBatch,
    pub crosses: SpriteBatch,
    pub corner_triangles: SpriteBatch,
    pub small_circles: SpriteBatch,
    pub big_circles: SpriteBatch,
    pub diamonds: SpriteBatch,
    pub dashes: SpriteBatch,
    pub dots: SpriteBatch,
    pub booms: SpriteBatch,
    pub skulls: SpriteBatch,
    pub side_triangles: SpriteBatch,
    pub ships: SpriteBatch,
    pub hearts: SpriteBatch,
    pub cursors: SpriteBatch,
    pub turnips: SpriteBatch,
    pub squids: SpriteBatch,
    pub lizards: SpriteBatch,
    pub balls: SpriteBatch,
    pub crabs: SpriteBatch,
    pub altars: SpriteBatch,
}

impl Sprites {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
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

        Ok(sprites)
    }

    pub fn draw_all_sprites(&mut self, ctx: &mut Context) -> GameResult {
        let origin_param = graphics::DrawParam::new()
            .dest(na::Point2::new(0.0, 0.0))
            .scale(na::Vector2::new(SPRITE_SCALE, SPRITE_SCALE));

        graphics::draw(ctx, &self.lines, origin_param)?;
        graphics::draw(ctx, &self.curves, origin_param)?;
        graphics::draw(ctx, &self.crosses, origin_param)?;
        graphics::draw(ctx, &self.corner_triangles, origin_param)?;
        graphics::draw(ctx, &self.small_circles, origin_param)?;
        graphics::draw(ctx, &self.big_circles, origin_param)?;
        graphics::draw(ctx, &self.diamonds, origin_param)?;
        graphics::draw(ctx, &self.dashes, origin_param)?;
        graphics::draw(ctx, &self.dots, origin_param)?;
        graphics::draw(ctx, &self.booms, origin_param)?;
        graphics::draw(ctx, &self.skulls, origin_param)?;
        graphics::draw(ctx, &self.side_triangles, origin_param)?;
        graphics::draw(ctx, &self.ships, origin_param)?;
        graphics::draw(ctx, &self.hearts, origin_param)?;
        graphics::draw(ctx, &self.cursors, origin_param)?;
        graphics::draw(ctx, &self.turnips, origin_param)?;
        graphics::draw(ctx, &self.squids, origin_param)?;
        graphics::draw(ctx, &self.lizards, origin_param)?;
        graphics::draw(ctx, &self.balls, origin_param)?;
        graphics::draw(ctx, &self.crabs, origin_param)?;
        graphics::draw(ctx, &self.altars, origin_param)?;

        self.lines.clear();
        self.curves.clear();
        self.crosses.clear();
        self.corner_triangles.clear();
        self.small_circles.clear();
        self.big_circles.clear();
        self.diamonds.clear();
        self.dashes.clear();
        self.dots.clear();
        self.booms.clear();
        self.skulls.clear();
        self.side_triangles.clear();
        self.ships.clear();
        self.hearts.clear();
        self.cursors.clear();
        self.turnips.clear();
        self.squids.clear();
        self.lizards.clear();
        self.balls.clear();
        self.crabs.clear();
        self.altars.clear();

        Ok(())
    }
}

pub struct SpriteGrid {
    sprite_types: [SpriteType; SPRITE_GRID_LENGTH],
}

impl SpriteGrid {
    pub fn new() -> Self {
        SpriteGrid {
            sprite_types: [SpriteType::Empty; SPRITE_GRID_LENGTH],
        }
    }

    pub fn big_circle_at(&mut self, x: u8, y: u8) {
        let index = (y * GRID_WIDTH + x) as usize;
        self.sprite_types[index] = SpriteType::BigCircle;
    }

    pub fn lizard_at(&mut self, x: u8, y: u8, color: Color) {
        let index = (y * GRID_WIDTH + x) as usize;
        self.sprite_types[index] = SpriteType::Lizard(color);
    }

    pub fn turnip_at(&mut self, x: u8, y: u8) {
        let index = (y * GRID_WIDTH + x) as usize;
        self.sprite_types[index] = SpriteType::Turnip;
    }

    pub fn skull_at(&mut self, x: u8, y: u8) {
        let index = (y * GRID_WIDTH + x) as usize;
        self.sprite_types[index] = SpriteType::Skull;
    }

    pub fn cursor_at(&mut self, x: u8, y: u8) {
        let index = (y * GRID_WIDTH + x) as usize;
        self.sprite_types[index] = SpriteType::Cursor;
    }

    pub fn sprite_type_at(&self, x: u8, y: u8) -> SpriteType {
        let index = (y * GRID_WIDTH + x) as usize;
        self.sprite_types[index]
    }
}

fn prep_sprites(ctx: &mut Context, sprite_number: usize) -> GameResult<SpriteBatch> {
    let filepath = format!("/separate/{}.png", sprite_number);

    let original = graphics::Image::new(ctx, filepath).unwrap();
    let inverted = invert(ctx, &original)?;

    let mut inverted_batch = SpriteBatch::new(inverted);
    inverted_batch.set_filter(ggez::graphics::FilterMode::Nearest);

    // Source images are "inverted" by our standard, hence the reverse positioning
    Ok(inverted_batch)
}

fn invert(ctx: &mut Context, image: &graphics::Image) -> GameResult<graphics::Image> {
    let image_u8 = image.to_rgba8(ctx)?;

    let image_u8_i: Vec<u8> = image_u8
        .iter()
        .enumerate()
        .map(|(i, p)| {
            if (i + 1) % 4 == 0 {
                if image_u8[i - 1] == 255 {
                    0 // transparent if the pixel is white
                } else {
                    255
                }
            } else {
                if *p == 0 {
                    255
                } else {
                    0
                }
            }
        })
        .collect();

    graphics::Image::from_rgba8(ctx, 8, 8, &image_u8_i)
}