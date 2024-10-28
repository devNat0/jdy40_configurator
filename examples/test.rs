use std::{time::Duration,
    ascii::escape_default,
    str
};

use serialport::SerialPort;
use jdy_40::{jdy_get, jdy_set, ATSet, AT};

fn show(bs: &[u8]) -> String {
    let mut visible = String::new();
    for &b in bs {
        let part: Vec<u8> = escape_default(b).collect();
        visible.push_str(str::from_utf8(&part).unwrap());
    }
    visible
}

fn read_port(port: &mut Box<dyn SerialPort> ) -> String {
    let mut serial_buf: Vec<u8> = vec![0; 32];
    let size = port.read(serial_buf.as_mut_slice()).expect("No data found");
    show(&serial_buf[0..size])
}

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