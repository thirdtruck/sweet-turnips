mod entities;
mod events; 

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use slotmap::{new_key_type, SlotMap, SecondaryMap};

use entities::{
    DeathMarker,
    Farm,
    GameEntity,
    Villager,
};
use events::{WE,WorldEvent};

pub const GRID_WIDTH: u8 = 8;
pub const GRID_HEIGHT: u8 = 8;

new_key_type! { pub struct EntityKey; }

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

pub type Coords = (u8, u8);

#[derive(Clone,Debug)]
pub struct World {
    events: Vec<WorldEvent>,
    entities: SlotMap<EntityKey, GameEntity>,
    pub coords: SecondaryMap<EntityKey, Coords>,
    pub death_markers: SecondaryMap<EntityKey, DeathMarker>,
    pub farms: SecondaryMap<EntityKey, Farm>,
    ticks: Ticks,
    pub satiation: SecondaryMap<EntityKey, u8>,
    pub villagers: SecondaryMap<EntityKey, Villager>,
}

impl World {
    pub fn new() -> Self {
        let mut world = World {
            entities: SlotMap::with_key(),
            coords: SecondaryMap::new(),
            events: vec![],
            ticks: 0,
            death_markers: SecondaryMap::new(),
            farms: SecondaryMap::new(),
            satiation: SecondaryMap::new(),
            villagers: SecondaryMap::new(),
        };

        world.add_villager_at(4, 4);
        world.add_villager_at(4, 5);

        world.add_farm_at(5, 4);
        world.add_farm_at(5, 5);
        world.add_farm_at(5, 6);

        world
    }

    fn copy_from_previous_world(&mut self, new_world: Self) {
        self.entities = new_world.entities.clone();
        self.coords = new_world.coords.clone();
        self.events = new_world.events.clone();
        self.ticks = new_world.ticks.clone();
        self.death_markers = new_world.death_markers.clone();
        self.farms = new_world.farms.clone();
        self.satiation = new_world.satiation.clone();
        self.villagers = new_world.villagers.clone();
    }

    fn process_events(&self) -> World {
        let mut new_world = self.clone();

        while let Some(evt) = new_world.events.pop() {
            let new_events = match evt {
                WE::VillagerMoved(key, dir) => new_world.villager_moved(key, dir),
                WE::VillagerAte(key) => new_world.villager_ate(key),
                WE::VillagersHungered => new_world.villagers_hungered(),
                WE::VillagerHungered(key) => new_world.villager_hungered(key),
                WE::FarmGrew(key) => new_world.farm_grew(key),
                WE::FarmHarvested(key) => new_world.farm_harvested(key),
                WE::VillagerDied(vk) => new_world.villager_died(vk),
                WE::FarmAdded(coords) => new_world.farm_added(coords),
                WE::VillagerHarvested(vk) => new_world.villager_harvested(vk),
                WE::GravesCleared => new_world.graves_cleared(),
                WE::FarmsCultivated => new_world.farms_cultivated(),
                WE::VillagersMoved => new_world.villagers_moved(),
                WE::EggLaid(coords) => new_world.egg_laid(coords),
            };

            new_world.events.extend(new_events);
        }

        new_world
    }

    fn villager_moved(&mut self, key: EntityKey, dir: Direction) -> Vec<WorldEvent>{
        let c = self.coords[key];
        self.coords[key] = coords_after_move(c, dir);

        vec![]
    }

    fn villager_ate(&mut self, key: EntityKey) -> Vec<WorldEvent> {
        let mut villager = self.villagers[key];
        villager.last_ate = self.ticks;
        self.villagers[key] = villager;

        self.satiation[key] += 1;

        vec![]
    }

    fn villagers_hungered(&mut self) -> Vec<WorldEvent> {
        let mut new_events = vec![];

        for key in self.villagers.keys() {
            new_events.push(WE::VillagerHungered(key));
        }

        new_events
    }

    fn villager_hungered(&mut self, key: EntityKey) -> Vec<WorldEvent> {
        let mut new_events = vec![];

        let mut villager = self.villagers[key];

        let feel_hunger = self.ticks - villager.last_ate > 4;

        if !feel_hunger {
            return new_events;
        }

        if self.satiation[key] > 0 {
            self.satiation[key] -= 1;

            villager.last_ate = self.ticks;
            self.villagers[key] = villager;

            new_events.push(WE::VillagerHarvested(key));
        } else {
            new_events.push(WE::VillagerDied(key));
        }

        new_events
    }

    fn farm_grew(&mut self, key: EntityKey) -> Vec<WorldEvent> {
        let mut new_events = vec![];

        let mut farm = self.farms[key];

        let ready_to_grow = self.ticks - farm.last_grew > 3;

        if ! ready_to_grow {
            return new_events;
        }

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
            if all_possible_coords.is_empty() {
                continue;
            }

            let occupied_coords = self.coords[key];

            all_possible_coords.retain(|c| *c != occupied_coords);

        }

        if all_possible_coords.len() == 0 {
            return new_events;
        }

        let mut rng = rand::thread_rng();

        let ci = rng.gen_range(0, all_possible_coords.len());

        let new_coords = all_possible_coords[ci];

        new_events.push(WE::FarmAdded(new_coords));

        new_events
    }

    fn farm_harvested(&mut self, key: EntityKey) -> Vec<WorldEvent> {
        self.farms.remove(key);

        vec![]
    }

    fn villager_died(&mut self, vk: EntityKey) -> Vec<WorldEvent> {
        let coords = self.coords[vk];

        let dmk = self.entities.insert(GameEntity);

        let dm = DeathMarker { key: dmk };

        self.death_markers.insert(dmk, dm);
        self.coords.insert(dmk, coords);

        self.villagers.remove(vk);

        vec![]
    }

    fn farm_added(&mut self, coords: Coords) -> Vec<WorldEvent> {
        let (x, y) = (coords.0, coords.1);

        let entity = GameEntity;
        let key = self.entities.insert(entity);

        let farm = Farm::new(key, x, y, self.ticks);

        self.farms.insert(key, farm);
        self.coords.insert(key, (x, y));

        vec![]
    }

    fn villager_harvested(&mut self, vk: EntityKey) -> Vec<WorldEvent> {
        let mut new_events = vec![];

        let mut rng = rand::thread_rng();

        let mut unharvested_farms: Vec<&Farm> = self.farms.values().collect();

        let food_left_to_eat = unharvested_farms.len() > 0;

        if food_left_to_eat {
            let farm_to_eat_index = rng.gen_range(0, unharvested_farms.len());
            let farm = unharvested_farms.remove(farm_to_eat_index);

            new_events.push(WE::FarmHarvested(farm.key));
            new_events.push(WE::VillagerAte(vk));
        }

        new_events
    }

    fn graves_cleared(&mut self) -> Vec<WorldEvent> {
        self.death_markers.clear();

        vec![]
    }

    fn farms_cultivated(&mut self) -> Vec<WorldEvent> {
        let mut new_events = vec![];

        for farm in self.farms.values() {
            new_events.push(WE::FarmGrew(farm.key));
        }

        new_events
    }

    fn villagers_moved(&mut self) -> Vec<WorldEvent> {
        let mut new_events = vec![];

        for key in self.villagers.keys() {
            let direction: Direction = rand::random();

            new_events.push(WE::VillagerMoved(key, direction));
        }

        new_events
    }

    fn egg_laid(&mut self, coords: Coords) -> Vec<WorldEvent> {
        let (x, y) = (coords.0, coords.1);

        let entity = GameEntity;
        let key = self.entities.insert(entity);

        let villager = Villager::new(key, self.ticks);

        self.villagers.insert(key, villager);
        self.coords.insert(key, (x, y));
        self.satiation.insert(key, 4);

        vec![]
    }

    pub fn add_villager_at(&mut self, x: u8, y: u8) {
        let entity = GameEntity;
        let key = self.entities.insert(entity);

        let villager = Villager::new(key, self.ticks);

        self.villagers.insert(key, villager);
        self.coords.insert(key, (x, y));
        self.satiation.insert(key, 4);
    }

    pub fn add_farm_at(&mut self, x: u8, y: u8) {
        let entity = GameEntity;
        let key = self.entities.insert(entity);

        let farm = Farm::new(key, x, y, self.ticks);

        self.farms.insert(key, farm);
        self.coords.insert(key, (x, y));
    }

    pub fn tick(&mut self) {
        self.ticks += 1;

        // self.events is a LIFO stack
        self.events.push(WE::VillagersMoved);
        self.events.push(WE::VillagersHungered);
        self.events.push(WE::FarmsCultivated);
        self.events.push(WE::GravesCleared);

        let new_world = self.process_events();

        self.copy_from_previous_world(new_world);
    }

    pub fn request_egg_spawn(&mut self, coords: Coords) {
        self.events.push(WE::EggLaid(coords));
    }

    pub fn villager_key_at(&self, x: u8, y: u8) -> Option<EntityKey> {
        for v in self.villagers.values() {
            let (vx, vy) = self.coords[v.key];
            if vx == x && vy == y {
                return Some(v.key);
            }
        }

        None
    }

    pub fn villager(&self, key: EntityKey) -> Option<&Villager> {
        self.villagers.get(key)
    }
}

fn coords_after_move(coords: Coords, dir: Direction) -> Coords {
    let (mut x, mut y) = (coords.0, coords.1);

    // Remember to account for the border
    match dir {
        Direction::Up => {
            if y > 1 {
                y -= 1;
            }
        },
        Direction::Down => {
            if y < GRID_HEIGHT - 2 {
                y += 1;
            }
        },
        Direction::Left => {
            if x > 1 {
                x -= 1;
            }
        },
        Direction::Right => {
            if x < GRID_WIDTH - 2 {
                x += 1;
            }
        },
    }

    (x, y)
}

fn can_move_in_dir(coords: Coords, dir: Direction) -> bool {
    let (x, y) = (coords.0, coords.1);

    // Remember to account for the border
    match dir {
        Dir::Up => y > 1,
        Dir::Down => y < GRID_HEIGHT - 2,
        Dir::Left => x > 1,
        Dir::Right => x < GRID_WIDTH - 2,
    }
}
