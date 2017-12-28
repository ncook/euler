// Two equations with three unknowns:
// a^2 + b^2 = c^2
// a + b + c = 1000
// a < b < c
// For c to be greater than a and b, it can not be less than 334 and can not be
// more than 998 for their sum to be 1000. b can not be less than half of
// 1000 - c.
fn main() {
    let target = 1000;
    let mut a: u64 = 0;
    (334..(target-1)).any(|c|
        ((target-c)/2..(target-c)).any(|b| {
            a = target - c - b;
            // if a + b + c != 1000 {
            //     panic!();
            // }
            if is_triplet(a, b, c) {
                println!("Pythagorean triplet summing to 1000: {} {} {}", a, b, c);
                println!("Product is {}", a*b*c);
                true
            } else { false }
        })
    );
}

fn is_triplet (a: u64, b: u64, c: u64) -> bool {
    if a*a + b*b == c*c { true } else { false }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_of() {
        assert_eq!(is_triplet(3, 4, 5), true);
        assert_eq!(is_triplet(4, 3, 5),true);
        assert_eq!(is_triplet(6, 8, 10), true);
        assert_eq!(is_triplet(4, 3, 15), false);
    }
}
