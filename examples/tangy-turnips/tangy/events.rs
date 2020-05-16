use crate::tangy::{Coords, Direction, EnemyShipKey};

#[derive(Copy, Clone, Debug)]
pub enum WorldEvent {
    PlayerShipMoved(Direction),
    PlayerShipDied(Coords),
    EnemyShipsMoved,
    EnemyShipMoved(EnemyShipKey, Direction),
}

pub type WE = WorldEvent;
