use enigo::*;
use std::thread;
use std::time::Duration;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "mouse mover", about = "Mouse mover usage.")]
struct Opt {
    /// set the distance delta when we want to move
    #[structopt(short, long, default_value = "5")]
    distance: usize,

    /// set the time interval in seconds, how often we run
    #[structopt(short, long, default_value = "60")]
    interval: u64,
}

pub fn main() {
    let opt = Opt::from_args();

    let mut enigo = Enigo::new();
    let size = Enigo::main_display_size();

    let edge = 0..(opt.distance as i32) + 1;
    let offset = -(opt.distance as i32);
    loop {
        thread::sleep(Duration::from_secs(opt.interval));

        let position = Enigo::mouse_location();
        if edge.contains(&position.0) || edge.contains(&position.1) {
            enigo.mouse_move_to(size.0 as i32, size.1 as i32);
        } else {
            enigo.mouse_move_relative(offset, offset);
        }
    }
}
