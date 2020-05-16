use crate::tangy::{Direction, EnemyShipKey};

#[derive(Copy, Clone, Debug)]
pub enum WorldEvent {
    PlayerShipMoved(Direction),
    EnemyShipsMoved,
    EnemyShipMoved(EnemyShipKey, Direction),
}

pub type WE = WorldEvent;
