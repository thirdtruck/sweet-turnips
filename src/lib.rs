pub mod midi;
pub mod sprites;

pub use ggez::conf;
pub use ggez::event;
pub use ggez::event::EventsLoop;
pub use ggez::{Context, ContextBuilder, GameResult};

use ggez::conf::WindowMode;

use serde::{Deserialize, Serialize};
use serde_yaml;

use std::fs;
use std::fs::File;
use std::path;
use std::path::PathBuf;

use sprites::{SPRITE_SCALE, SPRITE_SIZE};

// TODO:
// * Add an UnfinalizedAppConfig struct and have AppConfig::new return that
// * Move config methods to UnfinalizedAppConfig
// * Add a finalize() method that returns a vetted AppConfig
pub struct AppConfig {
    game_name: String,
    author_name: String,
    grid_dimensions: (u8, u8),
}

impl AppConfig {
    pub fn new(grid_dimensions: (u8, u8)) -> Self {
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

    pub fn grid_dimensions(self, grid_dimensions: (u8, u8)) -> Self {
        Self {
            grid_dimensions,
            ..self
        }
    }
}

pub fn build_context_and_event_loop(app_config: &AppConfig) -> GameResult<(Context, EventsLoop)> {
    let (width, height) = app_config.grid_dimensions;

    let cb = ContextBuilder::new(&app_config.game_name, &app_config.author_name)
        .add_resource_path(resource_dir())
        .window_mode(default_window_mode(width, height));

    Ok(cb.build()?)
}


pub fn prep_config_path(app_config: &AppConfig) -> GameResult<PathBuf> {
    let config_dir = resource_dir().join(&app_config.game_name);
    let config_path = config_dir.join("config.yaml");

    fs::create_dir_all(config_dir)?;

    Ok(config_path)
}

fn resource_dir() -> PathBuf {
    path::PathBuf::from("./resources")
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
