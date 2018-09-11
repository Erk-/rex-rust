extern crate rex_ucph;

use rex_ucph::Arlo;

use std::{thread, time};

fn main() {
    let mut robot = Arlo::new().unwrap();
    robot.go_diff(30,30,1,1);
    let wait = time::Duration::from_secs(2);
    thread::sleep(wait);
    robot.stop();
}
