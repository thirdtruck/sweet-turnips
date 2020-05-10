use crate::tangy::{Coords, Direction, EntityKey};

#[derive(Copy, Clone, Debug)]
pub enum WorldEvent {
    PlayerShipMoved(Direction),
}

pub type WE = WorldEvent;
