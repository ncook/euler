#![feature(inclusive_range_pattern)]
#![feature(exclusive_range_pattern)]
#![feature(dotdoteq_in_patterns)]

struct FirstOfMonth {
    first_day: u32,
    month: u32,
    year: u32,
}

impl FirstOfMonth {
    fn new(first_day: u32, year: u32, month: u32) -> FirstOfMonth {
        FirstOfMonth {
            first_day: first_day,
            year: year,
            month: month,
        }
    }
}

impl Iterator for FirstOfMonth {
    type Item = Self;
    fn next(&mut self) -> Option<Self::Item> {
        self.first_day = (month_length(self.month, self.year) + self.first_day) % 7;
        if self.month == 11 {
            self.month = 0;
            self.year += 1;
        } else {
            self.month += 1;
        }
        Some(FirstOfMonth::new(self.first_day, self.year, self.month))
    }
}

fn main() {
    // First we need to compute the day of week of 1 Jan 1901.
    // We are given that 1 Jan 1900 was a Monday.
    // 1900 was not a leap year so it had 365 days.
    // 365 days after that Monday was Wednesday 1 Jan 1901.
    // Monday is considered day 0. Wednesday is day 2.
    // let mut first_day_of_year = 2;
    // let mut monday_count = 0;
    // let mut year_length = 0;
    // for year in 1901..2000 {
    //     year_length = (0..12).map(|m| month_length(m, year)).sum();
    //     for month in 0..12 {
    //         //
    //     }
    //     first_day_of_year = (first_day_of_year + year_length) % 7;
    // }

    let fom = FirstOfMonth::new(0, 1900, 0);
    let monday_count = fom
        .take_while(|x| x.year < 2001)
        .filter(|x| x.year >= 1901 )
        .filter(|x| x.first_day == 6 )
        .inspect(|ref x| println!("{} {} {}", x.year, x.month, x.first_day))
        .count();
    println!("The number of Mondays in the 20 century is: {}", monday_count);
}

fn month_length(month: u32, year: u32) -> u32 {
    match month {
        0 => 31,
        1 => {
            match year {
                2000 => 29,
                1901..2000 if year % 4 == 0 => 29,
                1900..2000 => 28,
                _ => panic!("Year not in the twentieth century: {}", year),
            }
        },
        2 => 31,
        3 => 30,
        4 => 31,
        5 => 30,
        6 => 31,
        7 => 31,
        8 => 30,
        9 => 31,
        10 => 30,
        11 => 31,
        _ => panic!("Month our of calendar range! {}", month),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_month_length() {
        assert_eq!(month_length(1, 2000), 29);
        assert_eq!(month_length(1, 1900), 28);
    }
    #[test]
    fn test_iterator() {
        let mut it = FirstOfMonth::new(0, 1900, 0);
        assert_eq!(it.next().map(|i| i.first_day ), Some(3) );
        assert_eq!(it.next().map(|i| i.first_day ), Some(3) );
        assert_eq!(it.next().map(|i| i.first_day ), Some(6) );
    }
}
