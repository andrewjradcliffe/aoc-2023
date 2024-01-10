use aoc_2023::day15::*;
use std::env;

fn main() {
    let mut args = env::args();
    match args.nth(1) {
        Some(path) => match init_seq_from_path(&path) {
            Ok(s) => {
                let sum = init_seq_sum(&s);
                println!("{}", sum);
                match HashMap::try_from(s.as_str()) {
                    Ok(map) => {
                        let power = map.focusing_power();
                        println!("{}", power);
                    }
                    Err(e) => println!("{:#?}", e),
                }
            }
            Err(e) => println!("{:#?}", e),
        },
        None => println!("Please provide path to file as first argument"),
    }
}
