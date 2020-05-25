use crate::bitter::{EntityKey, World};
use sweet_turnips::sprites::{Color, Sprite, SpriteGrid};

const RED: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

// This trait exists solely to map more domain-specific
// (i.e. game-specific) language onto SpriteGrid's commands
trait BitterSpriteGrid {
    fn border_at(&mut self, x: u8, y: u8);
    fn farm_at(&mut self, x: u8, y: u8);
    fn villager_at(&mut self, color: Color, x: u8, y: u8);
    fn death_marker_at(&mut self, x: u8, y: u8);
    fn cursor_at(&mut self, x: u8, y: u8);
}

impl BitterSpriteGrid for SpriteGrid {
    fn border_at(&mut self, x: u8, y: u8) {
        self.render_sprite_at(Sprite::big_circle(), x, y);
    }

    fn farm_at(&mut self, x: u8, y: u8) {
        self.render_sprite_at(Sprite::turnip().colored(RED), x, y);
    }

    fn villager_at(&mut self, color: Color, x: u8, y: u8) {
        self.render_sprite_at(Sprite::lizard().colored(color), x, y);
    }

    fn death_marker_at(&mut self, x: u8, y: u8) {
        self.render_sprite_at(Sprite::skull(), x, y);
    }

    fn cursor_at(&mut self, x: u8, y: u8) {
        self.render_sprite_at(Sprite::cursor(), x, y);
    }
}

pub fn sprite_grid_from_world(
    world: &World,
    selected_villager_key: Option<EntityKey>,
) -> SpriteGrid {
    let selected_villager = match selected_villager_key {
        Some(key) => world.villager(key),
        None => None,
    };

    let mut sprite_grid = SpriteGrid::new();

    for x in 0..8 {
        sprite_grid.border_at(x, 0);
        sprite_grid.border_at(x, 7);
    }

    for y in 0..8 {
        sprite_grid.border_at(0, y);
        sprite_grid.border_at(7, y);
    }

    let farm_coords: Vec<(u8, u8)> = world.farms.values().map(|v| world.coords[v.key]).collect();

    for (x, y) in farm_coords {
        sprite_grid.farm_at(x, y);
    }

    for key in world.villagers.keys() {
        let (x, y) = world.coords[key];
        let satiation = world.satiation[key];

        let r = satiation as f32 / 5.0;

        let color = Color {
            r,
            g: 0.2,
            b: 0.2,
            a: 1.0,
        };

        sprite_grid.villager_at(color, x, y);
    }

    if let Some(villager) = selected_villager {
        let satiation = world.satiation[villager.key];

        for x in 1..7 {
            if satiation >= x {
                sprite_grid.farm_at(x, 7);
            }
        }
    }

    for dm in world.death_markers.values() {
        let (x, y) = world.coords[dm.key];
        sprite_grid.death_marker_at(x, y);
    }

    let coords = world.cursor_coords();

    sprite_grid.cursor_at(coords.0 + 1, coords.1 + 1);

    sprite_grid
}
