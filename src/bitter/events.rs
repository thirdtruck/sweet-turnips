use crate::bitter::{Coords,Direction,EntityKey};

#[derive(Copy,Clone,Debug)]
pub enum WorldEvent {
    GravesCleared,
    FarmsCultivated,
    VillagersFarmed,
    VillagersMoved,
    FarmAdded(Coords),
    VillagerMoved(EntityKey, Direction),
    VillagerAte(EntityKey),
    VillagerHungered(EntityKey),
    VillagerHarvested(EntityKey),
    FarmGrew(EntityKey),
    FarmHarvested(EntityKey),
    VillagerDied(EntityKey),
}

pub type WE = WorldEvent;
