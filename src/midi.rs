use midir;
pub use midir::{Ignore, MidiInput};

use std::io::stdin;
use std::sync::mpsc;
use std::thread;

pub type MidiReceiver = mpsc::Receiver<(u8, u8)>;
type MidiSender = mpsc::Sender<(u8, u8)>;

pub fn connect_to_midi(tx: MidiSender) {
    thread::spawn(move || {
        let mut midi_in =
            MidiInput::new("midir reading input").expect("Unable to read MIDI inputs");
        midi_in.ignore(midir::Ignore::None);

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

        let in_port_name = midi_in
            .port_name(in_port)
            .expect("Unable to fetch fetch MIDI port name");

        println!("\nFound {} MIDI connections", in_ports.len());
        println!("\nOpening connection to {}", in_port_name);

        let _connection = midi_in
            .connect(
                in_port,
                "midir-read-input",
                move |_, message, _| {
                    let (key, value) = (message[1], message[2]);
                    tx.send((key, value)).unwrap();
                },
                (),
            )
            .expect("Unable to open connection to MIDI input");

        // TODO: Find a better way to keep this thread alive
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Unable to read input from STDIN");
    });
}
