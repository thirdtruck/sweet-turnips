use crate::bitter::{Coords,Direction,EntityKey};

#[derive(Copy,Clone,Debug)]
pub enum WorldEvent {
    GravesCleared,
    FarmsCultivated,
    VillagersMoved,
    FarmAdded(Coords),
    VillagerMoved(EntityKey, Direction),
    VillagerAte(EntityKey),
    VillagersHungered,
    VillagerHungered(EntityKey),
    VillagerHarvested(EntityKey),
    FarmGrew(EntityKey),
    FarmHarvested(EntityKey),
    VillagerDied(EntityKey),
    EggLaid(Coords),
}

pub type WE = WorldEvent;
