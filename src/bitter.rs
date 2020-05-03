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

pub struct World {
    events: Vec<WorldEvent>,
    last_id: EntityId,
    pub villagers: Vec<Villager>,
    pub death_markers: Vec<DeathMarker>,
    pub farms: Vec<Farm>,
    ticks: Ticks,
    pub satiation: SecondaryMap<EntityKey, u8>,
    pub villagers_map: SlotMap<EntityKey, Villager>,
}

enum WorldEvent {
    HarvestFarm(EntityId),
}

type WE = WorldEvent;

impl World {
    pub fn new() -> Self {
        let mut world = World {
            events: vec![],
            last_id: 0,
            ticks: 0,
            villagers: vec![],
            death_markers: vec![],
            farms: vec![],
            satiation: SecondaryMap::new(),
            villagers_map: SlotMap::with_key(),
        };

        world.add_villager_at(4, 4);

        world.add_farm_at(5, 5);

        world
    }

    fn process_events(&mut self) {
        for evt in self.events.drain(..) {
            match evt {
                WE::HarvestFarm(id) => {
                    self.farms.retain(|f| !f.id == id);
                }
            }
        }
    }

    pub fn add_villager_at(&mut self, x: u8, y: u8) -> EntityId {
        let new_id = self.last_id + 1;

        let villager = Villager::new(new_id, x, y, self.ticks);

        self.last_id = new_id;

        self.villagers.push(villager);

        let key = self.villagers_map.insert(villager);
        self.satiation.insert(key, 3);

        new_id
    }

    pub fn add_farm_at(&mut self, x: u8, y: u8) -> EntityId {
        let new_id = self.last_id + 1;

        let farm = Farm::new(new_id, x, y, self.ticks);

        self.last_id = new_id;

        self.farms.push(farm);

        new_id
    }

    pub fn tick(&mut self) {
        let mut rng = rand::thread_rng();

        self.ticks += 1;

        if (self.ticks + 1) % 80 == 0 {
            self.death_markers.clear();

            let mut new_farm_coords: Vec<(u8, u8)> = vec![];

            for farm in self.farms.iter_mut() {
                if self.ticks - farm.last_grew > 20 {
                    farm.last_grew = self.ticks;

                    let direction: Direction = rand::random();
                    let coords = match direction {
                        Direction::Up if farm.y > 0 => Some((farm.x, farm.y - 1)),
                        Direction::Down if farm.y < GRID_HEIGHT => Some((farm.x, farm.y + 1)),
                        Direction::Left if farm.x > 0 => Some((farm.x - 1, farm.y)),
                        Direction::Right if farm.x < GRID_WIDTH => Some((farm.x + 1, farm.y)),
                        _ => None
                    };

                    if let Some(c) = coords {
                        new_farm_coords.push(c);
                    }
                }
            }

            for (x, y) in new_farm_coords {
                if !(x >= GRID_WIDTH || y >= GRID_HEIGHT) { 
                    self.add_farm_at(x, y);
                }
            }

            for (key, villager) in self.villagers_map.iter() {
                let satiation = self.satiation[key];

                if self.ticks - villager.last_ate > 40 && satiation > 0 {
                    if self.farms.len() > 0 && satiation < 5 {
                        let farm_to_eat_index = rng.gen_range(0, self.farms.len());
                        let farm = &self.farms[farm_to_eat_index];
                        self.events.push(WE::HarvestFarm(farm.id));

                        self.satiation[key] = satiation + 1;
                    } else {
                        self.satiation[key] = satiation - 1;
                    }

                    //villager.last_ate = self.ticks;

                    println!("Ate. Now: {}", satiation);
                }
            }

            self.process_events();

            for villager in self.villagers.iter_mut() {
                if villager.satiation == 0 {
                    let death_marker = DeathMarker {
                        x: villager.x,
                        y: villager.y,
                    };
                    self.death_markers.push(death_marker);

                    continue;
                }

                let direction: Direction = rand::random();
                villager.step(direction);
            }
        }

        self.villagers.retain(|v| v.satiation > 0);
    }

    pub fn villager_id_at(&self, x: u8, y: u8) -> Option<EntityId> {
        for v in self.villagers.iter() {
            if v.x == x && v.y == y {
                return Some(v.id);
            }
        }

        None
    }

    pub fn villager(&self, id: EntityId) -> Option<Villager> {
        for v in self.villagers.iter() {
            if v.id == id {
                return Some(v.clone());
            }
        }

        None
    }
}

pub struct DeathMarker {
    pub x: u8,
    pub y: u8,
}

#[derive(Copy,Clone)]
pub struct Villager {
    pub id: EntityId,
    pub satiation: u8,
    pub last_ate: Ticks,
    pub x: u8,
    pub y: u8,
}

impl Villager {
    pub fn new(id: EntityId, x: u8, y: u8, now: Ticks) -> Self {
        Villager {
            id,
            satiation: 3,
            last_ate: now,
            x,
            y,
        }
    }

    pub fn step(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                if self.y > 0 {
                    self.y -= 1;
                }
            },
            Direction::Down => {
                if self.y < GRID_HEIGHT {
                    self.y += 1;
                }
            },
            Direction::Left => {
                if self.x > 0 {
                    self.x -= 1;
                }
            },
            Direction::Right => {
                if self.x < GRID_WIDTH {
                    self.x += 1;
                }
            },
        }
    }
}

pub struct Farm {
    pub id: EntityId,
    pub last_grew: Ticks,
    pub x: u8,
    pub y: u8,
}

impl Farm {
    pub fn new(id: EntityId, x: u8, y: u8, now: Ticks) -> Self {
        Farm {
            id,
            last_grew: now,
            x,
            y,
        }
    }
}
