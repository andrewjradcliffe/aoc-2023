use aoc_2023::day1::{parse_file, part1, part2};
use std::env;

fn main() {
    let mut args = env::args();
    match args.nth(1) {
        Some(ref path) => {
            match parse_file(part1::parse_line, path) {
                Ok(x) => println!("{}", x),
                Err(e) => println!("{:?}", e),
            }
            match parse_file(part2::parse_line, path) {
                Ok(x) => println!("{}", x),
                Err(e) => println!("{:?}", e),
            }
        }
        _ => println!("Please provide input file as first argument"),
    }
}
