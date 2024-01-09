use aoc_2023::day12::*;
use std::env;

fn main() {
    let mut args = env::args();
    match args.nth(1) {
        Some(path) => match rows_from_path(path) {
            Ok(rows) => {
                let mut analyzers: Vec<_> = rows.into_iter().map(RowAnalyzer::from).collect();
                let sum = analyzers
                    .iter_mut()
                    .map(|x| x.count_arrangements())
                    .sum::<usize>();
                println!("{}", sum);
                let sum = analyzers
                    .iter_mut()
                    .map(|x| x.count_arrangements_with_unfold())
                    .sum::<usize>();
                println!("{}", sum);
            }
            Err(e) => println!("{:#?}", e),
        },
        None => println!("Please provide path to file as first argument"),
    }
}
