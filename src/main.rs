use std::io::Cursor;

use zbus::blocking::connection::{self, Builder};

mod fun;
mod music;

fn main() {
    music::play_metal_crusher_music();

    fun::notify("Fun things will happen soon", ":D");

    std::thread::sleep(std::time::Duration::from_secs(4));
}