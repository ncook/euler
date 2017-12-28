fn main() {
    let sum = (1..1000).fold(0, |acc, i| {
        acc + if (i % 3) == 0
            { i }
        else if (i % 5) == 0
            { i }
        else
            { 0 }
    });
    println!("Sum of all numbers below 1000 which are multiple of 3 or 5: {:?}", sum);
}
