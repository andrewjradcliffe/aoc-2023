use aoc_2023::day10::*;
use std::env;

fn main() {
    let mut args = env::args();
    match args.nth(1) {
        Some(path) => match Grid::from_path(path) {
            Ok(grid) => {
                match Visitor::try_from(&grid) {
                    Ok(mut vis) => {
                        vis.traverse();
                        println!("{}", vis.farthest());
                    }
                    Err(e) => println!("{:#?}", e),
                }
                match StatefulVisitor::try_from(&grid) {
                    Ok(mut vis) => {
                        vis.classify_states();
                        println!("{}", vis.enclosed());
                        println!("{}", vis);
                    }
                    Err(e) => println!("{:#?}", e),
                }
            }
            Err(e) => println!("{:#?}", e),
        },
        None => println!("Please provide path to file as first argument"),
    }
}
