use aoc_2023::day14::*;
use std::env;

fn main() {
    let mut args = env::args();
    match args.nth(1) {
        Some(path) => match Platform::from_path(path) {
            Ok(mut x) => {
                let mut part1 = x.clone();
                part1.tilt_north();
                let sum = part1.total_load();
                println!("{}", sum);
                let sum = x.cycle_and_compute_load(1_000_000_000);
                println!("{}", sum);
            }
            Err(e) => println!("{:#?}", e),
        },
        None => println!("Please provide path to file as first argument"),
    }
}
