// Digit fifth powers - problem 30
//
//
// Surprisingly there are only three numbers that can be written as the sum of fourth powers of their digits:
//
//     1634 = 1**4 + 6**4 + 3**4 + 4**4
//     8208 = 8**4 + 2**4 + 0**4 + 8**4
//     9474 = 9**4 + 4**4 + 7**4 + 4**4
//
// As 1 = 1**4 is not a sum it is not included.
//
// The sum of these numbers is 1634 + 8208 + 9474 = 19316.
//
// Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.

// The highest sum of a six digit number is 354,294 with all 9s as the digits (59049 * 6). For a
// seven digit number, the highest sum is 413,344. So, the highest number that could possibly be
// equal to the sum of the fifth powers of its' digits is 354,294 or less.
// Solutions having:
// 1 digit: Excluded by the problem defintiion
// 2 digits: Can only include digits of 2 or less. 10, 11, 12, 20, 21, 22 all fail obvioulsy.
// 3 digits: Can only include digits of 3 or less. They must include a 3 to have three digits.
//   Having more than one 3 would include a digit higher than 3.
//   300: 243, 301: 244, 302: 275, 310: 244, 311: 245, 312: 276, 320: 275, 321: 276, 322: 307
// 4 digits: The digits must be 5 or less. A 6 would introduce a 7 in the sum in the first place.
//   5000: 3125, (5001, 5010, 5100): 3126, (5002, 5020, 5200): 3157, (5003, 5030, 5300): 3368,
//   (5004, 5040, 5400): 4149, (5005, 5050, 5500): 6250, (5011, 5101, 5110): 3127,
//   (5012, 5021, 5102, 5120, 5201, 5210): 3158
//   5013 combined: 3369. 5014 combined: 4150 = 4150 and 4151

extern crate num_integer;

use num_integer::Integer;

fn main() {
    // let fp_digits: Vec<usize> = (1..355_000_000)
    let fp_digits: Vec<usize> = (2..400_000)
        .map(|a| (a as usize, fifth_power_sum2(a)))
        // .inspect(|&(a, b)| println!("n, pow: {}, {}", a, b))
        .filter(|&(n, fp)| fp == (n as u64))
        .map(|(n, _fp)| n )
        .collect();
    println!("All the numbers whose digit's fifth power's summer equal the number");
    println!("{:?}", fp_digits);
    println!("Sum of all such numbers: {}", fp_digits.iter().sum::<usize>());
}

fn fifth_power(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        2 => 32,
        3 => 243,
        4 => 1024,
        5 => 3125,
        6 => 7776,
        7 => 16807,
        8 => 32768,
        9 => 59049,
        _ => n*n*n*n*n,
    }

}

fn fifth_power_sum(number: Vec<u64>) -> u64 {
    number.into_iter().map(fifth_power).sum()
}

fn fifth_power_sum2(number: u64) -> u64 {
    let mut sum = 0;
    let mut number = number;
    loop {
        let (d, r) = number.div_rem(&10);
        sum += fifth_power(r);
        number = d;
        if number == 0 { break }
    }
    sum
}

fn digit_sum(number: &[u64]) -> u64 {
    number.iter().fold(0, |sum, d| sum * 10 + *d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fifth_power_sum() {
        assert_eq!(fifth_power_sum(vec![1, 1, 1]), 3);
        assert_eq!(fifth_power_sum(vec![3, 2, 1]), 276);
        assert_eq!(fifth_power_sum(vec![10, 0, 1]), 100001);
    }

    #[test]
    fn test_fifth_power_sum2() {
        assert_eq!(fifth_power_sum2(111), 3);
        assert_eq!(fifth_power_sum2(321), 276);
    }

    #[test]
    fn test_digit_sum() {
        assert_eq!(digit_sum(&[1, 0, 0]), 100);
        assert_eq!(digit_sum(&[9, 9, 9]), 999);
        assert_eq!(digit_sum(&[1, 2, 3, 4, 5, 6]), 123456);
        assert_eq!(digit_sum(&[5, 6, 9, 9, 0]), 56990);
    }

    #[test]
    fn test_fifth_power() {
        assert_eq!(fifth_power(1), 1);
        assert_eq!(fifth_power(2), 32);
        assert_eq!(fifth_power(3), 243);
        assert_eq!(fifth_power(4), 1024);
        assert_eq!(fifth_power(5), 3125);
        assert_eq!(fifth_power(6), 7776);
        assert_eq!(fifth_power(7), 16807);
        assert_eq!(fifth_power(8), 32768);
        assert_eq!(fifth_power(9), 59049);
        assert_eq!(fifth_power(0), 0);
    }
}
