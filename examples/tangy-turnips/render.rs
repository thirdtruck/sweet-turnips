use crate::tangy::{World, GRID_HEIGHT, GRID_WIDTH};
use sweet_turnips::sprites::{Sprite, SpriteGrid, SpriteType};

const MAX_X: u8 = GRID_WIDTH - 1;

trait TangySpriteGrid {
    fn player_ship_at(&mut self, x: u8, y: u8);
    fn enemy_ship_at(&mut self, x: u8, y: u8);
    fn big_gutter_at(&mut self, x: u8, y: u8);
    fn small_gutter_at(&mut self, x: u8, y: u8);
}

impl TangySpriteGrid for SpriteGrid {
    fn player_ship_at(&mut self, x: u8, y: u8) {
        self.render_sprite_at(Sprite::new(SpriteType::Ship), x, y);
    }

    fn enemy_ship_at(&mut self, x: u8, y: u8) {
        self.render_sprite_at(Sprite::new(SpriteType::Turnip), x, y);
    }

    fn big_gutter_at(&mut self, x: u8, y: u8) {
        self.render_sprite_at(Sprite::new(SpriteType::BigCircle), x, y);
    }

    fn small_gutter_at(&mut self, x: u8, y: u8) {
        self.render_sprite_at(Sprite::new(SpriteType::SmallCircle), x, y);
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
