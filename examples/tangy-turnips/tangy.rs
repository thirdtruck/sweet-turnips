mod entities;
mod events;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use slotmap::{new_key_type, SecondaryMap, SlotMap};

use entities::{EnemyShip, GameEntity, PlayerShip};
use events::{WorldEvent, WE};

pub const GRID_WIDTH: u8 = 8;
pub const GRID_HEIGHT: u8 = 8;

new_key_type! {
    pub struct EntityKey;

    pub struct PlayerShipKey;

    pub struct EnemyShipKey;
}

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
    pub player_ships: SlotMap<PlayerShipKey, PlayerShip>,
    pub enemy_ships: SlotMap<EnemyShipKey, EnemyShip>,
}

impl World {
    pub fn new() -> Self {
        World {
            entities: SlotMap::with_key(),
            coords: SecondaryMap::new(),
            events: vec![],
            ticks: 0,
            player_ships: SlotMap::with_key(),
            enemy_ships: SlotMap::with_key(),
        }
    }

    pub fn ticked(&self) -> Self {
        let world = self.clone();

        let world = Self {
            ticks: world.ticks + 1,
            ..world
        };

        world
            .with_event(WE::EnemyShipsMoved)
            .with_any_collisions()
            .with_events_processed()
    }

    fn with_any_collisions(&self) -> Self {
        let mut world = self.clone();

        let all_player_ship_coords: Vec<Coords> = world
            .player_ships
            .values()
            .map(|ps| world.coords[ps.key])
            .collect();

        for player_ship_coords in all_player_ship_coords {
            let mut death_coords: Option<Coords> = None;

            for enemy in world.enemy_ships.values() {
                let enemy_coords = world.coords[enemy.key];

                if player_ship_coords == enemy_coords {
                    death_coords = Some(enemy_coords);
                }
            }

            if let Some(coords) = death_coords {
                world = world.with_event(WE::PlayerShipDied(coords));
            }
        }

        world
    }

    pub fn with_events_processed(&self) -> Self {
        let mut world = self.clone();

        while world.events.len() > 0 {
            world = world.with_latest_event_processed();
        }

        world
    }

    pub fn with_player_ship_added_at(self, coords: Coords) -> Self {
        let mut world = self.clone();

        let key = world.entities.insert(GameEntity);

        let ship = PlayerShip { key };

        world.player_ships.insert(ship);
        world.coords.insert(key, coords);

        world
    }

    pub fn with_enemy_ship_added_at(self, coords: Coords) -> Self {
        let mut world = self.clone();

        let key = world.entities.insert(GameEntity);

        let ship = EnemyShip { key };

        world.enemy_ships.insert(ship);
        world.coords.insert(key, coords);

        world
    }

    pub fn with_enemy_ship_removed(self, ship_key: EnemyShipKey) -> Self {
        let mut world = self.clone();

        let ship = world.enemy_ships[ship_key];

        world.entities.remove(ship.key);
        world.coords.remove(ship.key);

        world.enemy_ships.remove(ship_key);

        world
    }

    pub fn with_event(self, evt: WorldEvent) -> Self {
        let mut events = self.events.clone();
        events.push(evt);

        Self { events, ..self }
    }

    fn with_player_ship_moved(&self, dir: Direction) -> Self {
        let mut world = self.clone();

        let keys_and_coords: Vec<(EntityKey, Coords)> = world
            .player_ships
            .values()
            .map(|ps| (ps.key, world.coords[ps.key]))
            .collect();

        for key_and_coords in keys_and_coords {
            let (key, coords) = (key_and_coords.0, key_and_coords.1);

            let (mut x, mut y) = coords;

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

            world.coords[key] = (x, y);
        }

        world
    }

    fn with_enemy_ship_moved(&self, key: EnemyShipKey, dir: Direction) -> Self {
        let mut world = self.clone();

        let ship = world.enemy_ships[key];

        let (mut x, mut y) = world.coords[ship.key];

        match dir {
            Direction::Up => {
                if y > 0 {
                    y -= 1
                }
            }
            Direction::Down => {
                if y < GRID_HEIGHT - 1 {
                    y += 1
                } else {
                    // They've scrolled off the screen
                    world = world.with_event(WE::EnemyShipRemoved(key));
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

        world.coords[ship.key] = (x, y);

        world
    }

    fn with_player_ship_death_at(&self, _coords: Coords) -> Self {
        let mut world = self.clone();

        let ship_keys: Vec<PlayerShipKey> = world.player_ships.keys().collect();

        for key in world.player_ships.keys() {
            let ship = world.player_ships[key];

            world.entities.remove(ship.key);
            world.coords.remove(ship.key);
        }

        for key in ship_keys.iter() {
            world.player_ships.remove(*key);
        }

        world
    }

    pub fn with_player_ship_move_requested(&self, dir: Direction) -> Self {
        self.clone().with_event(WE::PlayerShipMoved(dir))
    }

    fn with_latest_event_processed(self) -> Self {
        if self.events.len() == 0 {
            self
        } else {
            let mut world = self.clone();

            if let Some(event) = world.events.pop() {
                world.with_event_processed(event)
            } else {
                world
            }
        }
    }

    fn with_event_processed(self, event: WorldEvent) -> Self {
        match event {
            WE::EnemyShipsMoved => {
                let mut events = self.events.clone();
                for key in self.enemy_ships.keys() {
                    events.push(WE::EnemyShipMoved(key, Direction::Down));
                }
                Self { events, ..self }
            }
            WE::EnemyShipRemoved(key) => self.with_enemy_ship_removed(key),
            WE::PlayerShipDied(coords) => self.with_player_ship_death_at(coords),
            WE::EnemyShipMoved(key, dir) => self.with_enemy_ship_moved(key, dir),
            WE::PlayerShipMoved(dir) => self.with_player_ship_moved(dir),
        }
    }
}
