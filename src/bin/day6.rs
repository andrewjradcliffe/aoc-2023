use aoc_2023::day6::*;
use std::env;

fn main() {
    let mut args = env::args();
    match args.nth(1) {
        Some(path) => {
            match races_from_path_part1(&path) {
                Ok(x) => {
                    let n: u64 = x.iter().map(|r| r.ways_to_win_bracketing()).product();
                    println!("{}", n);
                }
                Err(e) => println!("{:#?}", e),
            }
            match Race::from_path(path) {
                Ok(x) => {
                    /*
                    hyperfine "./target/release/day6 ./input/day6.txt"
                    Benchmark 1: ./target/release/day6 ./input/day6.txt
                    Time (mean ± σ):      19.1 ms ±   1.1 ms    [User: 17.2 ms, System: 1.3 ms]
                    Range (min … max):    18.6 ms …  28.2 ms    144 runs
                    */
                    // let n = x.ways_to_win();

                    /*
                    hyperfine "./target/release/day6 ./input/day6.txt"
                    Benchmark 1: ./target/release/day6 ./input/day6.txt
                    Time (mean ± σ):      12.6 ms ±   1.0 ms    [User: 10.5 ms, System: 1.6 ms]
                    Range (min … max):    11.4 ms …  19.5 ms    140 runs
                    */
                    // let n = x.ways_to_win_bracketing();

                    /*
                    hyperfine "./target/release/day6 ./input/day6.txt"
                    Benchmark 1: ./target/release/day6 ./input/day6.txt
                    Time (mean ± σ):       1.9 ms ±   2.6 ms    [User: 0.7 ms, System: 1.1 ms]
                    Range (min … max):     1.3 ms …  27.8 ms    101 runs
                    */
                    let n = x.ways_to_win_newton();
                    println!("{}", n);
                }
                Err(e) => println!("{:#?}", e),
            }
        }
        None => println!("Please provide path to file as first argument"),
    }
}
