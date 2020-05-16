use crate::tangy::{Direction, EntityKey};

#[derive(Copy, Clone, Debug)]
pub enum WorldEvent {
    PlayerShipMoved(Direction),
    EnemyShipsMoved,
    EnemyShipMoved(EntityKey, Direction),
}

pub type WE = WorldEvent;
