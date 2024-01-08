use aoc_2023::day12::*;
use std::env;

fn main() {
    let mut args = env::args();
    match args.nth(1) {
        Some(path) => match rows_from_path(path) {
            Ok(rows) => {
                let sum = rows
                    .into_iter()
                    .map(RowAnalyzer::from)
                    .map(|mut x| x.count_arrangements())
                    .sum::<usize>();
                println!("{}", sum);
            }
            Err(e) => println!("{:#?}", e),
        },
        None => println!("Please provide path to file as first argument"),
    }
}
