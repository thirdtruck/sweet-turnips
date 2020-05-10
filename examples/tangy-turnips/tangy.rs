mod entities;
mod events;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use slotmap::{new_key_type, SecondaryMap, SlotMap};

use entities::{GameEntity, PlayerShip};
use events::{WorldEvent, WE};

pub const GRID_WIDTH: u8 = 8;
pub const GRID_HEIGHT: u8 = 8;

new_key_type! { pub struct EntityKey; }

pub type Ticks = usize;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0, 4) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            _ => Direction::Right,
        }
    }
}

pub type Coords = (u8, u8);

#[derive(Clone, Debug)]
pub struct World {
    events: Vec<WorldEvent>,
    entities: SlotMap<EntityKey, GameEntity>,
    pub coords: SecondaryMap<EntityKey, Coords>,
    pub ticks: Ticks,
    pub player_ships: SecondaryMap<EntityKey, PlayerShip>,
}

impl World {
    pub fn new() -> Self {
        World {
            entities: SlotMap::with_key(),
            coords: SecondaryMap::new(),
            events: vec![],
            ticks: 0,
            player_ships: SecondaryMap::new(),
        }
    }

    pub fn ticked(&self) -> Self {
        let world = self.clone();

        let world = Self {
            ticks: world.ticks + 1,
            ..world
        };

        world.events_processed()
    }

    pub fn events_processed(&self) -> Self {
        let mut world = self.clone();

        while let Some(evt) = world.events.pop() {
            let new_events = match evt {
                WE::PlayerShipMoved(dir) => world.player_ship_moved(dir),
            };

            world.events.extend(new_events);
        }

        world
    }

    pub fn with_player_ship_at(self, coords: Coords) -> Self {
        let mut world = self.clone();

        let key = world.entities.insert(GameEntity);

        let ship = PlayerShip { key };

        world.player_ships.insert(key, ship);
        world.coords.insert(key, coords);

        world
    }

    pub fn with_event(self, evt: WorldEvent) -> Self {
        let mut events = self.events.clone();
        events.push(evt);

        Self { events, ..self }
    }

    fn player_ship_moved(&mut self, dir: Direction) -> Vec<WorldEvent> {
        // We assume there's one and only one player ship for convenience
        let player_ship = self.player_ships.values().nth(0).expect("Found no player ship");
        let (mut x, mut y) = self.coords[player_ship.key];

        match dir {
            Direction::Up => {
                if y > 0 {
                    y -= 1
                }
            }
            Direction::Down => {
                if y < GRID_HEIGHT - 1 {
                    y += 1
                }
            }
            Direction::Left => {
                if x > 1 {
                    x -= 1
                }
            }
            Direction::Right => {
                if x < GRID_WIDTH - 2 {
                    x += 1
                }
            }
        };

        self.coords[player_ship.key] = (x, y);

        vec![]
    }

    pub fn with_player_ship_moved(&self, dir: Direction) -> Self {
        self.clone().with_event(WE::PlayerShipMoved(dir))
    }
}
