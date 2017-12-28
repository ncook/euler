struct Collatz {
    lengths: [u64; 1_000_000],
}

impl Collatz {
    fn has_len(&self, n: u64) -> bool {
        if (n as usize) < self.lengths.len() { self.lengths[n as usize] != 0 } else { false }
    }

    fn set_len(&mut self, i: u64, l: u64) {
        self.lengths[i as usize] = l;
    }

    fn len(&self, i: u64) -> u64 {
        self.lengths[i as usize]
    }

    fn length(&mut self, start: u64) -> u64 {
        let mut len = 1;
        let mut i = start;
        while (i != 1) && !self.has_len(i) {
            match i % 2 {
                0 => i = i / 2,
                _ => i = i * 3 + 1,
            }
            len += 1;
            println!("i, len: {} {}", i, len);
        }
        if self.has_len(i) {
            len += self.len(i) - 1;
        }
        self.set_len(start, len);
        len
    }
}

fn main() {
    let mut chain_len = 0;
    let mut target = 1;
    let mut collatz = Collatz { lengths: [0; 1_000_000] };
    for u in 1..1_000_000 {
        let c = collatz.length(u);
        println!("u, c: {} {}", u, c);
        if c > chain_len {
            chain_len = c;
            target = u;
        }
    }
    println!("Longest Collatz sequence of {} starts with {}", chain_len, target);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row() {
        let mut collatz = Collatz { lengths : [0; 1_000_000] };
        assert_eq!(collatz.length(1), 1);
        assert_eq!(collatz.length(2), 2);
        assert_eq!(collatz.length(3), 8);
        assert_eq!(collatz.length(4), 3);
        assert_eq!(collatz.length(5), 6);
        assert_eq!(collatz.length(13), 10);
    }
}
