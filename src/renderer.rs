use crate::bitter::{EntityId, World};
use crate::sprites::SpriteGrid;

pub fn sprite_grid_from_world(world: &World, selected_villager_id: Option<EntityId>) -> SpriteGrid {
    let selected_villager = match selected_villager_id {
        Some(id) => world.villager(id),
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

    let farm_coords: Vec<(u8, u8)> = world.farms.iter().map(|v| (v.x, v.y)).collect();

    for (x, y) in farm_coords {
        sprite_grid.turnip_at(x, y);
    }

    let villager_coords: Vec<(u8, u8)> = world.villagers.values().map(|v| world.coords[v.key]).collect();

    for (x, y) in villager_coords {
        sprite_grid.lizard_at(x, y);
    }

    if let Some(villager) = selected_villager {
        let satiation = world.satiation[villager.key];

        for x in 1..7 {
            if satiation >= x {
                sprite_grid.turnip_at(x, 7);
            }
        }
    }

    let death_marker_coords: Vec<(u8, u8)> = world.death_markers.iter().map(|dm| (dm.x, dm.y)).collect();

    for (x, y) in death_marker_coords {
        sprite_grid.skull_at(x, y);
    }

    sprite_grid
}
