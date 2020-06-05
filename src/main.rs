use enigo::*;
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

    let mut enigo = Enigo::new();
    let size = Enigo::main_display_size();

    let edge = 0..(opt.distance as i32) + 1;
    let offset = -(opt.distance as i32);
    loop {
        opt.log_fmt(format_args!("Sleep for {}", opt.interval))?;
        thread::sleep(Duration::from_secs(opt.interval));

        let position = Enigo::mouse_location();
        opt.log_fmt(format_args!(
            "Wake up at mouse cursor position at {} {}",
            position.0, position.1
        ))?;

        if edge.contains(&position.0) || edge.contains(&position.1) {
            enigo.mouse_move_to(size.0 as i32, size.1 as i32);
        } else {
            enigo.mouse_move_relative(offset, offset);
        }

        let position = Enigo::mouse_location();
        opt.log_fmt(format_args!(
            "Tried to move and now at {} {}",
            position.0, position.1
        ))?;
    }
}

impl Opt {
    fn log_fmt(&self, fmt: std::fmt::Arguments) -> Result<()> {
        if self.verbose {
            io::stdout().write_fmt(fmt)?;
            io::stdout().write_all(b"\n")?;
        }
        Ok(())
    }
}
