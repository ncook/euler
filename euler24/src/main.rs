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

const FACTS: &[u64] = &[1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

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

fn nth_perm(nth: u64, nvalues: u64) -> String {
    let mut rem = nth;
    let mut fidx = (nvalues - 1) as usize;
    let mut digits = String::new();

    loop {
        let (d, r) = rem.div_rem(&FACTS[fidx]);
        digits += &d.to_string();
        rem = r;
        if fidx == 0 { break }
        fidx -= 1;
    }
    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_perm() {
        assert_eq!(nth_perm(1, 3), "123");
    }
}
