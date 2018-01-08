// 1000-digit Fibonacci number
//
// The Fibonacci sequence is defined by the recurrence relation:
//
//     Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
//
// Hence the first 12 terms will be:
//
//     F1 = 1
//     F2 = 1
//     F3 = 2
//     F4 = 3
//     F5 = 5
//     F6 = 8
//     F7 = 13
//     F8 = 21
//     F9 = 34
//     F10 = 55
//     F11 = 89
//     F12 = 144
//
// The 12th term, F12, is the first term to contain three digits.
//
// What is the index of the first term in the Fibonacci sequence to contain 1000 digits?
extern crate num;
extern crate num_traits;

use num::bigint::BigUint;
use std::ops::Add;
use num_traits::{Zero, One};

struct Fib {
    current: BigUint,
    previous: BigUint
}

impl Fib {
    fn new() -> Fib {
        Fib { current: Zero::zero(), previous: One::one() }
    }
}

impl Iterator for Fib {
    type Item = BigUint;
    fn next(&mut self) -> Option<BigUint> {
        let i = self.previous.clone().add(&self.current);
        self.previous = self.current.clone();
        self.current = i.clone();
        Some(i)
    }
}

fn main() {
    let fib = Fib::new();
    if let Some((idx, num)) = fib
        .enumerate()
        .find(|&(_index, ref x)| x.to_str_radix(10).len() >= 1000) {
            println!("Index of the first fib with 1000 or more digits {:?}", idx + 1);
            println!("It is {:?}", num.to_str_radix(10));
            println!("It is {:?} digits long", num.to_str_radix(10).len());
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_sequence() {
        let mut fib = Fib::new().enumerate();
        assert_eq!(fib.next(), Some((0, BigUint::from(1u64))));
        assert_eq!(fib.next(), Some((1, BigUint::from(1u64))));
        assert_eq!(fib.next(), Some((2, BigUint::from(2u64))));
        assert_eq!(fib.next(), Some((3, BigUint::from(3u64))));
        assert_eq!(fib.next(), Some((4, BigUint::from(5u64))));
        assert_eq!(fib.next(), Some((5, BigUint::from(8u64))));
        assert_eq!(fib.next(), Some((6, BigUint::from(13u64))));
        assert_eq!(fib.next(), Some((7, BigUint::from(21u64))));
    }
}
