use serde::{Deserialize, Serialize};
use serde_yaml;

use std::fs;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PlayerShipConfig {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WorldConfig {
    pub starting_player_ship: PlayerShipConfig,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GameConfig {
    pub world: WorldConfig,
}

fn example_game_config() -> GameConfig {
    let world_config = WorldConfig {
        starting_player_ship: PlayerShipConfig { x: 2, y: 5 },
    };

    GameConfig {
        world: world_config,
    }
}

pub fn setup_game_config(config_path: PathBuf) -> GameConfig {
    if !config_path.exists() {
        let new_file =
            &File::create(config_path.clone()).expect("Could not create new config file");

        serde_yaml::to_writer(new_file, &example_game_config())
            .expect("Could not write to new config file");
    }

    let config_string = fs::read_to_string(config_path).expect("Could not read config file");

    let config: GameConfig =
        serde_yaml::from_str(&config_string).expect("Could not parse config file");

    config
}
