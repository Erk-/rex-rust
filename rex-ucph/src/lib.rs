extern crate serialport;

use serialport::SerialPortSettings;
use serialport::posix::*;
use serialport::Error as sError;

use std::{thread, time};
use std::io::{Write, Read};
use std::str;
use std::path::Path;

pub struct Arlo {
    #[allow(dead_code)]
    connection: TTYPort,
}

impl Arlo {
    pub fn new() -> Result<Self, sError> {
        let mut settings = SerialPortSettings::default();
        settings.timeout = time::Duration::from_secs(60);
        let conn = TTYPort::open(Path::new("/dev/ttyACM0"), &settings)?;
        let wait = time::Duration::from_secs(2);
        thread::sleep(wait);
        Ok(Arlo {
            connection: conn,
        })
    }

    fn send_command(&mut self, cmd: &str, sleep: Option<u64>) -> String {
        let wait = match sleep {
            Some(n) => n,
            None => 0,
        };
        let conn: &mut TTYPort = &mut self.connection;
        conn.write(cmd.as_bytes()).unwrap();
        thread::sleep(time::Duration::from_millis(wait));
        let mut buf = Vec::<u8>::new();
        let retval = match conn.read(buf.as_mut_slice()) {
            Ok(_t) => String::from_utf8(buf).unwrap(),
            Err(e) => panic!("Reason: {}", e),
        };
        retval
    }

    pub fn go_diff(&mut self, power_left: u8, power_right: u8, dir_left: u8, dir_right: u8) -> String {
        let cmd = format!("d{},{},{},{}\n", power_left, power_right, dir_left, dir_right);
        self.send_command(&cmd, None)
    }

    pub fn stop(&mut self) -> String {
        self.send_command("s\n", None)
    }
}

impl Drop for Arlo {
    fn drop(&mut self) {
        println!("Shutting down the robot ...");
        let wait = time::Duration::from_millis(5);
        let long_wait = time::Duration::from_millis(10);
        thread::sleep(wait);
        self.stop();
        thread::sleep(long_wait);
        self.send_command("k\n", None);
    }
}
