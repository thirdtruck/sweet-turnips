use serde::{Serialize, Deserialize};
use serde_yaml;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct VillagerConfig {
    x: u8,
    y: u8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct FarmConfig {
    x: u8,
    y: u8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct WorldConfig {
    starting_villagers: Vec<VillagerConfig>,
    starting_farms: Vec<FarmConfig>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct GameConfig {
    world: WorldConfig,
}

pub fn initialize_config_file() -> Result<(), serde_yaml::Error> {
    let world_config = WorldConfig {
        starting_villagers: vec![
            VillagerConfig { x: 2, y: 2 },
        ],
        starting_farms: vec![
            FarmConfig { x: 3, y: 3 },
        ],
    };

    let game_config = GameConfig { world: world_config };

    let output = serde_yaml::to_string(&game_config)?;

    println!("{}", output);

    Ok(())
}
