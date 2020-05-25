pub mod midi;
pub mod sprites;

pub use ggez::conf;
pub use ggez::event;
pub use ggez::{Context, ContextBuilder, GameResult};

use ggez::conf::WindowMode;

use sprites::{SPRITE_SCALE, SPRITE_SIZE};

pub fn default_window_mode(grid_width: u8, grid_height: u8) -> WindowMode {
    conf::WindowMode::default().dimensions(
        grid_width as f32 * SPRITE_SIZE * SPRITE_SCALE,
        grid_height as f32 * SPRITE_SIZE * SPRITE_SCALE,
    )
}
