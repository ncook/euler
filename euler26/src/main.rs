// Reciprocal cycles - Problem 26
//
// A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions with denominators 2 to 10 are given:
//
//     1/2	= 	0.5
//     1/3	= 	0.(3)
//     1/4	= 	0.25
//     1/5	= 	0.2
//     1/6	= 	0.1(6)
//     1/7	= 	0.(142857)
//     1/8	= 	0.125
//     1/9	= 	0.(1)
//     1/10	= 	0.1
//
// Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has a 6-digit recurring cycle.
//
// Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.

#![feature(inclusive_range_syntax)]

extern crate num_integer;

fn main() {
    let mut max = 1;
    let mut max_denom = 0;
    for denom in 2..1000 {
        let fr = fraction(denom);
        // println!("denom, fraction: {}, {:?}", denom, fr);
        let pattern_len = max_pattern(&fr);
        println!("denom, len: {}, {}", denom, pattern_len);
        if pattern_len > max {
            max = pattern_len;
            max_denom = denom
        };
    }
    println!(
        "Denominator {} has the longest recurring fraction sequence: {}",
        max_denom, max
    );
}

fn fraction(denominator: u32) -> Vec<u32> {
    use num_integer::Integer;

    if denominator < 2 {
        panic!("Only implemented for integers of 2 or greater")
    }
    let mut numerator = 10;
    let mut fraction: Vec<u32> = Vec::new();
    loop {
        let (digit, rem) = numerator.div_rem(&denominator);
        fraction.push(digit);
        if rem == 0 {
            break;
        }
        if fraction.len() > 5000 {
            break;
        }
        numerator = rem * 10;
    }
    fraction
}

fn max_pattern(fraction: &[u32]) -> usize {
    let start_index = fraction.len() - 1;
    if start_index <= 0 {
        return 1;
    }
    let mut recurring_len = 0;
    let mut cur_index = start_index - 1;
    let mut pattern_len = 0;
    loop {
        if fraction[cur_index] == fraction[start_index] {
            // This could be the start of a match
            pattern_len = start_index - cur_index;
            // println!("cur_index, pattern_len: {}, {}", cur_index, pattern_len);
            // println!("Pattern is: {:?}", &fraction[start_index + 1 - pattern_len..=start_index]);
            // println!("test is a: {:?} b: {:?}", &fraction[cur_index + 1 - pattern_len..=cur_index],
            //     &fraction[start_index + 1 - pattern_len..=start_index]);
            if (cur_index + 1) >= pattern_len
                && fraction[cur_index + 1 - pattern_len..=cur_index]
                    == fraction[start_index + 1 - pattern_len..=start_index]
            {
                recurring_len = pattern_len;
                // println!("Pattern length: {}", recurring_len);
                if pattern_len > 1 {
                    let the_same = fraction[start_index];
                    if fraction[start_index + 1 - pattern_len..=start_index]
                        .iter()
                        .all(|&d| d == the_same ) {
                            // This is a sequence where all the digits are the same
                            recurring_len = 1;
                        }
                    break
                }
            }
        }
        if (cur_index == 0) || (pattern_len >= cur_index) { break }
        cur_index -= 1;
    }
    println!("Pattern length at end: {}", recurring_len);
    recurring_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_pattern() {
        assert_eq!(max_pattern(&[3, 3, 3]), 1);
        assert_eq!(max_pattern(&[0, 2, 5]), 0);
        assert_eq!(max_pattern(&[1, 6, 6]), 1);
        assert_eq!(max_pattern(&[0, 5, 2, 6, 3, 1, 5, 7, 8, 9, 4, 7, 3, 6, 8, 4, 2, 1, 0, 5, 2, 6, 3, 1, 5, 7, 8, 9, 4, 7, 3, 6, 8, 4, 2, 1, 0, 5]), 18);
        assert_eq!(max_pattern(&[0, 5, 8, 8, 2, 3, 5, 2, 9, 4, 1, 1, 7, 6, 4, 7, 0, 5, 8, 8, 2, 3, 5, 2, 9, 4, 1, 1, 7, 6, 4, 7, 0, 5, 8, 8, 2, 3, 5, 2, 9, 4, 1, 1, 7, 6, 4, 7, 0, 5, 8, 8, 2, 3, 5, 2, 9, 4, 1, 1, 7, 6, 4, 7, 0, 5, 8, 8, 2, 3, 5, 2, 9, 4, 1, 1]), 16);
    }
}
