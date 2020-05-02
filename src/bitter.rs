use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

pub const GRID_WIDTH: u8 = 8;
pub const GRID_HEIGHT: u8 = 8;

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
    last_id: EntityId,
    pub villagers: Vec<Villager>,
    pub death_markers: Vec<DeathMarker>,
}

impl World {
    pub fn new() -> Self {
        let starting_id = 0;
        let ticks = 0;

        World {
            last_id: 0,
            villagers: vec![Villager::new(starting_id, ticks)],
            death_markers: vec![],
        }
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
    pub fn new(id: EntityId, now: Ticks) -> Self {
        Villager {
            id: id,
            satiation: 3,
            last_ate: now,
            x: 4,
            y: 4,
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
