use serde::{Deserialize, Serialize};

use std::path::PathBuf;

use sweet_turnips::default_game_config_setup;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct VillagerConfig {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FarmConfig {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CursorConfig {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WorldConfig {
    pub starting_villagers: Vec<VillagerConfig>,
    pub starting_farms: Vec<FarmConfig>,
    pub starting_cursor: CursorConfig,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GameConfig {
    pub world: WorldConfig,
}

fn example_game_config() -> GameConfig {
    let world_config = WorldConfig {
        starting_villagers: vec![VillagerConfig { x: 4, y: 4 }, VillagerConfig { x: 4, y: 5 }],
        starting_farms: vec![
            FarmConfig { x: 5, y: 4 },
            FarmConfig { x: 5, y: 5 },
            FarmConfig { x: 5, y: 6 },
        ],
        starting_cursor: CursorConfig { x: 2, y: 2 },
    };

    GameConfig {
        world: world_config,
    }
}

pub fn setup_game_config(config_path: PathBuf) -> GameConfig {
    default_game_config_setup(config_path, example_game_config())
}
