// 2**15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
// What is the sum of the digits of the number 21000?

// Each power of 2 increases the length of the number by 1/3. So 2**1000
// would have about 333 digits and be beyond the range of even u128.

fn main() {
    let mut arr = vec![1];
    for _ in 0..1000 { double(&mut arr) };
    println!("2**1000 digits summed is: {}", sum_digits(arr));
}

fn double(ref mut num: &mut Vec<u8>) {
    let mut carry = false;
    for mut d in num.iter_mut() {
        let mut d2 = if carry { *d * 2 + 1 } else { *d * 2 };
        if d2 > 9 {
            carry = true;
            d2 -= 10;
        } else {
            carry = false;
        }
        *d = d2;
    }
    if carry {
        num.push(1);
    }
}

fn sum_digits(num: Vec<u8>) -> u64 {
    num.iter().fold(0, |sum, &d| sum + u64::from(d))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_cost() {
        let mut arr = vec![1u8, 2u8];
        double(&mut arr);
        assert_eq!((arr), [2, 4]);
        double(&mut arr);
        assert_eq!((arr), [4, 8]);
        double(&mut arr);
        assert_eq!((arr), [8, 6, 1]);
        double(&mut arr);
        assert_eq!((arr), [6, 3, 3]);
        arr = vec![8u8, 6, 7, 2, 3];
        assert_eq!(sum_digits(arr), 26);
        arr = vec![1];
        for _ in 0..15 { double(&mut arr) };
        assert_eq!(sum_digits(arr), 26);
        assert_eq!(sum_digits(vec!(1, 2, 3)), 6);
        assert_eq!(sum_digits(vec!(9, 9, 9)), 27);
        assert_eq!(sum_digits(vec!(1, 2, 3, 9, 9)), 24);
    }
}
