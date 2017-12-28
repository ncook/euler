#![feature(inclusive_range_syntax)]
#![feature(iterator_step_by)]

fn main() {

    // Filter all numbers which are not factors out. Factors reduce the remainder.
    // println!("number: {:?} sqrt: {:?}, squared: {:?}", number, sqrt);
    // let mut target = 600851475143 as u64;
    let mut target = 698896 as u64;

    println!("number: {:?}", target);

    let mut high_so_far = 1;
    while let Some(factor) = sift(target, high_so_far) {
        target = target / factor;
        if high_so_far < factor {
            high_so_far = factor;
        }
        println!("factor: {:?} ", factor);
    }
    println!("highest prime factor: {:?} ", target);
}

fn sift(target: u64, min_factor: u64) -> Option<u64> {
    if target < 4 {
        return Some(target);
    }

    if (target / 2) * 2 == target {
        return Some(2);
    }

    let sqrt = f64::floor(f64::sqrt(target as f64)) as u64;
    let low = u64::max(3, min_factor);
    let range = low..=sqrt;

    range
        .step_by(2)
        // .inspect(|x| println!("target: {:?}, x: {:?}, target/x {:?}", target, x, target / x))
        .find(|x| (target / x) * x == target )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sift() {
        let x = 12;
        assert_eq!(sift(x, 1), Some(2));
        assert_eq!(sift(6, 2), Some(2));
        assert_eq!(sift(3, 2), Some(3));
        assert_eq!(sift(13195, 2), Some(5));
        assert_eq!(sift(7, 1), None);
    }
}
