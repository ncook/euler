// Project Euler problem 32 - Pandigital products
//
// We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n
// exactly once; for example, the 5-digit number, 15234, is 1 through 5 pandigital.
//
// The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing multiplicand,
// multiplier, and product is 1 through 9 pandigital.
//
// Find the sum of all products whose multiplicand/multiplier/product identity can be written as a
// 1 through 9 pandigital.
// HINT: Some products can be obtained in more than one way so be sure to only include it once in
// your sum.

// At least one digit must be part of the multiplier and multiplicand. Therefore at most seven
// digits can be part of the product.
// However two numbers multiplied together to produce a seven digit number would require at least
// seven digits between them.
// For a six digit number, the sum of the digits of the products would be at least six.
// However for a five digit number, the sum of the digits of the products would be at least five.
// For a three digit number the number multiplied together would produce at least five digits in
// the result. The situation would be even worse for a two or one digit product.
// So, only four digit products with the multiplicand and multiplier having a total of five digits
// between them will work.

// Only the products of 1 and 4 digit numbers and 2 and 3 digit numbers may have product that are
// in the range where the total digits may sum to nine.
// 1x4 4-5 9-10
// 2x3 4-5 9-10

extern crate num_integer;
use num_integer::Integer;
use std::collections::HashSet;

fn main() {
    let target: HashSet<_> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter().collect();
    let mut digits = HashSet::new();
    for i in 1..=99 {
        for d in into_digits(i) {
            digits.insert(d);
        }
    }
    println!("Hello, world!");
}

fn into_digits(number: u64) -> HashSet<u8> {
    let mut digits = HashSet::new();
    while number > 9 {
        let (number, remainder) = number.div_rem(&10);
        digits.insert(remainder as u8);
    }
    digits.insert(number as u8);
    digits
}

fn add_digits(digits: HashSet<u8>, existing: HashSet<u8>) -> HashSet<u8> {
    if existing.is_disjoint(&digits) {
        return existing.union(&digits).iter().collect::<HashSet<u8>>()
    }
    existing
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_digits() {
        assert_eq!(into_digits(40), vec![4, 0]);
        assert_eq!(into_digits(41), vec![4, 1]);
        assert_eq!(into_digits(991), vec![9, 9, 1]);
        assert_eq!(into_digits(997), vec![9, 9, 7]);
        assert_eq!(into_digits(1601), vec![1, 6, 0, 1]);
    }
}
