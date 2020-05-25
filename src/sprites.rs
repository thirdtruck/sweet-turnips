use ggez;
use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

use ggez::graphics::spritebatch::SpriteBatch;

pub use ggez::graphics::Color;

// TODO: Make these constants into parameters wherever practical
const GRID_WIDTH: u8 = 8;
const GRID_HEIGHT: u8 = 8;

const SPRITE_GRID_LENGTH: usize = (GRID_WIDTH * GRID_HEIGHT) as usize;

const SPRITE_SCALE: f32 = 4.0;
const SPRITE_SIZE: f32 = 8.0;

#[derive(Copy, Clone, Debug)]
pub struct Sprite {
    sprite_type: SpriteType,
    grid_param: GridParam,
}

impl Sprite {
    pub fn new(sprite_type: SpriteType) -> Self {
        Self {
            sprite_type,
            grid_param: GridParam::new(),
        }
    }

    pub fn skull() -> Self {
        Self::new(SpriteType::Skull)
    }

    pub fn ship() -> Self {
        Self::new(SpriteType::Ship)
    }

    pub fn big_circle() -> Self {
        Self::new(SpriteType::BigCircle)
    }

    pub fn small_circle() -> Self {
        Self::new(SpriteType::SmallCircle)
    }

    pub fn turnip() -> Self {
        Self::new(SpriteType::Turnip)
    }

    pub fn lizard() -> Self {
        Self::new(SpriteType::Lizard)
    }

    pub fn cursor() -> Self {
        Self::new(SpriteType::Cursor)
    }

    pub fn colored(self, color: Color) -> Self {
        Self {
            grid_param: self.grid_param.color(color),
            ..self
        }
    }
}


#[derive(Copy, Clone, Debug)]
struct GridParam {
    draw_param: DrawParam,
}

impl GridParam {
    pub fn new() -> Self {
        let draw_param = graphics::DrawParam::new();

        GridParam { draw_param }
    }

    fn at(&self, x: u8, y: u8) -> Self {
        let x = x as f32;
        let y = y as f32;

        let point = na::Point2::new(SPRITE_SIZE * x, SPRITE_SIZE * y);

        GridParam {
            draw_param: self.draw_param.dest(point),
        }
    }

    pub fn color(&self, color: Color) -> Self {
        GridParam {
            draw_param: self.draw_param.color(color),
        }
    }
}

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
    Lizard,
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

        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

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

        graphics::present(ctx)?;

        Ok(())
    }

    fn render_sprite_at(&mut self, sprite_type: SpriteType, gp: GridParam, x: u8, y: u8) {
        if sprite_type == SpriteType::Empty {
            return;
        }

        let gp = gp.at(x, y);

        match sprite_type {
            SpriteType::Ship => self.ships.add(gp.draw_param),
            SpriteType::BigCircle => self.big_circles.add(gp.draw_param),
            SpriteType::SmallCircle => self.small_circles.add(gp.draw_param),
            SpriteType::Lizard => self.lizards.add(gp.draw_param),
            SpriteType::Turnip => self.turnips.add(gp.draw_param),
            SpriteType::Skull => self.skulls.add(gp.draw_param),
            SpriteType::Cursor => self.cursors.add(gp.draw_param),
            _ => unimplemented!("Unimplemented sprite type: {:?}", sprite_type),
        };
    }

    pub fn render_sprite_grid(&mut self, sprite_grid: SpriteGrid) {
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                let sprite_type = sprite_grid.sprite_type_at(x, y);
                let grid_param = match sprite_grid.grid_param_at(x, y) {
                    Some(gp) => gp,
                    None => GridParam::new(),
                };
                self.render_sprite_at(sprite_type, grid_param, x, y);
            }
        }
    }
}

pub struct SpriteGrid {
    sprite_types: [SpriteType; SPRITE_GRID_LENGTH],
    grid_params: [Option<GridParam>; SPRITE_GRID_LENGTH],
}

fn index(x: u8, y: u8) -> usize {
    (y * GRID_WIDTH + x) as usize
}

impl SpriteGrid {
    pub fn new() -> Self {
        SpriteGrid {
            sprite_types: [SpriteType::Empty; SPRITE_GRID_LENGTH],
            grid_params: [None; SPRITE_GRID_LENGTH],
        }
    }

    pub fn render_sprite_at(&mut self, sprite: Sprite, x: u8, y: u8) {
        self.sprite_types[index(x, y)] = sprite.sprite_type;
        self.grid_params[index(x, y)] = Some(sprite.grid_param.at(x, y));
    }

    fn sprite_type_at(&self, x: u8, y: u8) -> SpriteType {
        self.sprite_types[index(x, y)]
    }

    fn grid_param_at(&self, x: u8, y: u8) -> Option<GridParam> {
        self.grid_params[index(x, y)]
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
