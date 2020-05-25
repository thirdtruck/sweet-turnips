use serde::{Deserialize, Serialize};

use std::path::PathBuf;

use sweet_turnips::default_game_config_setup;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PlayerShipConfig {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EnemyShipConfig {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WorldConfig {
    pub starting_player_ship: PlayerShipConfig,
    pub starting_enemy_ships: Vec<EnemyShipConfig>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GameConfig {
    pub world: WorldConfig,
}

fn example_game_config() -> GameConfig {
    let world_config = WorldConfig {
        starting_player_ship: PlayerShipConfig { x: 2, y: 5 },
        starting_enemy_ships: vec![EnemyShipConfig { x: 3, y: 1 }],
    };

    GameConfig {
        world: world_config,
    }
}

pub fn setup_game_config(config_path: PathBuf) -> GameConfig {
    default_game_config_setup(config_path, example_game_config())
}
