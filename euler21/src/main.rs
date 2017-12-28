#![feature(inclusive_range_syntax)]

use std::collections::HashMap;

fn main() {
    const LIMIT: usize = 10_000;
    let mut pd_sum: HashMap<usize, usize> = HashMap::new();
    let mut amicable = Vec::new();
    for num_a in 1..LIMIT {
        // I insert the sum and the number. When the sum is the same as a different number then we
        // have amicable numbers. The value of the HashMap is the second of the numbers.
        let sum_a = proper_divisors_sum(num_a);
        pd_sum.insert(sum_a, num_a);
        // If we can get num_a then it equals sum_b
        if let Some(&num_b) = pd_sum.get(&num_a) {
            if num_b != num_a && num_b == sum_a {
                amicable.push(num_a);
                amicable.push(num_b);
            }
        }
    }
    println!("{:?}", amicable);
    println!("There are {} amicable numbers less than {} and their sum is {}",
        amicable.len(), LIMIT, amicable.into_iter().sum::<usize>());
}


fn proper_divisors_sum(number: usize) -> usize {
    (1usize..=(number / 2)).fold(0usize, |acc, x| acc + if number % x == 0 { x } else { 0 })
}

// fn amicable(a: u64, b: u64) -> bool {
//
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_cost() {
        assert_eq!(proper_divisors_sum(42), 54); //1, 2, 3, 6, 7, 14, 21 sum=54
        assert_eq!(proper_divisors_sum(284), 220);
        assert_eq!(proper_divisors_sum(220), 284);
        assert_eq!(proper_divisors_sum(1184), 1210);
        assert_eq!(proper_divisors_sum(1210), 1184);
        assert_eq!(proper_divisors_sum(5020), 5564);
        assert_eq!(proper_divisors_sum(5564), 5020);
        assert_eq!(proper_divisors_sum(6368), 6232);
        assert_eq!(proper_divisors_sum(6232), 6368);
    }
}
