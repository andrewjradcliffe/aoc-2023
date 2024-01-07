use aoc_2023::day11::*;
use std::env;
use std::num::NonZeroUsize;

fn main() {
    let mut args = env::args();
    match args.nth(1) {
        Some(path) => match Grid::from_path(path) {
            Ok(grid) => {
                let galaxies_part1 = expanded_universe(&grid, NonZeroUsize::new(2).unwrap());
                println!("{}", galaxies_part1.sum_manhattan_distances());
                let galaxies_part2 =
                    expanded_universe(&grid, NonZeroUsize::new(1_000_000).unwrap());
                println!("{}", galaxies_part2.sum_manhattan_distances());
            }
            Err(e) => println!("{:#?}", e),
        },
        None => println!("Please provide path to file as first argument"),
    }
}
