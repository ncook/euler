// Quadaratic primes - problem 27
//
// Euler discovered the remarkable quadratic formula:
//
// n2+n+41
//
// It turns out that the formula will produce 40 primes for the consecutive integer values 0≤n≤39
// . However, when n=40,402+40+41=40(40+1)+41 is divisible by 41, and certainly when n=41,412+41+41
//
// is clearly divisible by 41.
//
// The incredible formula n2−79n+1601
// was discovered, which produces 80 primes for the consecutive values 0≤n≤79
//
// . The product of the coefficients, −79 and 1601, is −126479.
//
// Considering quadratics of the form:
//
//     n2+an+b
//
// , where |a|<1000 and |b|≤1000
//
// where |n|
// is the modulus/absolute value of n
// e.g. |11|=11 and |−4|=4
//
// Find the product of the coefficients, a
// and b, for the quadratic expression that produces the maximum number of primes for consecutive
// values of n, starting with n=0.

#![feature(inclusive_range_syntax)]
#![feature(iterator_step_by)]

fn main() {
    let mut max_primes = 0;
    let mut max_a = 0;
    let mut max_b = 0;
    let mut n: u64;

    for a in -999i64..1000 {
        for b in -1000i64..=1000 {
            n = prime_sequence(a, b);
            if n > 0 { println!{"a, b, primes: {}, {}, {}", a, b, n} }
            if n > max_primes { max_primes = n; max_a = a; max_b = b }
        }
    }
    println!("Most primes: {}, generated by a, b, product: {}, {}, {}",
        max_primes, max_a, max_b, max_a * max_b);
}

fn prime_sequence(a: i64, b: i64) -> u64 {
    let mut n = 0;
    let mut val: i64;

    loop {
        val = n * n + a * n + b;
        if val < 2 || !is_prime(val as u64) { break; }
        n += 1;
    }
    n as u64
}

fn is_prime(number: u64) -> bool {
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
    fn test_is_prime() {
        assert_eq!(is_prime(40), false);
        assert_eq!(is_prime(41), true);
        assert_eq!(is_prime(991), true);
        assert_eq!(is_prime(997), true);
        assert_eq!(is_prime(1601), true);
    }

    #[test]
    fn test_prime_sequence() {
        assert_eq!(prime_sequence(1, 41), 40);
        assert_eq!(prime_sequence(-79, 1601), 80);
        assert_eq!(prime_sequence(-999, -899), 0);
        assert_eq!(prime_sequence(-61, 971), 71);
    }
}
