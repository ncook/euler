// Distinct powers - problem 29

// Consider all integer combinations of ab for 2 ≤ a ≤ 5 and 2 ≤ b ≤ 5:
//
//     2**2=4, 2**3=8, 2**4=16, 2**5=32
//     3**2=9, 3**3=27, 3**4=81, 3**5=243
//     4**2=16, 4**3=64, 4**4=256, 4**5=1024
//     5**2=25, 5**3=125, 5**4=625, 5**5=3125
//
// If they are then placed in numerical order, with any repeats removed, we get the following
// sequence of 15 distinct terms:
//
// 4, 8, 9, 16, 25, 27, 32, 64, 81, 125, 243, 256, 625, 1024, 3125
//
// How many distinct terms are in the sequence generated by a**b for 2 ≤ a ≤ 100 and 2 ≤ b ≤ 100?

#![feature(inclusive_range_syntax)]
#![feature(iterator_step_by)]

use std::collections::HashSet;

extern crate num_integer;

fn main() {
    let mut distinct: HashSet<Vec<(u64, u64)>> = HashSet::new();
    for a in 2..=100 {
        let mut fs = factor(a);
        for b in 2..=100 {
            let full_factor = fs.iter().map(|&(f, t)| (f, t*b)).collect();
            distinct.insert(full_factor);
        }
    }
    println!("Distinct factors: {:?}", distinct.len());
    println!("Factors: {:?}", distinct);
    // for a in (2..100) {
    //     let mut entries: Vec<(u64, u64)> = Vec::new();
    //     distinct.contains((2, ))
    //     println!("For {}: {:?}");
    // }
}

fn factor(number: u64) -> Vec<(u64, u64)> {
    use num_integer::Integer;

    let mut factors: Vec<(u64, u64)> = Vec::new();
    let mut rem = number;
    let mut times = 0;
    while rem > 1 {
        let (d, r) = rem.div_rem(&2);
        if r != 0 { break }
        record_factor(&mut factors, 2);
        times += 1;
        rem = d;
    }
    if times > 0 {
        println!("factor: {}, {}", 2, times);
        record_factor(&mut factors, 2);
    }

    let mut found_factor = false;
    while rem > 1 {
        let sqrt = f64::floor(f64::sqrt(rem as f64)) as u64;
        for i in (3..=sqrt).step_by(2) {
            let (d, r) = rem.div_rem(&i);
            if r == 0 {
                record_factor(&mut factors, d);
                found_factor = true;
                rem = d;
                break
            }
        }
        // We have two paths to this point. found_factor allows us to break the loop
        if found_factor {
            found_factor = false;
        } else {
            // rem is prime
            record_factor(&mut factors, rem);
            rem = 1;
        }
    }
    factors
}

fn record_factor(factors: &mut Vec<(u64, u64)>, factor: u64) {
    if let Some(mut most_recent_factor) = factors.pop() {
        if most_recent_factor.0 == factor {
            most_recent_factor.1 += 1;
            println!("factor, count: {:?}", most_recent_factor);
            factors.push(most_recent_factor);
        } else {
            println!("factor, one: {}, 1", factor);
            factors.push((factor, 1));
        }
    } else {
        println!("factor, 1: {}, 1", factor);
        factors.push((factor, 1));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factor() {
        assert_eq!(factor(17), [(17, 1)]);
        assert_eq!(factor(24), [(2, 3)]);
        assert_eq!(factor(40), [(2, 3), (5, 1)]);
        assert_eq!(factor(12), [(2, 2), (3, 1)]);
    }
}