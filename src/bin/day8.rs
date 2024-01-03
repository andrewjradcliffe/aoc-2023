use aoc_2023::day8::*;
use std::env;

fn main() {
    let mut args = env::args();
    match args.nth(1) {
        Some(path) => match seq_network_from_path(path) {
            Ok((seq, network)) => {
                let entry = Node::from(['A', 'A', 'A']);
                let exit = Node::from(['Z', 'Z', 'Z']);
                match network.traverse(seq, entry, exit) {
                    Ok(n) => println!("terminate at exit after: {}", n),
                    Err(n) => println!("does not terminate at exit after: {}", n),
                }
            }
            Err(e) => println!("{:#?}", e),
        },
        None => println!("Please provide path to file as first argument"),
    }
}
