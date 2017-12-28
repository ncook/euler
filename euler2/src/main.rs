struct Fib {
    current: u64,
    previous: u64
}

impl Fib {
    fn new() -> Fib {
        Fib { current: 1, previous: 0 }
    }
}

impl Iterator for Fib {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let i = self.previous + self.current;
        self.previous = self.current;
        self.current = i;
        Some(i)
    }
}

fn main() {
    // let sum: u64 = (2..4000000).scan(0, |acc, i| {};
    let sum: u64;
    let fib = Fib::new();
    sum = fib.filter(|&item| item % 2 == 0 ).take_while(|x| *x < 4_000_000).sum();
    println!("fib sum of evens less than 4,000,000 {:?}", sum);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_sequence() {
        let mut fib = Fib::new();
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(2));
        assert_eq!(fib.next(), Some(3));
        assert_eq!(fib.next(), Some(5));
        assert_eq!(fib.next(), Some(8));
        assert_eq!(fib.next(), Some(13));
        assert_eq!(fib.next(), Some(21));
    }
}
