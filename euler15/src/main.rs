#![feature(inclusive_range_syntax)]
// Paths through a 20x20 grid from top left to bottom right moving only right or down.
// For a 1x1 grid there are two
// For a 2x2 grid there are six
// For a 3x3 grid there are
// For decision points that have two exit nodes there are two paths starting at that point
// For decision points that have one exit node to the right there is one path
// For decision points that have one exit node down there is one paths


// 35 15  5 1
// 20 10  4 1
// 10  6  3 1
// 4   3  2 1
// 1   1  1 0

fn main() {
    let result = path(20, 20);
    println!("Paths for 20, 20: {}", result);
}

fn path(width: u64, height: u64) -> u64 {
    let mut value = 1;
    let mut right: Vec<u64> = vec!(1; 21);
    for i in 1..=width {
        let mut down = 1;
        for j in 1..=height {
            value = match (i, j) {
                (1, 1) => 2,
                (1, _) => down + 1,
                (i, j) if i == j => right[j as usize] * 2,
                (_, j) => right[j as usize] + down
            };
            print!(" {:>12}", value);
            down = value;
            right[j as usize] = value;
        }
        println!("");
    }
    value
}

// fn path_cost(i: u64, j: u64) -> u64 {
//     match (i, j) {
//         (1, 1) => 1,
//         (1, _) => 1,
//         (i, j) if i == j => path_cost(i-1, j) * 2,
//         (i, j) => path_cost(i-1, j) + path_cost(i, j-1)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_cost() {
        assert_eq!(path(1, 1), 2);
        assert_eq!(path(1, 3), 4);
        assert_eq!(path(1, 7), 8);
        assert_eq!(path(2, 2), 6);
        assert_eq!(path(3, 3), 20);
        assert_eq!(path(3, 4), 35);
        assert_eq!(path(1, 20), 21);
        assert_eq!(path(2, 20), 231);
    }
}
