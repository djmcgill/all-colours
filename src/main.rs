use std::io::{stderr, Write};

pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 128;
pub const COLOUR_WIDTH: usize = 32;

fn main() {
    println!("P3 {} {} {}", WIDTH, HEIGHT, COLOUR_WIDTH);
    for r in 0..COLOUR_WIDTH {
        for g in 0..COLOUR_WIDTH {
            for b in 0..COLOUR_WIDTH {
                println!("{} {} {} ", r, g, b); // for P3 (ascii) PPM    
            }
        };
        stderr().write_all(format!("{:.2}% done\n", (r as f32/COLOUR_WIDTH as f32) * 100.0).as_bytes()).unwrap();
    }
}
