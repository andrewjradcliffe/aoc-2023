use aoc_2023::day2::*;
use std::env;

fn main() {
    let mut args = env::args();
    match args.nth(1) {
        Some(path) => match games_from_file(path) {
            Ok(games) => {
                println!("{}", sum_possible(&games, 12, 13, 14));
                println!("{}", sum_powerset(&games));
                println!("{}", sum_powerset_incl_null_set(&games));
            }
            Err(e) => println!("{:?}", e),
        },
        None => println!("Please provide a path as the first argument."),
    }
}
