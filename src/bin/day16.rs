use aoc_2023::day16::*;
use std::env;

fn main() {
    let mut args = env::args();
    match args.nth(1) {
        Some(path) => match Contraption::from_path(path) {
            Ok(x) => {
                let sum = x.count_energized();
                println!("{}", sum);
                let max = x.maximum_energized();
                println!("{}", max);
            }
            Err(e) => println!("{:#?}", e),
        },
        None => println!("Please provide path to file as first argument"),
    }
}
