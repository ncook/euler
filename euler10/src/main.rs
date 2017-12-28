#![feature(inclusive_range_syntax)]
#![feature(iterator_step_by)]

fn main() {
    // Sum of squares - square of sum
    let target = 2_000_000;
    let primes_sum = sum_primes_below(target);
    println!("Prime sum below {}: {}", target, primes_sum);
}

fn sum_primes_below(target: u64) -> u64 {
    (2..target).filter(|&x| prime(x) ).sum()
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
        assert_eq!(sum_primes_below(10), 17);
        assert_eq!(sum_primes_below(12), 28);
        assert_eq!(sum_primes_below(20), 77);
    }
}
