#![feature(inclusive_range_syntax)]
// 10! = 362800 and the sum of its digits is 3 + 6 + 2 + 8 + 0 + 0 = 27.
// What is the sum of the digits of the number 100!?

// Each multiplication lengthens the number. From 11..100 it will tend towards two digits each
// time. I estimate about 70 digits in total which is beyond the range of u128.

// To avoid overflow the number will be represented as a vector of digits.

fn main() {
    let mut number: Vec<u16> = Vec::with_capacity(100);
    for j in 10u16..101 {
        number.clear();
        number.push(1);
        for i in 1u16..=j { mult(i, &mut number); };
        println!("{}! ({:?}) digits summed is: {}", j, number, sum_digits(&number));
    }
}

fn mult(op: u16, ref mut num: &mut Vec<u16>) {
    let mut carry = 0;
    for mut d in num.iter_mut() {
        let mut d2 = *d * op + carry;
        carry = d2 / 10;
        d2 = d2 % 10;
        *d = d2;
    }
    while carry > 0 {
        num.push(carry % 10);
        carry = carry / 10;
    }
}

fn sum_digits(num: &Vec<u16>) -> u64 {
    num.iter().fold(0, |sum, &d| sum + u64::from(d))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_cost() {
        let mut arr = vec![1u16];
        for i in 1..6 { mult(i, &mut arr) };
        assert_eq!(arr, vec![0u16, 2, 1]);
        for i in 6..11 { mult(i, &mut arr) };
        assert_eq!(arr, vec![0u16, 0, 8, 8, 2, 6, 3]);
        assert_eq!(sum_digits(&vec![3u16, 6, 2, 8, 8, 0, 0]), 27);
    }
}
