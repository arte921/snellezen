use std::time::Duration;
use std::thread::sleep;
use raylib::prelude::*;
use std::env;
use std::fs;
use regex::Regex;

const WPM: u64 = 300;

const HOOGTE: i32 = 800;
const BREEDTE: i32 = 1200;

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let (mut rl, thread) = raylib::init().size(BREEDTE, HOOGTE).title("Snellezen").build();

    rl.set_target_fps(60);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let regex = Regex::new(r"\n| ").unwrap();
    let woorden = regex.split(&contents);

    for woord in woorden {
        if rl.window_should_close() {
            break;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        d.draw_text(woord, 200, 350, 70, Color::WHITE);
        sleep(Duration::from_millis(1000 / WPM * 60));
    }

    Ok(())
}
