#![doc = include_str!("../README.md")]

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
    pub speed: Option<u8>,
    pub turn_speed: Option<u8>,
    pub step_time: Option<usize>,
    pub turn_time: Option<usize>,
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
            speed: None,
            turn_speed: None,
            step_time: None,
            turn_time: None,
        })
    }

    pub fn new_port(port: &str) -> Result<Self, sError> {
        let mut settings = SerialPortSettings::default();
        settings.timeout = time::Duration::from_secs(60);
        let conn = TTYPort::open(Path::new(port), &settings)?;
        let wait = time::Duration::from_secs(2);
        thread::sleep(wait);
        Ok(Arlo {
            connection: conn,
            speed: None,
            turn_speed: None,
            step_time: None,
            turn_time: None,
        })
    }

    /// Sends a command to the Arduino robot controller
    fn send_command(&mut self, cmd: &str, sleep: Option<u64>) -> String {
        use std::io::BufReader;
        use std::io::BufRead;
        let wait = match sleep {
            Some(n) => n,
            None => 0,
        };
        let conn: &mut TTYPort = &mut self.connection;
        conn.write(cmd.as_bytes()).expect("Write failed");
        thread::sleep(time::Duration::from_millis(wait));
        let mut reader = BufReader::new(conn);
        let mut line = String::new();
        let _len = reader.read_line(&mut line).expect("Read failed");
        //println!("line: {}, len: {}", line.trim(), _len);
        line.trim().to_string()
    }

    /// Start left motor with motor power powerLeft (in \[0;127\]) and
    /// direction dirLeft (0=reverse, 1=forward) and right motor with
    /// motor power powerRight (in ª[0;127\]) and direction dirRight
    /// (0=reverse, 1=forward).
    ///       
    /// NOTE: Does NOT use wheel encoders.
    pub fn go_diff(&mut self, power_left: u8, power_right: u8, dir_left: u8, dir_right: u8) -> String {
        if power_left > 127 || power_right > 127 || dir_left > 1 || dir_right > 1 {
            panic!("Reason: Variables set too high");
        }
        let cmd = format!("d{},{},{},{}\n", power_left, power_right, dir_left, dir_right);
        self.send_command(&cmd, None)
    }

    /// Send a stop command to stop motors. Sets the motor power on both wheels to zero.
    ///
    /// NOTE: Does NOT use wheel encoders.
    pub fn stop(&mut self) -> String {
        self.send_command("s\n", None)
    }

    /// Send a go command for continuous forward driving using the wheel encoders
    pub fn go(&mut self) -> String {
        if self.speed.is_none() {
            panic!("Speed not set!");
        }
        let cmd = "g\n";
        self.send_command(&cmd, None)
    }

    /// Send a backward command for continuous reverse driving using the wheel encoders
    pub fn backward(&mut self) -> String {
        if self.speed.is_none() {
            panic!("Speed not set!");
        }
        let cmd = "v\n";
        self.send_command(&cmd, None)
    }

    /// Send a rotate left command for continuous rotating left using the wheel encoders
    pub fn left(&mut self) -> String {
        if self.speed.is_none() {
            panic!("Speed not set!");
        }
        let cmd = "n\n";
        self.send_command(&cmd, None)
    }

    /// Send a rotate right command for continuous rotating right using the wheel encoders
    pub fn right(&mut self) -> String {
        if self.speed.is_none() {
            panic!("Speed not set!");
        }
        let cmd = "m\n";
        self.send_command(&cmd, None)
    }

    /// Send a step forward command for driving forward using the wheel encoders for a 
    /// predefined amount of time
    pub fn step_forward(&mut self) -> String {
        if self.step_time.is_none() {
            panic!("step_time not set!")
        }
        let cmd = "f\n";
        self.send_command(&cmd, None)
    }

    /// Send a step backward command for driving backward using the wheel encoders for a 
    /// predefined amount of time
    pub fn step_backward(&mut self) -> String {
        if self.step_time.is_none() {
            panic!("step_time not set!")
        }
        let cmd = "b\n";
        self.send_command(&cmd, None)
    }

    /// Send a step rotate left command for rotating left using the wheel encoders for a 
    /// predefined amount of time
    pub fn step_rotate_left(&mut self) -> String {
        if self.turn_time.is_none() {
            panic!("turn_time not set!")
        }
        let cmd = "l\n";
        self.send_command(&cmd, None)
    }

    /// Send a step rotate right command for rotating right using the wheel encoders for 
    /// a predefined amount of time
    pub fn step_rotate_right(&mut self) -> String {
        if self.turn_time.is_none() {
            panic!("turn_time not set!")
        }
        let cmd = "r\n";
        self.send_command(&cmd, None)
    }

    /// Send a read sensor command with sensorid and return sensor value.
    fn read_sensor(&mut self, sensor_id: u8) -> Result<usize, ()> {
        let cmd = format!("{}\n", sensor_id);
        let return_value = self.send_command(&cmd, None).parse::<usize>().map_err(|_| ())?;
        //println!("Ret: {}", return_value);
        if return_value <= 0 {
            Err(())
        } else {
            Ok(return_value)
        }
    }
    /// Read the front sonar ping sensor and return the measured range in milimeters \[mm\]
    pub fn read_front_ping_sensor(&mut self) -> Result<usize, ()> {
        self.read_sensor(0)
    }

    /// Read the back sonar ping sensor and return the measured range in milimeters \[mm\]
    pub fn read_back_ping_sensor(&mut self) -> Result<usize, ()> {
        self.read_sensor(1)
    }
    /// Read the left sonar ping sensor and return the measured range in milimeters \[mm\]
    pub fn read_left_ping_sensor(&mut self) -> Result<usize, ()> {
        self.read_sensor(2)
    }

    /// Read the right sonar ping sensor and return the measured range in milimeters \[mm\]
    pub fn read_right_ping_sensor(&mut self) -> Result<usize, ()> {
        self.read_sensor(3)
    }

    /// Speed must be a value in the range \[0; 255\]. This speed is used in commands based on 
    /// using the wheel encoders.
    pub fn set_speed(&mut self, speed: u8) -> String {
        self.speed = Some(speed);
        let cmd = format!("z{}\n", speed);
        self.send_command(&cmd, None)
    }

    /// Turnspeed must be a value in the range \[0; 255\]. This speed is used in commands based on 
    /// using the wheel encoders.
    pub fn set_turnspeed(&mut self, turn_speed: u8) -> String {
        self.turn_speed = Some(turn_speed);
        let cmd = format!("x{}\n", turn_speed);
        self.send_command(&cmd, None)
    }

    /// steptime is the amount of miliseconds used in the step_forward and step_backwards 
    /// commands.
    pub fn set_step_time(&mut self, step_time: usize) -> String {
        self.step_time = Some(step_time);
        let cmd = format!("t{}\n", step_time);
        self.send_command(&cmd, None)
    }

    /// turntime is the amount of miliseconds used in the step_rotate_left and 
    /// step_rotate_right commands.
    pub fn set_turn_time(&mut self, step_time: usize) -> String {
        self.step_time = Some(step_time);
        let cmd = format!("y{}\n", step_time);
        self.send_command(&cmd, None)
    }

    /// Reads the left wheel encoder counts since last reset_encoder_counts command.
    /// The encoder has 144 counts for one complete wheel revolution.
    pub fn read_left_wheel_encoder(&mut self) -> usize {
        let cmd = "e0\n";
        self.send_command(&cmd, Some(45)).parse::<usize>().unwrap()
    }

    /// Reads the right wheel encoder counts since last clear reset_encoder_counts command.
    /// The encoder has 144 counts for one complete wheel revolution.
    pub fn read_right_wheel_encoder(&mut self) -> usize {
        let cmd = "e1\n";
        self.send_command(&cmd, Some(45)).parse::<usize>().unwrap()
    }

    /// Reset the wheel encoder counts.
    pub fn reset_encoder_counts(&mut self) -> String {
        self.send_command("c\n", None)
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
