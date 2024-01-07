use aoc_2023::day11::*;
use std::env;

fn main() {
    let mut args = env::args();
    match args.nth(1) {
        Some(path) => match Grid::from_path(path) {
            Ok(mut grid) => {
                grid.expand_empty_rows();
                grid.expand_empty_columns();
                let galaxies = Galaxies::from(&grid);
                println!("{}", galaxies.sum_manhattan_distances());
            }
            Err(e) => println!("{:#?}", e),
        },
        None => println!("Please provide path to file as first argument"),
    }
}
