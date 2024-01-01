use std::env;
use aoc_2023::day3::*;

fn main() {
    let mut args = env::args();
    match args.nth(1) {
        Some(path) => {
            match sum_schematic(path) {
                Ok(sum) => println!("{}", sum),
                Err(e) => println!("{:#?}", e),
            }
        }
        None => println!("Please provide a path as the first argument"),
    }
}
