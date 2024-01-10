use aoc_2023::day14::*;
use std::env;

fn main() {
    let mut args = env::args();
    match args.nth(1) {
        Some(path) => match Platform::from_path(path) {
            Ok(mut x) => {
                x.tilt_north();
                let sum = x.total_load();
                println!("{}", sum);
            }
            Err(e) => println!("{:#?}", e),
        },
        None => println!("Please provide path to file as first argument"),
    }
}
