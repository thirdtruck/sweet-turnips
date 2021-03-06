mod entities;
mod events;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use slotmap::{new_key_type, SecondaryMap, SlotMap};

use entities::{EnemyShip, GameEntity, PlayerBullet, PlayerShip};
use events::{WorldEvent, WE};

pub const GRID_WIDTH: u8 = 8;
pub const GRID_HEIGHT: u8 = 8;

new_key_type! {
    pub struct EntityKey;

    pub struct PlayerShipKey;

    pub struct PlayerBulletKey;

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
    pub player_bullets: SlotMap<PlayerBulletKey, PlayerBullet>,
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
            player_bullets: SlotMap::with_key(),
            enemy_ships: SlotMap::with_key(),
        }
    }

    pub fn ticked(&self) -> Self {
        let world = self.clone();

        let world = Self {
            ticks: world.ticks + 1,
            ..world
        };

        let world = world
            .with_event(WE::PlayerBulletsMoved)
            .with_any_collisions()
            .with_events_processed();

        let world = if world.ticks % 2 == 0 {
            // On every other tick
            world
                .with_event(WE::EnemyShipsMoved)
                .with_any_collisions()
                .with_events_processed()
        } else {
            world
        };

        world
    }

    fn with_any_collisions(&self) -> Self {
        let mut world = self.clone();

        let mut bullet_keys_to_remove: Vec<PlayerBulletKey> = vec![];
        let mut enemy_ship_keys_to_remove: Vec<EnemyShipKey> = vec![];

        for bullet_key in world.player_bullets.keys() {
            let bullet = world.player_bullets[bullet_key];
            let bullet_coords = world.coords[bullet.key];

            for enemy_key in world.enemy_ships.keys() {
                let enemy = world.enemy_ships[enemy_key];
                let enemy_coords = world.coords[enemy.key];

                if bullet_coords == enemy_coords {
                    bullet_keys_to_remove.push(bullet_key);
                    enemy_ship_keys_to_remove.push(enemy_key);
                }
            }
        }

        for key in bullet_keys_to_remove.iter() {
            world = world.with_event(WE::PlayerBulletRemoved(*key));
        }

        for key in enemy_ship_keys_to_remove.iter() {
            world = world.with_event(WE::EnemyShipRemoved(*key));
        }

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

    pub fn with_player_bullet_removed(self, bullet_key: PlayerBulletKey) -> Self {
        let mut world = self.clone();

        let bullet = world.player_bullets[bullet_key];

        world.entities.remove(bullet.key);
        world.coords.remove(bullet.key);

        world.player_bullets.remove(bullet_key);

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

    fn with_player_bullet_moved(&self, key: PlayerBulletKey, dir: Direction) -> Self {
        let mut world = self.clone();

        let ship = world.player_bullets[key];

        let (mut x, mut y) = world.coords[ship.key];

        match dir {
            Direction::Up => {
                if y > 0 {
                    y -= 1
                } else {
                    // They've scrolled off the screen
                    world = world.with_event(WE::PlayerBulletRemoved(key));
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

        world.coords[ship.key] = (x, y);

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

    pub fn with_player_bullets_fired(&self) -> Self {
        let mut world = self.clone();

        for ship in self.player_ships.values() {
            let coords = world.coords[ship.key];
            world = world.with_event(WE::PlayerBulletFired(coords));
        }

        world
    }

    fn with_player_bullet_fired_from(&self, coords: Coords) -> Self {
        let mut world = self.clone();

        let (x, y) = coords;

        if y < 2 {
            // Don't bother spawning a bullet if it would just disappear off the top of the screen
            return world
        }

        let y = y - 1;

        let key = world.entities.insert(GameEntity);

        let bullet = PlayerBullet { key };

        world.player_bullets.insert(bullet);
        world.coords.insert(key, (x, y));

        world
    }

    fn with_latest_event_processed(self) -> Self {
        if self.events.len() == 0 {
            self
        } else {
            let mut events = self.events.clone();

            if let Some(event) = events.pop() {
                Self { events, ..self }.with_event_processed(event)
            } else {
                self
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
            WE::PlayerBulletsMoved => {
                let mut events = self.events.clone();
                for key in self.player_bullets.keys() {
                    events.push(WE::PlayerBulletMoved(key, Direction::Up));
                }
                Self { events, ..self }
            }
            WE::EnemyShipRemoved(key) => self.with_enemy_ship_removed(key),
            WE::PlayerShipDied(coords) => self.with_player_ship_death_at(coords),
            WE::EnemyShipMoved(key, dir) => self.with_enemy_ship_moved(key, dir),
            WE::PlayerShipMoved(dir) => self.with_player_ship_moved(dir),
            WE::PlayerBulletFired(coords) => self.with_player_bullet_fired_from(coords),
            WE::PlayerBulletMoved(key, coords) => self.with_player_bullet_moved(key, coords),
            WE::PlayerBulletRemoved(key) => self.with_player_bullet_removed(key),
        }
    }
}
