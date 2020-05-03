use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use slotmap::{new_key_type, SlotMap, SecondaryMap};

pub const GRID_WIDTH: u8 = 8;
pub const GRID_HEIGHT: u8 = 8;

new_key_type! { pub struct EntityKey; }

pub type EntityId = usize;
pub type Ticks = usize;

#[derive(Copy,Clone,Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Dir = Direction;

const CARDINAL_DIRECTIONS: [Direction; 4] = [
    Dir::Up,
    Dir::Down,
    Dir::Left,
    Dir::Right,
];

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

#[derive(Copy,Clone,Debug)]
struct GameEntity;

pub type Coords = (u8, u8);

pub struct World {
    events: Vec<WorldEvent>,
    entities: SlotMap<EntityKey, GameEntity>,
    pub coords: SecondaryMap<EntityKey, Coords>,
    last_id: EntityId,
    pub death_markers: SecondaryMap<EntityKey, DeathMarker>,
    pub farms: SecondaryMap<EntityKey, Farm>,
    ticks: Ticks,
    pub satiation: SecondaryMap<EntityKey, u8>,
    pub villagers: SecondaryMap<EntityKey, Villager>,
}

enum WorldEvent {
    AddFarm(Coords),
    VillagerMoved(EntityKey, Direction),
    VillagerAte(EntityKey),
    VillagerHungered(EntityKey),
    FarmGrew(EntityKey),
    FarmHarvested(EntityKey),
    VillagerDied(EntityKey),
}

type WE = WorldEvent;

impl World {
    pub fn new() -> Self {
        let mut world = World {
            entities: SlotMap::with_key(),
            coords: SecondaryMap::new(),
            events: vec![],
            last_id: 0,
            ticks: 0,
            death_markers: SecondaryMap::new(),
            farms: SecondaryMap::new(),
            satiation: SecondaryMap::new(),
            villagers: SecondaryMap::new(),
        };

        world.add_villager_at(4, 4);

        world.add_farm_at(5, 5);

        world
    }

    fn process_events(&mut self) {
        let mut new_events: Vec<WorldEvent> = vec![];

        for evt in self.events.drain(..) {
            match evt {
                WE::VillagerMoved(key, dir) => {
                    let c = self.coords[key];
                    self.coords[key] = coords_after_move(c, dir);
                },
                WE::VillagerAte(key) => {
                    self.satiation[key] += 1;

                    let mut villager = self.villagers[key];
                    villager.last_ate = self.ticks;
                    self.villagers[key] = villager;
                },
                WE::VillagerHungered(key) => {
                    if self.satiation[key] > 0 {
                        self.satiation[key] -= 1;
                    }

                    let mut villager = self.villagers[key];
                    villager.last_ate = self.ticks;
                    self.villagers[key] = villager;
                },
                WE::FarmGrew(key) => {
                    let mut farm = self.farms[key];
                    farm.last_grew = self.ticks;
                    self.farms[key] = farm;

                    let farm_coords = self.coords[key];

                    let mut all_possible_coords: Vec<Coords> = vec![];

                    for dir in CARDINAL_DIRECTIONS.iter() {
                        if can_move_in_dir(farm_coords, *dir) {
                            let new_coords = coords_after_move(farm_coords, *dir);
                            all_possible_coords.push(new_coords);
                        }
                    }

                    for key in self.farms.keys() {
                        let occupied_coords = self.coords[key];

                        all_possible_coords.retain(|c| *c != occupied_coords);

                        if all_possible_coords.is_empty() {
                            return;
                        }
                    }

                    let mut rng = rand::thread_rng();

                    let ci = rng.gen_range(0, all_possible_coords.len());

                    let new_coords = all_possible_coords[ci];

                    new_events.push(WE::AddFarm(new_coords));
                }
                WE::FarmHarvested(key) => {
                    self.farms.remove(key);
                }
                WE::VillagerDied(vk) => {
                    let coords = self.coords[vk];

                    let dmk = self.entities.insert(GameEntity);

                    let dm = DeathMarker { key: dmk };

                    self.death_markers.insert(dmk, dm);
                    self.coords.insert(dmk, coords);

                    self.villagers.remove(vk);
                }
                WE::AddFarm(coords) => {
                    let (x, y) = (coords.0, coords.1);

                    let new_id = self.last_id + 1;

                    let entity = GameEntity;
                    let key = self.entities.insert(entity);

                    let farm = Farm::new(new_id, key, x, y, self.ticks);

                    self.last_id = new_id;

                    self.farms.insert(key, farm);
                    self.coords.insert(key, (x, y));

                    self.last_id = new_id;
                }
            }
        }

        self.events = new_events;
    }

    pub fn add_villager_at(&mut self, x: u8, y: u8) -> EntityId {
        let new_id = self.last_id + 1;

        let entity = GameEntity;
        let key = self.entities.insert(entity);

        let villager = Villager::new(new_id, key, self.ticks);

        self.villagers.insert(key, villager);
        self.coords.insert(key, (x, y));
        self.satiation.insert(key, 1);

        self.last_id = new_id;

        new_id
    }

    pub fn add_farm_at(&mut self, x: u8, y: u8) -> EntityId {
        let new_id = self.last_id + 1;

        let entity = GameEntity;
        let key = self.entities.insert(entity);

        let farm = Farm::new(new_id, key, x, y, self.ticks);

        self.last_id = new_id;

        self.farms.insert(key, farm);
        self.coords.insert(key, (x, y));

        self.last_id = new_id;

        new_id
    }

    pub fn tick(&mut self) {
        self.ticks += 1;

        let mut rng = rand::thread_rng();

        if (self.ticks + 1) % 80 == 0 {
            self.death_markers.clear();

            for farm in self.farms.values() {
                if self.ticks - farm.last_grew > 20 {
                    self.events.push(WE::FarmGrew(farm.key));
                }
            }

            self.process_events();

            for (vk, villager) in self.villagers.iter() {
                let satiation = self.satiation[vk];
                let mut unharvested_farms: Vec<&Farm> = self.farms.values().collect();

                if self.ticks - villager.last_ate > 40 && satiation > 0 {
                    if unharvested_farms.len() > 0 && satiation < 5 {
                        let farm_to_eat_index = rng.gen_range(0, unharvested_farms.len());
                        let farm = unharvested_farms.remove(farm_to_eat_index);

                        self.events.push(WE::FarmHarvested(farm.key));
                        self.events.push(WE::VillagerAte(vk));
                    } else {
                        self.events.push(WE::VillagerHungered(vk));
                    }

                }
            }

            self.process_events();

            for key in self.villagers.keys() {
                if self.satiation[key] == 0 {
                    self.events.push(WE::VillagerDied(key));
                }
            }

            self.process_events();

            for key in self.villagers.keys() {
                let direction: Direction = rand::random();

                self.events.push(WE::VillagerMoved(key, direction));
            }

            self.process_events();
        }
    }

    pub fn villager_id_at(&self, x: u8, y: u8) -> Option<EntityId> {
        for v in self.villagers.values() {
            let (vx, vy) = self.coords[v.key];
            if vx == x && vy == y {
                return Some(v.id);
            }
        }

        None
    }

    pub fn villager(&self, id: EntityId) -> Option<Villager> {
        for v in self.villagers.values() {
            if v.id == id {
                return Some(v.clone());
            }
        }

        None
    }
}

fn coords_after_move(coords: Coords, dir: Direction) -> Coords {
    let (mut x, mut y) = (coords.0, coords.1);

    match dir {
        // Remember to account for the border

        Direction::Up => {
            if y > 1 {
                y -= 1;
            }
        },
        Direction::Down => {
            if y < GRID_HEIGHT - 1 {
                y += 1;
            }
        },
        Direction::Left => {
            if x > 1 {
                x -= 1;
            }
        },
        Direction::Right => {
            if x < GRID_WIDTH - 1 {
                x += 1;
            }
        },
    }

    (x, y)
}

fn can_move_in_dir(coords: Coords, dir: Direction) -> bool {
    let (x, y) = (coords.0, coords.1);

    match dir {
        Dir::Up => y > 1,
        Dir::Down => y < GRID_HEIGHT + 1,
        Dir::Left => x > 1,
        Dir::Right => x < GRID_HEIGHT + 1,
    }
}

#[derive(Copy,Clone,Debug)]
pub struct DeathMarker {
    pub key: EntityKey,
}

#[derive(Copy,Clone,Debug)]
pub struct Villager {
    pub id: EntityId,
    pub key: EntityKey,
    pub last_ate: Ticks,
}

impl Villager {
    pub fn new(id: EntityId, key: EntityKey, now: Ticks) -> Self {
        Villager {
            id,
            key,
            last_ate: now,
        }
    }
}

#[derive(Copy,Clone,Debug)]
pub struct Farm {
    pub id: EntityId,
    pub key: EntityKey,
    pub last_grew: Ticks,
    pub x: u8,
    pub y: u8,
}

impl Farm {
    pub fn new(id: EntityId, key: EntityKey, x: u8, y: u8, now: Ticks) -> Self {
        Farm {
            id,
            key,
            last_grew: now,
            x,
            y,
        }
    }
}
