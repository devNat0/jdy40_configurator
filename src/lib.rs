use core::panic;
use std::io::Write;
use serialport::SerialPort;

pub enum AT {
    BAUD,
    RFID,
    DVID,
    RFC,
    POWE,
    CLSS,
}

pub fn jdy_get(port: &mut Box<dyn SerialPort>, at: AT) -> Result<usize, std::io::Error> {
    let command: &[u8] = match at {
        AT::BAUD => b"AT+BAUD\r\n",
        AT::RFID => b"AT+RFID\r\n",
        AT::DVID => b"AT+DVID\r\n",
        AT::RFC => b"AT+RFC\r\n",
        AT::POWE => b"AT+POWE\r\n",
        AT::CLSS => b"AT+CLSS\r\n",
    };
    port.write(command)
}

pub enum CLSSParam {
    A0, //
    C0,
    C1,
    C2, // Normally Low
    C3,
    C4,
    C5,
}
pub enum ATSet {
    BAUD(u8),
    RFID(u16),
    DVID(u16),
    RFC(u8),
    POWE(u8),
    CLSS(CLSSParam),
}
pub fn jdy_set(port: &mut Box<dyn SerialPort>, at: ATSet) -> Result<usize, std::io::Error> {
    let command: &[u8] = match at {
        ATSet::BAUD(param) => {
            if param > 7 {
                panic!("BAUD param must be <= 7");
            }
            &format!("AT+BAUD{}\r\n", param).into_bytes()
        },
        ATSet::RFID(param) => &format!("AT+RFID{:04X}\r\n", param).into_bytes(),
        ATSet::DVID(param) => &format!("AT+DVID{:04X}\r\n", param).into_bytes(),
        ATSet::RFC(param) => {
            if param < 1 || param > 128 {
                panic!("RFC param must be 1 - 128");
            }
            &format!("AT+RFC{:03}\r\n", param).into_bytes()
        },
        ATSet::POWE(param) => {
            if param > 9 {
                panic!("POWE param must be <= 9");
            }
            &format!("AT+POWE{}\r\n", param).into_bytes()
        },
        ATSet::CLSS(x) => match x {
            CLSSParam::A0 => b"AT+CLSSA0\r\n",
            CLSSParam::C0 => b"AT+CLSSC0\r\n",
            CLSSParam::C1 => b"AT+CLSSC1\r\n",
            CLSSParam::C2 => b"AT+CLSSC2\r\n",
            CLSSParam::C3 => b"AT+CLSSC3\r\n",
            CLSSParam::C4 => b"AT+CLSSC4\r\n",
            CLSSParam::C5 => b"AT+CLSSC5\r\n",
        }
    };
    port.write(command)
}
