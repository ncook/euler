// Number spiral diagonals - problem 28

// Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5 spiral is formed as follows:
//
// 21 22 23 24 25
// 20  7  8  9 10
// 19  6  1  2 11
// 18  5  4  3 12
// 17 16 15 14 13
//
// It can be verified that the sum of the numbers on the diagonals is 101.
//
// What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?

#![feature(iterator_step_by)]

fn main() {
    // Only odd numbered sized spiral exist
    // A 1X spiral sums to 1.
    // A 3X spiral adds 4 numbers, the third, fifth, seventh and nineth. Those are 3, 5, 7, 9.
    // A 5X spiral adds 4 numbers, at intervals of four. 13, 17, 21, 25.
    // A 7X spiral adds 31, 37, 43, 49.
    // A 1001X spiral adds 999,001, 1,000,001, 1,001,001, 1,002,001
    let mut sum: i64 = 1;
    let mut cur_value = 1;
    for i in (3..1002).step_by(2) {
        cur_value += (i - 1) * 4;
        sum += cur_value * 4 - (i - 1) * 6;
        println!("spiral {}, sum: {}", i, sum);
    }
    println!("Diagonal sum for 1001 spiral is: {}", sum);
}
