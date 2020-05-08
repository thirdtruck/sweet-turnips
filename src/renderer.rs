use crate::bitter::{EntityKey, World};
use sweet_turnips::sprites::{Color, SpriteGrid, SpriteGridRenderer};

pub struct WorldRenderer {
    pub world: World,
    pub selected_villager_key: Option<EntityKey>,
}

impl SpriteGridRenderer for WorldRenderer {
    fn render_grid(&self) -> SpriteGrid {
        sprite_grid_from_world(&self.world, self. selected_villager_key)
    }
}

fn sprite_grid_from_world(
    world: &World,
    selected_villager_key: Option<EntityKey>,
) -> SpriteGrid {
    let selected_villager = match selected_villager_key {
        Some(key) => world.villager(key),
        None => None,
    };

    let mut sprite_grid = SpriteGrid::new();

    for x in 0..8 {
        sprite_grid.big_circle_at(x, 0);
        sprite_grid.big_circle_at(x, 7);
    }

    for y in 0..8 {
        sprite_grid.big_circle_at(0, y);
        sprite_grid.big_circle_at(7, y);
    }

    let farm_coords: Vec<(u8, u8)> = world.farms.values().map(|v| world.coords[v.key]).collect();

    for (x, y) in farm_coords {
        sprite_grid.turnip_at(x, y);
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

        sprite_grid.lizard_at(x, y, color);
    }

    if let Some(villager) = selected_villager {
        let satiation = world.satiation[villager.key];

        for x in 1..7 {
            if satiation >= x {
                sprite_grid.turnip_at(x, 7);
            }
        }
    }

    for dm in world.death_markers.values() {
        let (x, y) = world.coords[dm.key];
        sprite_grid.skull_at(x, y);
    }

    sprite_grid
}
