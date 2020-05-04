use crate::bitter::{
    EntityKey,
    EntityId,
    Ticks,
};

#[derive(Copy,Clone,Debug)]
pub struct GameEntity;

#[derive(Copy,Clone,Debug)]
pub struct DeathMarker {
    pub key: EntityKey,
}

#[derive(Copy,Clone,Debug)]
pub struct Villager {
    pub id: EntityId,
    pub key: EntityKey,
    pub last_ate: Ticks,
}

impl Villager {
    pub fn new(id: EntityId, key: EntityKey, now: Ticks) -> Self {
        Villager {
            id,
            key,
            last_ate: now,
        }
    }
}

#[derive(Copy,Clone,Debug)]
pub struct Farm {
    pub key: EntityKey,
    pub last_grew: Ticks,
    pub x: u8,
    pub y: u8,
}

impl Farm {
    pub fn new(key: EntityKey, x: u8, y: u8, now: Ticks) -> Self {
        Farm {
            key,
            last_grew: now,
            x,
            y,
        }
    }
}
