// Removed: Unused `process` module import
// use std::process;
// Removed: Unused `TryInto` trait import
// use std::convert::TryInto;

const WIDTH: i32 = 80;
const YEAR: i32 = 1969;

#[derive(Debug, Clone)] // Modified: Annotate the Month struct with #[derive(Clone)]
struct Month {
    name: &'static str,
    days: i32,
    start_wday: i32,
    at: i32,
}

const MONTHS: [Month; 12] = [
    Month { name: "January", days: 31, start_wday: 0, at: 0 },
    Month { name: "February", days: 28, start_wday: 0, at: 0 },
    Month { name: "March", days: 31, start_wday: 0, at: 0 },
    Month { name: "April", days: 30, start_wday: 0, at: 0 },
    Month { name: "May", days: 31, start_wday: 0, at: 0 },
    Month { name: "June", days: 30, start_wday: 0, at: 0 },
    Month { name: "July", days: 31, start_wday: 0, at: 0 },
    Month { name: "August", days: 31, start_wday: 0, at: 0 },
    Month { name: "September", days: 30, start_wday: 0, at: 0 },
    Month { name: "October", days: 31, start_wday: 0, at: 0 },
    Month { name: "November", days: 30, start_wday: 0, at: 0 },
    Month { name: "December", days: 31, start_wday: 0, at: 0 },
];

const WEEKDAYS: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

fn space(n: i32) {
    for _ in 0..n {
        print!(" ");
    }
}

fn init_months() {
    // Modified: Use a mutable clone of the MONTHS array
    let mut months = MONTHS.clone();

    if (YEAR % 4 == 0 && YEAR % 100 != 0) || (YEAR % 400 == 0) {
        months[1].days = 29;
    }

    let mut year = YEAR - 1;
    months[0].start_wday = (year * 365 + year / 4 - year / 100 + year / 400 + 1) % 7;

    for i in 1..12 {
        months[i].start_wday = (months[i - 1].start_wday + months[i - 1].days) % 7;
    }

    // Removed: Unused variable `months`
}

fn print_row(row: i32, cols: i32, lead: i32, gap: i32) {
    let from = row * cols;
    let to = from + cols;
    space(lead);
    for c in from..to {
        let i = MONTHS[c as usize].name.len();
        space(((20 - i) / 2).try_into().unwrap()); // Modified: Convert the result of the expression to `i32`
        print!("{}", MONTHS[c as usize].name);
        space((20 - i - (20 - i) / 2 + if c == to - 1 { 0 } else { gap as usize }).try_into().unwrap()); // Modified: Convert gap to usize
    }
    println!();

    space(lead);
    for c in from..to {
        for i in 0..7 {
            print!("{}", WEEKDAYS[i]);
            if i != 6 {
                print!(" ");
            }
        }
        if c < to - 1 {
            space(gap);
        } else {
            println!();
        }
    }

    loop {
        let mut break_outer = false;
        for c in from..to {
            if MONTHS[c as usize].at < MONTHS[c as usize].days {
                break_outer = true;
                break;
            }
        }
        if !break_outer {
            break;
        }

        space(lead);
        for c in from..to {
            let mut i = 0;
            while i < MONTHS[c as usize].start_wday {
                space(3);
                i += 1;
            }
            while i < 7 && MONTHS[c as usize].at < MONTHS[c as usize].days {
                print!("{:2}", MONTHS[c as usize].at + 1);
                MONTHS[c as usize].at += 1;
                if i < 6 || c < to - 1 {
                    print!(" ");
                }
                i += 1;
            }
            while i <= 7 && c < to - 1 {
                space(3);
                i += 1;
            }
            if c < to - 1 {
                space(gap - 1);
            }
            MONTHS[c as usize].start_wday = 0;
        }
        println!();
    }
    println!();
}

fn print_year() {
    print_row(0, (WIDTH + 2) / 22, (WIDTH - (20 + 4) * ((WIDTH + 2) / 22) + 4 + 1) / 2, 4);
}

fn main() {
    init_months();
    print_year();
}