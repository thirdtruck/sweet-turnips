pub mod midi;
pub mod sprites;

pub use ggez::conf;
pub use ggez::event;
pub use ggez::{Context, ContextBuilder, GameResult};

use ggez::conf::WindowMode;

use serde::{Deserialize, Serialize};
use serde_yaml;

use std::fs;
use std::fs::File;
use std::path::PathBuf;

use sprites::{SPRITE_SCALE, SPRITE_SIZE};

pub struct AppConfig {
    game_name: String,
    author_name: String,
    grid_dimensions: (usize, usize),
}

impl AppConfig {
    pub fn new(grid_dimensions: (usize, usize)) -> Self {
        Self {
            game_name: "A Sweet Turnips Game".to_string(),
            author_name: "Your Name Goes Here".to_string(),
            grid_dimensions,
        }
    }

    pub fn game_name<S>(self, game_name: S) -> Self
        where S: AsRef<str>
    {
        Self {
            game_name: game_name.as_ref().to_string(),
            ..self
        }
    }

    pub fn author_name<S>(self, author_name: S) -> Self
        where S: AsRef<str>
    {
        Self {
            author_name: author_name.as_ref().to_string(),
            ..self
        }
    }

    pub fn grid_dimensions(self, grid_dimensions: (usize, usize)) -> Self {
        Self {
            grid_dimensions,
            ..self
        }
    }
}

pub fn default_window_mode(grid_width: u8, grid_height: u8) -> WindowMode {
    conf::WindowMode::default().dimensions(
        grid_width as f32 * SPRITE_SIZE * SPRITE_SCALE,
        grid_height as f32 * SPRITE_SIZE * SPRITE_SCALE,
    )
}

pub fn default_game_config_setup<'a, S, D>(config_path: PathBuf, example_game_config: S) -> D
where
    S: Serialize,
    for<'de> D: Deserialize<'de> + 'a,
{
    if !config_path.exists() {
        let new_file =
            &File::create(config_path.clone()).expect("Could not create new config file");

        serde_yaml::to_writer(new_file, &example_game_config)
            .expect("Could not write to new config file");
    }

    let config_string = fs::read_to_string(config_path).expect("Could not read config file");

    let config: D = serde_yaml::from_str(&config_string).expect("Could not parse config file");

    config
}
