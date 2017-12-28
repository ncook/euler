#![feature(inclusive_range_syntax)]
#![feature(dotdoteq_in_patterns)]

fn main() {
    let arr = [
        75,
        95, 64,
        17, 47, 82,
        18, 35, 87, 10,
        20, 04, 82, 47, 65,
        19, 01, 23, 75, 03, 34,
        88, 02, 77, 73, 07, 63, 67,
        99, 65, 04, 28, 06, 16, 70, 92,
        41, 41, 26, 56, 83, 40, 80, 70, 33,
        41, 48, 72, 33, 47, 32, 37, 16, 94, 29,
        53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14,
        70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57,
        91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48,
        63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31,
        04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23,
    ];

    const MAX: usize = 120;
    let mut cost = vec![0usize; MAX];

    let mut row_len = 15u32;
    let mut row_idx = 0u32;
    for r in (0usize..MAX).rev() {
        cost[r] = value(&arr, &cost, r, row_len as usize, MAX);
        println!("idx, cell, cost: {} {} {}", r, arr[r], cost[r]);
        row_idx += 1;
        if row_idx == row_len {
            row_len -= 1;
            row_idx = 0;
        }
    }
    let sum: usize = cost[0];
    println!("The highest value path is: {}", sum);
}

fn value(arr: &[usize], vals: &[usize], idx: usize, row_len: usize, size: usize) -> usize {
    (arr[idx] + if (idx + row_len) < size {
        if vals[idx + row_len] > vals[idx + row_len + 1] {
            vals[idx + row_len]
        } else {
            vals[idx + row_len + 1]
        }
    } else { 0 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        let r = [1, 2, 3];
        let s = [3; 0];
        assert_eq!(value(&r, &s, 2, 2, 3), 3);
        assert_eq!(value(&r, &s, 1, 2, 3), 2);
        assert_eq!(value(&r, &s, 0, 1, 3), 4);
    }
}
