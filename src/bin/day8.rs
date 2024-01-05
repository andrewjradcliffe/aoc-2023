use aoc_2023::day8::*;
use std::env;

fn main() {
    let mut args = env::args();
    match args.nth(1) {
        Some(path) => match seq_network_from_path(path) {
            Ok((seq, network)) => {
                let entry = Node::from(['A', 'A', 'A']);
                let exit = Node::from(['Z', 'Z', 'Z']);
                match network.traverse(seq.clone(), entry, exit) {
                    Ok(n) => println!("terminate at exit after: {}", n),
                    Err(n) => println!("does not terminate at exit after: {}", n),
                }
                match network.simultaneous_traverse(seq) {
                    Ok(n) => println!("terminate at exit after: {}", n),
                    Err(n) => println!("does not terminate at exit after: {}", n),
                }
            }
            Err(e) => println!("{:#?}", e),
        },
        None => println!("Please provide path to file as first argument"),
    }
}
/*
time ./target/release/day8 ./input/day8.txt
terminate at exit after: 11567
terminate at exit after: 9858474970153

real	983m23.435s
user	981m19.564s
sys	0m1.988s


Really...?

I guess so:

(define n 9858474970153)
(define time (+ 983 (/ 23.435 60)))
(define secs (* time 60))
(/ (* secs 1e9) n)
;; time per iteration = 5.985046894031348 ns

which, for an input with 6 addresses that end in 'A', is 1 ns per dereference.
 */
