#![feature(inclusive_range_syntax)]
#![feature(iterator_step_by)]

extern crate itertools;
use itertools::iterate;

fn main() {
    // Sum of squares - square of sum
    let mut nprimes = 0;

    if let Some(target) = iterate(2, |x| *x + 1 )
        .filter(|&y| prime(y) )
        .find(|_| {
            nprimes += 1; nprimes == 10_001
        }) {
        println!("10,001st prime number: {:?}", target);
    } else {
        println!("Weird - there are fewer than 10,001 prime numbers!")
    }
}

fn prime(number: u64) -> bool {
    let sqrt = f64::floor(f64::sqrt(number as f64)) as u64;
    let range = 3..=sqrt;
    if number != 2 && number % 2 == 0 {
        return false
    }

    match number {
        2 | 3 | 5 | 7 => true,
        4 | 6 | 8 => false,
        _ => range
            .step_by(2)
            // .inspect(|x| println!("target: {:?}, x: {:?}, target/x {:?}", target, x, target / x))
            .all(|x| number % x != 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime() {

        assert_eq!(prime(2), true);
        assert_eq!(prime(3), true);
        assert_eq!(prime(4), false);
        assert_eq!(prime(5), true);
        assert_eq!(prime(6), false);
        assert_eq!(prime(7), true);
        assert_eq!(prime(9), false);
        assert_eq!(prime(11), true);
        assert_eq!(prime(13), true);
        assert_eq!(prime(15), false);
        assert_eq!(prime(101), true);
    }
}
