#![feature(inclusive_range_syntax)]
// Find the sum of all the positive integers which cannot be written as the sum of two abundant
// numbers.
//
// Given that all integers greater than 28123 CAN be written as the sum of two abundant numbers.
// We need to enumerate the ones which are less than or equal to 28123.
// Given that the smallest abundant number is 12, all numbers less than 24 meet the goal.

fn main() {
    let mut perfect = Vec::new();
    let mut abundant = Vec::new();
    let mut deficient = Vec::new();

    for i in 1u64..28123 {
        let pd = proper_divisors_sum(i);
        // println!("pd_sum: {:?} {:?}", pd, i);
        match i {
            n if pd > i => abundant.push(n),
            n if pd < i => deficient.push(n),
            n => perfect.push(n),
        }
    }
    println!("perfect: {:?}", perfect);
    // if perfect.len() > 0 { println!("perfect: {:?}", perfect) }
    // println!("deficient: {:?}", deficient);
    println!("abundant: {:?}", abundant);
    // let r: Vec<u64> = abundant.into_iter().filter(|a| *a % 2 != 0).collect();
    // println!("oddly abundant: {:?}", r);

    // The lowest abundant number which is odd is 945. Thus there can be no odd numbers which are
    // the sum of two abundant numbers that are less than 945+12 = 957.

    let mut sum = 0;
    for i in 1..=28123 {
        match i {
            _ if i < 24 => more_abundant(i, &mut sum, 1),
            _ if (i % 2 != 0) && (i < 957) => more_abundant(i, &mut sum, 2),
            _ if !pick_two_abundant(i, &abundant) => more_abundant(i, &mut sum, 3),
            _ => (),
        }
    }
    println!("Sum of all numbers which are not the sum of two abundant numbers, but less than 28124: {}",
        sum);
}

fn proper_divisors_sum(number: u64) -> u64 {
    (1u64..=(number / 2)).fold(0u64, |acc, x| acc + if number % x == 0 { x } else { 0 })
}

fn more_abundant(new: u64, sum: &mut u64, p: u64) {
    *sum += new;
    println!("{} {}", p, new);
}

fn pick_two_abundant(target: u64, abundant: &[u64]) -> bool {
    let mut found = false;

    for i in abundant {
        if *i >= target { break }
        for j in abundant {
            if (*i + *j) > target { break }
            if (*i + *j) == target { found = true; break }
        }
        if found { break }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abundant() {
        let abundant = vec![12u64, 18, 20, 24, 30, 36, 40, 42, 48, 54, 56, 60];

        assert_eq!(pick_two_abundant(16, &abundant), false);
        assert_eq!(pick_two_abundant(26, &abundant), false);
        assert_eq!(pick_two_abundant(27, &abundant), false);
        assert_eq!(pick_two_abundant(28, &abundant), false);
        assert_eq!(pick_two_abundant(30, &abundant), true);
        assert_eq!(pick_two_abundant(32, &abundant), true);
        assert_eq!(pick_two_abundant(34, &abundant), false);
        assert_eq!(pick_two_abundant(36, &abundant), true);
    }
}
