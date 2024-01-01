use aoc_2023::day4::*;
use std::env;

fn main() {
    let mut args = env::args();
    match args.nth(1) {
        Some(path) => match cards_from_file(path) {
            Ok(cards) => {
                let points = cards.iter().map(|x| x.points()).sum::<u64>();
                println!("{}", points);
            }
            Err(e) => println!("{:#?}", e),
        },
        None => println!("Please provide a path as the first argument"),
    }
}
