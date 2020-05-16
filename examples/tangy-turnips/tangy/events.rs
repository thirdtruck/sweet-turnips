use crate::tangy::Direction;

#[derive(Copy, Clone, Debug)]
pub enum WorldEvent {
    PlayerShipMoved(Direction),
}

pub type WE = WorldEvent;
