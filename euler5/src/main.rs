#![feature(inclusive_range_syntax)]
#![feature(iterator_step_by)]

extern crate itertools;
use itertools::iterate;

fn main() {
    // A number evenly divisible by the numbers 1..=20 if it is always divisible by the numbers
    // 3, 7, 11, 13, 17, 16, 20

    let factors = [7, 9, 11, 13, 16, 17, 19, 20];
    let result = iterate(20, |&i| i + 1).find(|j| factors.iter().all(|k| even_div(*j, *k)));

    if let Some(result) = result {
        println!("The lowest positive number that has 1..=20 as factors is {}", result)
    } else {
        println!("What! No way!");
    }
}

fn even_div(target: u64, number: u64) -> bool {
    target % number == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_div() {
        let x = 232792560;
        assert_eq!(even_div(x, 2), true);
        assert_eq!(even_div(x, 3), true);
        assert_eq!(even_div(x, 4), true);
        assert_eq!(even_div(x, 5), true);
        assert_eq!(even_div(x, 6), true);
        assert_eq!(even_div(x, 7), true);
        assert_eq!(even_div(x, 8), true);
        assert_eq!(even_div(x, 9), true);
        assert_eq!(even_div(x, 19), true);
        assert_eq!(even_div(x, 11), true);
        assert_eq!(even_div(x, 12), true);
        assert_eq!(even_div(x, 13), true);
        assert_eq!(even_div(x, 14), true);
        assert_eq!(even_div(x, 15), true);
        assert_eq!(even_div(x, 16), true);
        assert_eq!(even_div(x, 17), true);
        assert_eq!(even_div(x, 18), true);
        assert_eq!(even_div(x, 19), true);
        assert_eq!(even_div(x, 20), true);
    }
}
