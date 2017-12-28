#![feature(inclusive_range_syntax)]
#![feature(iterator_step_by)]

extern crate itertools;
use itertools::Itertools;

fn main() {

    let mut highest_palindrome: u64 = 0;
    let mut product: u64;

    let three_digits = (100..1000).cartesian_product((100..1000));
    println!("3 digits size {}", three_digits.clone().count());
    for (a, b) in three_digits {
        product = a * b;
        let pal = palindrome(product);
        if pal && product > highest_palindrome {
            highest_palindrome = product;
            println!("pal {} - {}, a {}, b {}", product, pal, a, b);
        }
    }

    if highest_palindrome == 0 {
        println!("There are no palindromes which are the products of three digit numbers")
    } else {
        println!("The highest palindrome that is the product of two three digit numbers is: {} ",
            highest_palindrome);
    }
}

fn palindrome(number: u64) -> bool {
    let n = number.to_string();
    let u = n.chars().rev().collect::<String>();
    u == n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sift() {
        let x = 998001;
        assert_eq!(palindrome(x), false);
        assert_eq!(palindrome(545), true);
        assert_eq!(palindrome(52425), true);
    }
}
