use crate::tangy::{Coords, Direction, EnemyShipKey};

#[derive(Copy, Clone, Debug)]
pub enum WorldEvent {
    PlayerShipMoved(Direction),
    PlayerShipDied(Coords),
    PlayerBulletFired(Coords),
    EnemyShipsMoved,
    EnemyShipMoved(EnemyShipKey, Direction),
    EnemyShipRemoved(EnemyShipKey),
}

pub type WE = WorldEvent;
