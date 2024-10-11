use std::env;
use std::io::{self, Write};

const WIDTH: usize = 80;
const YEAR: usize = 1969;

const WEEKDAYS: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

struct Month {
    name: &'static str,
    days: usize,
    start_wday: usize,
    at: usize,
}

static mut MONTHS: [Month; 12] = [
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

fn space(n: usize) {
    for _ in 0..n {
        print!(" ");
    }
}

fn init_months(year: usize) {
    unsafe {
        if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
            MONTHS[1].days = 29;
        }

        let year = year - 1;
        MONTHS[0].start_wday = (year * 365 + year / 4 - year / 100 + year / 400 + 1) % 7;

        for i in 1..12 {
            MONTHS[i].start_wday = (MONTHS[i - 1].start_wday + MONTHS[i - 1].days) % 7;
        }

        let cols = (WIDTH + 2) / 22;
        let cols = if 12 % cols == 0 { cols } else { cols - 1 };
        let gap = if cols > 1 { (WIDTH - 20 * cols) / (cols - 1) } else { 0 };
        let gap = if gap > 4 { 4 } else { gap };
        let lead = (WIDTH - (20 + gap) * cols + gap + 1) / 2;

        for month in MONTHS.iter_mut() {
            month.at = 0;
        }
    }
}

fn print_row(row: usize, cols: usize, gap: usize, lead: usize) {
    unsafe {
        let from = row * cols;
        let to = from + cols;

        space(lead);
        for c in from..to {
            let i = MONTHS[c].name.len();
            space((20 - i) / 2);
            print!("{}", MONTHS[c].name);
            space(20 - i - (20 - i) / 2 + if c == to - 1 { 0 } else { gap });
        }
        println!();

        space(lead);
        for c in from..to {
            for (i, wday) in WEEKDAYS.iter().enumerate() {
                print!("{}{}", wday, if i == 6 { "" } else { " " });
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
                if MONTHS[c].at < MONTHS[c].days {
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
                while i < MONTHS[c].start_wday {
                    space(3);
                    i += 1;
                }
                while i < 7 && MONTHS[c].at < MONTHS[c].days {
                    print!("{:2}", MONTHS[c].at + 1);
                    MONTHS[c].at += 1;
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
                MONTHS[c].start_wday = 0;
            }
            println!();
        }
        println!();
    }
}

fn print_year(year: usize, cols: usize, gap: usize, lead: usize) {
    let year_str = year.to_string();
    space((WIDTH - year_str.len()) / 2);
    println!("{}\n", year_str);

    for row in 0..(12 / cols) {
        print_row(row, cols, gap, lead);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut width = WIDTH;
    let mut year = YEAR;
    let mut year_set = false;

    let mut i = 1;
    while i < args.len() {
        if args[i] == "-w" {
            i += 1;
            if i == args.len() || args[i].parse::<usize>().unwrap_or(0) < 20 {
                eprintln!("bad args\nUsage: {} year [-w width (>= 20)]", args[0]);
                std::process::exit(1);
            }
            width = args[i].parse().unwrap();
        } else if !year_set {
            if let Ok(y) = args[i].parse::<usize>() {
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

    init_months(year);
    let cols = (width + 2) / 22;
    let cols = if 12 % cols == 0 { cols } else { cols - 1 };
    let gap = if cols > 1 { (width - 20 * cols) / (cols - 1) } else { 0 };
    let gap = if gap > 4 { 4 } else { gap };
    let lead = (width - (20 + gap) * cols + gap + 1) / 2;

    print_year(year, cols, gap, lead);
}