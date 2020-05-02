use ggez::graphics::spritebatch::SpriteBatch;

use crate::bitter::{GRID_HEIGHT, GRID_WIDTH};

const SPRITE_GRID_LENGTH: usize = (GRID_WIDTH * GRID_HEIGHT) as usize;

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

    pub fn lizard_at(&mut self, x: u8, y: u8) {
        let index = (y * GRID_WIDTH + x) as usize;
        self.sprite_types[index] = SpriteType::Lizard;
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
