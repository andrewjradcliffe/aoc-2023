use aoc_2023::day13::*;
use std::env;

fn main() {
    let mut args = env::args();
    match args.nth(1) {
        Some(path) => match grids_from_path(path) {
            Ok(mut grids) => {
                let sum = sum_reflections_part1(&mut grids);
                println!("{}", sum);
                let sum = sum_reflections_part2(&mut grids);
                println!("{}", sum);
            }
            Err(e) => println!("{:#?}", e),
        },
        None => println!("Please provide path to file as first argument"),
    }
}
