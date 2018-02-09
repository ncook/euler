// Lexicographic permutations
// Problem 24
//
// A permutation is an ordered arrangement of objects. For example, 3124 is one possible
// permutation of the digits 1, 2, 3 and 4. If all of the permutations are listed numerically or
// alphabetically, we call it lexicographic order. The lexicographic permutations of 0, 1 and 2 are:
//
// 012   021   102   120   201   210
//
// What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?

extern crate num_integer;

use num_integer::Integer;

const FACTS: &[u64] = &[1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800];
const LEX_ORDER: [u8; 10] = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];

fn main() {
    // There are 10! permutations of ten separate things. That is 3,628,800.
    // 9! = 362,880 8! = 40,320 7! = 5,040 6! = 720 5! = 120 4! = 24 3! = 6
    // In lexicographic order, there will be 9! permutations starting with 0 or 362,880.
    // That means that the millionth one will start with 2 since the first starting with 2 will
    // be the 725,761th one and the last will be the 1,088,640th,
    // Of those starting with 2, 8! will follow with a 0 and similarly for 1 etc.
    // 2
    // 725761                                                                           1088640
    //  0      1      3      4      5       6       7       8       9
    //  766081 806401 846721 887041 927361  967681

    let digits = nth_perm(1_000_000, 10);
    // let mut rem = 1_000_000;
    // let mut fidx = 8;
    // let mut digits = String::new();
    //
    // loop {
    //     let (d, r) = rem.div_rem(&FACTS[fidx]);
    //     digits += &d.to_string();
    //     rem = r;
    //     if fidx == 0 { break }
    //     fidx -= 1;
    // }
    println!("The nth digit order is: {}", digits);
}

fn digit_order(nth: u64, nvalues: u64) -> Vec<u8> {
    let mut rem = nth - 1;
    let mut so_far = 0;
    let mut fidx = (nvalues - 2) as usize;
    let mut digits = Vec::<u8>::new();

    loop {
        let (d, r) = rem.div_rem(&FACTS[fidx]);
        digits.push(d as u8);
        rem = r;
        so_far += FACTS[fidx] * d;
        println!("{:?}, {:?}, {:?}", so_far, d, r);
        if fidx == 0 {
            digits.push(0);
            break
        }
        fidx -= 1;
    }
    digits
}

fn nth_perm(nth: u64, nvalues: u64) -> String {
    let digits = digit_order(nth, nvalues);
    let mut sequence = String::new();
    let mut available: Vec<u8> = From::from(&LEX_ORDER[..]);
    for digit in digits {
        let next = available.remove(digit as usize);
        sequence.push((next + ('0' as u8)) as char);
    }
    sequence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_perm() {
        assert_eq!(nth_perm(1, 3), "012");
        assert_eq!(nth_perm(2, 3), "021");
        assert_eq!(nth_perm(3, 3), "102");
        assert_eq!(nth_perm(4, 3), "120");
        assert_eq!(nth_perm(5, 3), "201");
        assert_eq!(nth_perm(6, 3), "210");
    }
}
