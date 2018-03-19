// Project Euler problem 32 - Pandigital products
//
// We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once; for example, the 5-digit number, 15234, is 1 through 5 pandigital.
//
// The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing multiplicand, multiplier, and product is 1 through 9 pandigital.
//
// Find the sum of all products whose multiplicand/multiplier/product identity can be written as a 1 through 9 pandigital.
// HINT: Some products can be obtained in more than one way so be sure to only include it once in your sum.

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
fn main() {
    println!("Hello, world!");
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
}
