use std::thread;
use std::time::Duration;

use std::io::{self, Result, Write};
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

    #[structopt(short, long)]
    verbose: bool,
}

pub fn main() -> Result<()> {
    let opt = Opt::from_args();
    macro_rules! log {
        ($($expression:expr),+) => {
            if opt.verbose {
                write!(io::stdout(), $($expression),+)?;
                io::stdout().write_all(b"\n")?;
            }
        };
    }

    let mouse = mouse_rs::Mouse::new();
    let mut offset = opt.distance as i32;
    loop {
        log!("Sleep for {} seconds...", opt.interval);
        thread::sleep(Duration::from_secs(opt.interval));

        let position = mouse.get_position().unwrap();
        log!(
            "Wake up at mouse cursor position at {} {}",
            position.x,
            position.y
        );

        mouse
            .move_to(position.x as i32 + offset, position.y as i32 + offset)
            .expect("Unable to move mouse");

        let new_position = mouse.get_position().unwrap();
        log!(
            "Tried to move and now at {} {}",
            new_position.x,
            new_position.y
        );
        if new_position.x == position.x && new_position.y == position.y {
            offset = -offset;
        }
    }
}
