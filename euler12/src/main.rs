#![feature(test)]
#![feature(inclusive_range_syntax)]
#![feature(iterator_step_by)]

extern crate test;
extern crate itertools;
use itertools::iterate;

// The nth triangle number is the sum of the n positive integers.
// The formula for summing positive integers is: n(n+1)/2
// However the nth one is n higher than the n-1th one.

struct Triangle {
    number: u64,
    current: u64,
    previous: u64
}

impl Triangle {
    fn new() -> Triangle {
        Triangle {
            number: 0,
            current: 0,
            previous: 0,
        }
    }
}

impl Iterator for Triangle {
    type Item = (u64, u64);
    fn next(&mut self) -> Option<Self::Item> {
        self.number += 1;
        let i = self.current;
        self.current = self.current + self.number;
        self.previous = i;
        Some((self.number, self.current))
    }
}

fn main() {
    // Start at the 34th triangular number - a very conservative choice
    let tri_iter = Triangle::new();
    let result = tri_iter
        .map(|(n, t)| (n, t, ndivisors(t)))
        .inspect(|k| println!("n: {}, tri: {}, ndivisors: {}", k.0, k.1, k.2))
        .find(|l| l.2 > 20);

    if let Some(tri) = result {
        println!("First triangular number to have more than 500 divisors: {}", tri.1);
    } else {
        println!("No such triangular number.")
    }
}

fn ndivisors(x: u64) -> u64 {
    let mut n: u64 = 2; // x itself and 1 are always divisors
    n += iterate(2, |i| i + 1)
        .take_while(|j| *j <= (x / 2))
        .filter(|k| factor(x, *k))
        .count() as u64;
    n
}

fn factor(n: u64, possible: u64) -> bool {
    if n % possible == 0 { true } else { false }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_ndivisors(b: &mut Bencher) {
        b.iter(|| ndivisors(630));
    }

    #[test]
    fn test_ndivisors() {
        assert_eq!(ndivisors(120), 16);
        assert_eq!(ndivisors(23787753), 24);
        assert_eq!(ndivisors(76576500), 576);
    }

    #[test]
    fn test_iter() {
        let mut t = Triangle::new();
        assert_eq!(t.next(), Some((1, 1)));
        assert_eq!(t.next(), Some((2, 3)));
        assert_eq!(t.next(), Some((3, 6)));
        assert_eq!(t.next(), Some((4, 10)));
        assert_eq!(t.next(), Some((5, 15)));
        assert_eq!(t.next(), Some((6, 21)));
        assert_eq!(t.next(), Some((7, 28)));
    }
}
