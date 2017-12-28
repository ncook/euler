use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let mut names = Vec::new();

    let f = File::open("../names.txt").expect("Failed to open ../names.txt");
    let reader = BufReader::new(f);

    for s in reader.split(b',') {
        match s {
            Ok(s) => {
                names.push(s[1..s.len()].to_vec());
            },
            Err(_) => panic!("Error reading file")
        }
    };
    for s in reader.split(b',') {
        s.map(|qs| qs[1..qs.len()].to_vec());
    };
    names.push(reader.split(b',').map(|qs| qs[1..qs.len()].to_vec()));
    // println!("There are {} amicable numbers less than {} and their sum is {}",
    // amicable.len(), LIMIT, amicable.into_iter().sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_cost() {
        // assert_eq!(proper_divisors_sum(42), 54); //1, 2, 3, 6, 7, 14, 21 sum=54
    }
}
