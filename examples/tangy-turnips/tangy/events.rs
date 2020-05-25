use crate::tangy::{Coords, Direction, EnemyShipKey, PlayerBulletKey};

#[derive(Copy, Clone, Debug)]
pub enum WorldEvent {
    PlayerShipMoved(Direction),
    PlayerShipDied(Coords),
    PlayerBulletFired(Coords),
    PlayerBulletRemoved(PlayerBulletKey),
    EnemyShipsMoved,
    EnemyShipMoved(EnemyShipKey, Direction),
    EnemyShipRemoved(EnemyShipKey),
}

pub type WE = WorldEvent;
