use std::env;
use std::io::{self, Write};

const WIDTH: i32 = 80;
const YEAR: i32 = 1969;

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

fn init_months(year: i32, months: &mut [Month; 12]) {
    if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
        months[1].days = 29;
    }

    let mut year = year - 1;
    months[0].start_wday = (year * 365 + year / 4 - year / 100 + year / 400 + 1) % 7;

    for i in 1..12 {
        months[i].start_wday = (months[i - 1].start_wday + months[i - 1].days) % 7;
    }

    let cols = (WIDTH + 2) / 22;
    let mut gap = if cols > 1 { (WIDTH - 20 * cols) / (cols - 1) } else { 0 };
    if gap > 4 {
        gap = 4;
    }
    let lead = (WIDTH - (20 + gap) * cols + gap + 1) / 2;
    year += 1;
}

fn print_row(row: i32, cols: i32, gap: i32, lead: i32, months: &mut [Month; 12]) {
    let from = row * cols;
    let to = from + cols;
    space(lead);
    for c in from..to {
        let i = months[c as usize].name.len();
        space((20 - i as i32) / 2);
        print!("{}", months[c as usize].name);
        space(20 - i as i32 - (20 - i as i32) / 2 + if c == to - 1 { 0 } else { gap });
    }
    println!();

    space(lead);
    for c in from..to {
        for (i, wday) in WEEKDAYS.iter().enumerate() {
            print!("{}", wday);
            if i < 6 {
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
        let mut all_done = true;
        for c in from..to {
            if months[c as usize].at < months[c as usize].days {
                all_done = false;
                break;
            }
        }
        if all_done {
            break;
        }

        space(lead);
        for c in from..to {
            let mut i = 0;
            while i < months[c as usize].start_wday {
                space(3);
                i += 1;
            }
            while i < 7 && months[c as usize].at < months[c as usize].days {
                print!("{:2}", months[c as usize].at + 1);
                months[c as usize].at += 1;
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
            months[c as usize].start_wday = 0;
        }
        println!();
    }
    println!();
}

fn print_year(year: i32, width: i32, months: &mut [Month; 12]) {
    let cols = (width + 2) / 22;
    let gap = if cols > 1 { (width - 20 * cols) / (cols - 1) } else { 0 };
    let lead = (width - (20 + gap) * cols + gap + 1) / 2;

    let year_str = year.to_string();
    space((width - year_str.len() as i32) / 2);
    println!("{}", year_str);
    println!();

    for row in 0..(12 / cols) {
        print_row(row, cols, gap, lead, months);
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut year = YEAR;
    let mut width = WIDTH;
    let mut year_set = false;

    let mut i = 1;
    while i < args.len() {
        if args[i] == "-w" {
            i += 1;
            if i == args.len() || args[i].parse::<i32>().unwrap_or(0) < 20 {
                eprintln!("bad args\nUsage: {} year [-w width (>= 20)]", args[0]);
                std::process::exit(1);
            }
            width = args[i].parse().unwrap();
        } else if !year_set {
            if let Ok(y) = args[i].parse::<i32>() {
                if y > 0 {
                    year = y;
                    year_set = true;
                }
            }
        } else {
            eprintln!("bad args\nUsage: {} year [-w width (>= 20)]", args[0]);
            std::process::exit(1);
        }
        i += 1;
    }

    let mut months = MONTHS;
    init_months(year, &mut months);
    print_year(year, width, &mut months);

    Ok(())
}