use crate::tangy::EntityKey;

#[derive(Copy, Clone, Debug)]
pub struct GameEntity;

#[derive(Copy, Clone, Debug)]
pub struct PlayerShip {
    pub key: EntityKey,
}

#[derive(Copy, Clone, Debug)]
pub struct EnemyShip {
    pub key: EntityKey,
}
