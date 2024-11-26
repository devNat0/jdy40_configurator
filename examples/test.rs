use std::time::Duration;

use jdy_40::{jdy_get, jdy_set, read_port, ATSet, AT};

fn main() {
    let mut port = serialport::new("/dev/ttyUSB0", 9600)
        .timeout(Duration::from_secs(1))
        .open()
        .expect("Failed to open port");

    jdy_get(&mut port, AT::CLSS).unwrap();
    println!("old POWE: {}", read_port(&mut port));

    jdy_set(&mut port, ATSet::CLSS(jdy_40::CLSSParam::C2)).unwrap();
    println!("Response: {}", read_port(&mut port));

    jdy_get(&mut port, AT::CLSS).unwrap();
    println!("new POWE: {}", read_port(&mut port));
}
