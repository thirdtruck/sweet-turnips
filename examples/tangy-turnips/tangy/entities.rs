use crate::tangy::{EntityKey, Ticks};

#[derive(Copy, Clone, Debug)]
pub struct GameEntity;

#[derive(Copy, Clone, Debug)]
pub struct PlayerShip {
    pub key: EntityKey,
}
