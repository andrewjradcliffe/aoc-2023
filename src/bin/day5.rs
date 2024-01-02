use aoc_2023::day5::*;
use std::env;

fn main() {
    let mut args = env::args();
    match args.nth(1) {
        Some(path) => match Almanac::from_path(path) {
            Ok(x) => println!("{}", x.minimum_location()),
            Err(e) => println!("{:#?}", e),
        },
        None => println!("Please provide path to file as first argument"),
    }
}
