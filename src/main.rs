use enigo::*;
use std::thread;
use std::time::Duration;

pub fn main() {
    let mut enigo = Enigo::new();
    let size = Enigo::main_display_size();

    loop {
        thread::sleep(Duration::from_secs(60));

        match Enigo::mouse_location() {
            (0, 0) => enigo.mouse_move_to(size.0 as i32, size.1 as i32),
            _ => enigo.mouse_move_relative(-5, -5),
        }
    }
}
