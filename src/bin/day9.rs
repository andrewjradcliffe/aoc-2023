use aoc_2023::day9::*;
use std::env;

fn main() {
    let mut args = env::args();
    match args.nth(1) {
        Some(path) => match sum_extrapolated_from_path(path) {
            Ok((fwd, back)) => println!("{}\n{}", fwd, back),
            Err(e) => println!("{:#?}", e),
        },
        None => println!("Please provide path to file as first argument"),
    }
}
