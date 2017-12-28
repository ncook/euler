// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

// Import the macro. Don't forget to add `error-chain` in your
// `Cargo.toml`!
#[macro_use]
extern crate error_chain;

// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! { }
}

use errors::*;

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    let mut names = Vec::new();
    const FNAME: &'static str = "../names.txt";

    let f = File::open(FNAME)
        .chain_err(|| "Failed to open name file")?;

    let reader = BufReader::new(f);

    for s in reader.split(b',') {
        match s {
            Ok(s) => {
                names.push(s[1..s.len()].to_vec());
            },
            Err(_) => panic!("Error reading file")
        }
    };
    // println!("There are {} amicable numbers less than {} and their sum is {}",
    // amicable.len(), LIMIT, amicable.into_iter().sum::<usize>());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_cost() {
        // assert_eq!(proper_divisors_sum(42), 54); //1, 2, 3, 6, 7, 14, 21 sum=54
    }
}
