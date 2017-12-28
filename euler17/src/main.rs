#![feature(inclusive_range_syntax)]
#![feature(dotdoteq_in_patterns)]
// Couning numbers by letter counts using British usage
// Numbers from one to nineteen have the counts from the letters in their names.
// Two digit numbers have the former plus the word for the number of tens with the exception of
// the teens.
// From 100 up, we have a digit plus the word hundred. If the number % 100 is not zero then
// "and" is added and the count for the remainder which matches the number less than 100.
fn main() {
    let mut idx = 1;
    let mut sum = 0;
    let sum = (1..1000).map(hundreds).inspect(|a| { sum += a; println!("{} {} {}", idx, a, sum); idx += 1;} ).sum::<u32>() + 11;
    println!("The sum of all the letters in all the numbers to 1000 is: {}", sum);
}

fn hundreds(num: u32) -> u32 {
    let n = num / 100;
    let rem = num % 100;
    if n == 0 { under_a_hundred(rem) } else {
        if rem == 0 { digit(n) + 7 } else { digit(n) + 10 + under_a_hundred(rem) }
    }
}

fn under_a_hundred(num: u32) -> u32 {
    match num {
        1..=19 => under_twenty(num),
        _ => tens(num) + digit(num % 10),
    }
}

fn under_twenty(num: u32) -> u32 {
    match num {
        19 => 8, // nineteen
        18 => 8, // eighteen
        17 => 9, // seventeen
        16 => 7, // sixteen
        15 => 7, // fifteen
        14 => 8, // fourteen
        13 => 8, // thirteen
        12 => 6, // twelve
        11 => 6, // eleven
        10 => 3, // ten
        x => digit(x)
    }
}

fn tens(num: u32) -> u32 {
    match num / 10 {
        9 => 6, // ninety
        8 => 6, // eighty
        7 => 7, // seventy
        6 => 5, // sixty
        5 => 5, // fifty
        4 => 5, // forty
        3 => 6, // thirty
        2 => 6, // twenty
        _ => 0, //
    }
}

fn digit(num: u32) -> u32 {
    match num {
        9 => 4, // nine
        8 => 5, // eight
        7 => 5, // seven
        6 => 3, // six
        5 => 4, // five
        4 => 4, // four
        3 => 5, // three
        2 => 3, // two
        1 => 3, // one
        _ => 0, //
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit() {
        assert_eq!(digit(9), 4);
        assert_eq!(digit(19), 0);
        assert_eq!(tens(99), 6);
        assert_eq!(tens(19), 0);
        assert_eq!(under_twenty(19), 8);
        assert_eq!(under_twenty(5), 4);
        assert_eq!(under_twenty(20), 0);
        assert_eq!(under_a_hundred(99), 10);
        assert_eq!(under_a_hundred(19), 8);
        assert_eq!(hundreds(999), 24);
        assert_eq!(hundreds(19), 8);
        assert_eq!(hundreds(120), 19);
        assert_eq!(hundreds(130), 19);
        assert_eq!(hundreds(150), 18);
        assert_eq!(hundreds(160), 18);
        assert_eq!(hundreds(199), 23);
        assert_eq!(hundreds(200), 10);
        assert_eq!(hundreds(280), 19);
        assert_eq!(hundreds(290), 19);
        assert_eq!(hundreds(800), 12);
        assert_eq!(hundreds(880), 21);
        assert_eq!(hundreds(888), 26);
        assert_eq!(hundreds(900), 11);
        assert_eq!(hundreds(999), 24);
        assert_eq!((1..100).map(hundreds).sum::<u32>() + 99*13 + 10, (100..200).map(hundreds).sum::<u32>());
        assert_eq!((100..200).map(hundreds).sum::<u32>(), (200..300).map(hundreds).sum::<u32>());
        assert_eq!((200..300).map(hundreds).sum::<u32>(), (600..700).map(hundreds).sum::<u32>());
        assert_eq!((400..500).map(hundreds).sum::<u32>(), (500..600).map(hundreds).sum::<u32>());
        assert_eq!((400..500).map(hundreds).sum::<u32>(), (900..1000).map(hundreds).sum::<u32>());
        assert_eq!((300..400).map(hundreds).sum::<u32>(), (700..800).map(hundreds).sum::<u32>());
        assert_eq!((800..900).map(hundreds).sum::<u32>(), (700..800).map(hundreds).sum::<u32>());
        assert_eq!((100..200).map(hundreds).sum::<u32>(), ((700..800).map(hundreds).sum::<u32>() - 200));
    }
}
