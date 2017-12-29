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

    let mut names: Vec<Vec<u8>> = Vec::new();
    const FNAME: &'static str = "p022_names.txt";

    let f = File::open(FNAME)
        .chain_err(|| "Failed to open name file")?;

    let reader = BufReader::new(f);

    for s in reader.split(b',') {
        match s {
            Ok(s) => names.push(s[1..(s.len()-1)].to_vec()),
            Err(_) => panic!("Error reading file"),
        }
    };

    names.as_mut_slice().sort();

    let total: usize = name_position(names).iter().sum();
    println!("The total name position product sum is {}", total);

    Ok(())
}

fn name_value(bytes: &[u8]) -> usize {
    bytes.iter().map(|&b| (b + 1 - b'A') as usize).sum()
}

fn name_position(names: Vec<Vec<u8>>) -> Vec<usize> {
    names.iter().enumerate().map(|(i, n)| name_value(n) * (i + 1) ).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_value() {
        assert_eq!(name_value(b"A"), 1);
        assert_eq!(name_value(b"COLIN"), 53);
        assert_eq!(name_value(b"ALFA"), 20);
        assert_eq!(name_value(b"BETA"), 28);
    }

    #[test]
    fn test_name_position_value() {
        let names: Vec<Vec<u8>> = vec![b"ALFA".to_vec(), b"BETA".to_vec()];
        let name_positions = name_position(names);
        assert_eq!(name_positions[0], 20);
        assert_eq!(name_positions[1], 56);
    }
}
