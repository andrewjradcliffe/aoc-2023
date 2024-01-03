use aoc_2023::day7::*;
use std::env;

fn main() {
    let mut args = env::args();
    match args.nth(1) {
        Some(path) => match hand_bids_from_path(path) {
            Ok(mut x) => {
                let n = total_winnings(&mut x);
                println!("{}", n);
            }
            Err(e) => println!("{:#?}", e),
        },
        None => println!("Please provide path to file as first argument"),
    }
}
