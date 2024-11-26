use std::time::Duration;

use jdy_40::{jdy_get, read_port, AT};

fn main() {
    let mut port = serialport::new("/dev/ttyUSB0", 9600)
        .timeout(Duration::from_secs(1))
        .open()
        .expect("Failed to open port");


    let commands = [AT::BAUD, AT::RFID, AT::DVID, AT::RFC, AT::POWE, AT::CLSS];
    for command in commands {
        jdy_get(&mut port, command).unwrap();
        println!("{}", read_port(&mut port));
    }
}
