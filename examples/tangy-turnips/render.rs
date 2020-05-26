use crate::tangy::{World, GRID_HEIGHT, GRID_WIDTH};
use sweet_turnips::sprites::{Color, Sprite, SpriteGrid};

const MAX_X: u8 = GRID_WIDTH - 1;

const RED: Color = Color { r: 0.8, g: 0.2, b: 0.2, a: 1.0 };
const YELLOW: Color = Color { r: 0.8, g: 0.8, b: 0.2, a: 1.0 };
const BLUE: Color = Color { r: 0.2, g: 0.2, b: 0.8, a: 1.0 };
const GREEN: Color = Color { r: 0.2, g: 0.8, b: 0.2, a: 1.0 };

// This trait exists solely to map more domain-specific
// (i.e. game-specific) language onto SpriteGrid's commands
trait TangySpriteGrid {
    fn player_ship_at(&mut self, x: u8, y: u8);
    fn player_bullet_at(&mut self, x: u8, y: u8);
    fn enemy_ship_at(&mut self, x: u8, y: u8);
    fn big_gutter_at(&mut self, x: u8, y: u8);
    fn small_gutter_at(&mut self, x: u8, y: u8);
}

impl TangySpriteGrid for SpriteGrid {
    fn player_ship_at(&mut self, x: u8, y: u8) {
        self.render_sprite_at(Sprite::ship().colored(GREEN), x, y);
    }

    fn player_bullet_at(&mut self, x: u8, y: u8) {
        self.render_sprite_at(Sprite::ball().colored(YELLOW), x, y);
    }

    fn enemy_ship_at(&mut self, x: u8, y: u8) {
        self.render_sprite_at(Sprite::turnip().colored(RED), x, y);
    }

    fn big_gutter_at(&mut self, x: u8, y: u8) {
        self.render_sprite_at(Sprite::big_circle(), x, y);
    }

    fn small_gutter_at(&mut self, x: u8, y: u8) {
        self.render_sprite_at(Sprite::small_circle().colored(BLUE), x, y);
    }
}

pub fn sprite_grid_from_world(world: &World) -> SpriteGrid {
    let mut sprite_grid = SpriteGrid::new();

    let y_transit = (world.ticks % GRID_HEIGHT as usize) as u8;

    for y in 0..GRID_HEIGHT {
        if y == y_transit {
            sprite_grid.small_gutter_at(0, y);
            sprite_grid.small_gutter_at(MAX_X, y);
        } else {
            sprite_grid.big_gutter_at(0, y);
            sprite_grid.big_gutter_at(MAX_X, y);
        }
    }

    for bullet in world.player_bullets.values() {
        let coords = world.coords[bullet.key];

        sprite_grid.player_bullet_at(coords.0, coords.1);
    }

    for ship in world.player_ships.values() {
        let coords = world.coords[ship.key];

        sprite_grid.player_ship_at(coords.0, coords.1);
    }

    for ship in world.enemy_ships.values() {
        let coords = world.coords[ship.key];

        sprite_grid.enemy_ship_at(coords.0, coords.1);
    }

    sprite_grid
}
