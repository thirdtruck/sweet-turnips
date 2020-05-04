use serde::{Serialize, Deserialize};
use serde_yaml;

use std::error;
use std::fmt;
use std::fs::File;
use std::io;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, CouldNotSetUpGameConfig>;

#[derive(Debug, Clone)]
pub struct CouldNotSetUpGameConfig;

impl fmt::Display for CouldNotSetUpGameConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not read and/or write and/or parse the config file")
    }
}

impl error::Error for CouldNotSetUpGameConfig {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct VillagerConfig {
    x: u8,
    y: u8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FarmConfig {
    x: u8,
    y: u8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WorldConfig {
    starting_villagers: Vec<VillagerConfig>,
    starting_farms: Vec<FarmConfig>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GameConfig {
    world: WorldConfig,
}

fn example_game_config() -> GameConfig {
    let world_config = WorldConfig {
        starting_villagers: vec![
            VillagerConfig { x: 2, y: 2 },
        ],
        starting_farms: vec![
            FarmConfig { x: 3, y: 3 },
        ],
    };

    GameConfig { world: world_config }
}

pub fn setup_game_config(config_path: PathBuf) -> Result<GameConfig> {
    if !config_path.exists() {
        let new_file = &File::create(config_path);
        let new_file = match new_file {
            Ok(f) => f,
            Err(e) => return Err(CouldNotSetUpGameConfig),
        };

        let save_result = serde_yaml::to_writer(new_file, &example_game_config());
        if let Err(e) = save_result {
            return Err(CouldNotSetUpGameConfig);
        }
    }

    Ok(example_game_config())
}
