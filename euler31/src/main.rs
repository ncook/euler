// Coin sums: Project Euler problem 31
//
// In England the currency is made up of pound, £, and pence, p, and there are eight coins in general circulation:
//
//     1p, 2p, 5p, 10p, 20p, 50p, £1 (100p) and £2 (200p).
//
// It is possible to make £2 in the following way:
//
//     1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
//
// How many different ways can £2 be made using any number of coins?

// Using a certain value of coin or less to total a certain sum. How many solutions are there?
// Using recursion and caching solutions to subproblems as phrased above:

// For those inputs and outputs, the inputs are the keys to a set and the outputs the values.

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]
#![feature(inclusive_range_syntax)]

// Import the macro. Don't forget to add `error-chain` in your
// `Cargo.toml`!
#[macro_use]
extern crate error_chain;
extern crate num_integer;

// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! { }
}

use errors::*;


fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

// Most functions will return the `Result` type, imported from the
// `errors` module. It is a typedef of the standard `Result` type
// for which the error type is always our own `Error`.
fn run() -> Result<()> {
    let coins: Vec<u64> = vec![1, 2, 5, 10, 20, 50, 100, 200];

    let solutions = using_at_most(200, coins.as_slice());;

    println!("There are {} solutions.", solutions);

    Ok(())
}

fn using_at_most(target: u64, coins: &[u64]) -> u64 {
    use num_integer::Integer;

    if target == 0 { return 1 };

    let mut solutions = 0;
    if let Some(head) = coins.get(0) {
        let (max_used, left) = target.div_rem(&head);
        // Use from zero to as many of first_coin as possible before recusively seeking solutions
        // for the remainer of the target
        println!("{:?} {}", max_used, left);
        if coins.len() > 1 {
            let c = &coins[1..=coins.len() - 1];
            for used in 0..=max_used {
                let s = using_at_most(left + used * head, c);
                solutions += s;
                println!{"new: {}, tot: {}, coins used: {:?}", s, solutions, c}
            }
        } else {
            solutions += 1;
            println!{"new: 1, tot: {}, coins: {:?}", solutions, coins};
        }
    }
    solutions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_using_at_most() {
        let mut arr = &[1u64];
        assert_eq!(using_at_most(10, arr), 1);
        assert_eq!(using_at_most(10, &[5u64, 1]), 3);
    }
}
