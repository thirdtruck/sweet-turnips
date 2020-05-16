mod tangy;
mod config;
mod render;

use tangy::{Direction, Ticks, World, GRID_HEIGHT, GRID_WIDTH};

use config::{GameConfig, WorldConfig};

use sweet_turnips;
use sweet_turnips::{Context, GameResult};
use sweet_turnips::event;
use sweet_turnips::event::{KeyCode, KeyMods};
use sweet_turnips::sprites::{Sprites};
use sweet_turnips::midi;
use sweet_turnips::midi::{MidiInput};

use std::convert::From;
use std::fs;
use std::path;
use std::thread;
use std::sync::mpsc;
use std::io::{stdin};

const GAME_NAME: &str = "tangy-turnips";

const SPRITE_SCALE: f32 = 4.0;
const SPRITE_SIZE: f32 = 8.0 * SPRITE_SCALE;

type MidiReceiver = mpsc::Receiver<(u8, u8)>;

struct MainState {
    world: World,
    sprites: Sprites,
    ticks: Ticks,
    rx: Option<MidiReceiver>,
    tick_speed: usize,
}

impl MainState {
    fn new(ctx: &mut Context, game_config: GameConfig, rx: Option<MidiReceiver>) -> GameResult<MainState> {
        let sprites = Sprites::new(ctx)?;

        let ticks: Ticks = 0;

        let s = MainState {
            world: game_config.world.into(),
            sprites,
            ticks,
            rx,
            tick_speed: 40,
        };
        Ok(s)
    }

    fn move_player_ship(&mut self, direction: Direction) {
        self.world = self.world.with_player_ship_move_requested(direction);
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let mut tick_speed: usize = self.tick_speed;
        let mut dir: Option<Direction> = None;

        if let Some(receiver) = &self.rx {
            // Mapped to a Korg nanoKONTROL2
            // TODO: Add mapping options to config

            while let Ok(key_value) = receiver.try_recv() {
                let (key, value) = key_value;

                if key == 43 && value == 127 { // rewind
                    dir = Some(Direction::Left);
                }

                if key == 44 && value == 127 { // fast forward
                    dir = Some(Direction::Right);
                }

                if key == 16 {
                    if value > 0 {
                        tick_speed = value as usize;
                    } else {
                        tick_speed = 1;
                    }
                }
            }
        }

        self.tick_speed = tick_speed;
        if let Some(d) = dir {
            self.move_player_ship(d);
        }

        self.ticks += 1;

        if (self.ticks + 1) % self.tick_speed == 0 {
            self.world = self.world.ticked();
        } else {
            self.world = self.world.with_events_processed();
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Escape => event::quit(ctx),
            KeyCode::W => self.move_player_ship(Direction::Up),
            KeyCode::A => self.move_player_ship(Direction::Left),
            KeyCode::S => self.move_player_ship(Direction::Down),
            KeyCode::D => self.move_player_ship(Direction::Right),
            _ => (),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let sprite_grid = render::sprite_grid_from_world(&self.world);

        self.sprites.render_sprite_grid(sprite_grid);

        self.sprites.draw_all_sprites(ctx)?;

        Ok(())
    }
}

impl From<WorldConfig> for World {
    fn from(config: WorldConfig) -> Self {
        let player_ship_coords =
            (config.starting_player_ship.x, config.starting_player_ship.y);

        let world = Self::new().with_player_ship_added_at(player_ship_coords);

        world
    }
}

pub fn main() -> GameResult {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut midi_in = MidiInput::new("midir reading input").expect("Unable to read MIDI inputs");
        midi_in.ignore(midi::Ignore::None);

        let in_ports = midi_in.ports();
        let in_port = match in_ports.len() {
            0 => None,
            1 => Some(&in_ports[0]),
            _ => Some(&in_ports[1]),
        };

        if in_port.is_none() {
            return;
        }

        let in_port = in_port.expect("Unable to select a MIDI input");

        let in_port_name = midi_in.port_name(in_port).expect("Unable to fetch fetch MIDI port name");

        println!("\nFound {} MIDI connections", in_ports.len());
        println!("\nOpening connection to {}", in_port_name);

        let _connection = midi_in.connect(in_port, "midir-read-input", move |_, message, _| {
            let (key, value) = (message[1], message[2]);
            tx.send((key, value)).unwrap();
        }, ()).expect("Unable to open connection to MIDI input");

        // TODO: Find a better way to keep this thread alive
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Unable to read input from STDIN");
    });

    let resource_dir = path::PathBuf::from("./resources");

    let config_dir = resource_dir.join(GAME_NAME);
    let config_path = config_dir.join("config.yaml");

    fs::create_dir_all(config_dir)?;
    let game_config = config::setup_game_config(config_path);

    let cb = sweet_turnips::ContextBuilder::new(GAME_NAME, "JC Holder")
        .add_resource_path(resource_dir)
        .window_mode(sweet_turnips::conf::WindowMode::default().dimensions(
            GRID_WIDTH as f32 * SPRITE_SIZE,
            GRID_HEIGHT as f32 * SPRITE_SIZE,
        ));

    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new(ctx, game_config, Some(rx))?;

    event::run(ctx, event_loop, state)
}
