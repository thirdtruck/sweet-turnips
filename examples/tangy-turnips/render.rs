use crate::tangy::{EntityKey, World, GRID_HEIGHT, GRID_WIDTH};
use sweet_turnips::sprites::{Color, SpriteGrid};

const MAX_X: u8 = GRID_WIDTH - 1;

pub fn sprite_grid_from_world(
    world: &World,
) -> SpriteGrid {
    let mut sprite_grid = SpriteGrid::new();

    for y in 0..GRID_HEIGHT {
        sprite_grid.big_circle_at(0, y);
        sprite_grid.big_circle_at(MAX_X, y);
    }

    sprite_grid
}
