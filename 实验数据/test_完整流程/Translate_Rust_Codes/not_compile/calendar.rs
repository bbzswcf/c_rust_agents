#[macro_use]
extern crate lazy_static; // Modified: Added `#[macro_use]` to import the `lazy_static` macro correctly
use std::process;
use std::io::Write; // Added: Import for `writeln!` macro

const WD: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

struct Month {
    name: &'static str,
    days: i32,
    start_wday: i32,
    at: i32,
}

lazy_static! {
    static ref MONTHS: std::sync::Mutex<[Month; 12]> = std::sync::Mutex::new([
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
    ]);
}

fn space(n: i32) {
    for _ in 0..n {
        print!(" ");
    }
}

fn init_months(year: i32, width: i32) {
    let mut months = MONTHS.lock().unwrap(); // Modified: Ensured `MONTHS` is in scope
    if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
        months[1].days = 29;
    }

    let mut prev_year = year - 1; // Avoid shadowing the `year` variable
    months[0].start_wday = (prev_year * 365 + prev_year / 4 - prev_year / 100 + prev_year / 400 + 1) % 7;

    for i in 1..12 {
        months[i].start_wday = (months[i - 1].start_wday + months[i - 1].days) % 7;
    }

    let cols = (width + 2) / 22; // Removed `mut` as it is not reassigned
    while 12 % cols != 0 {
        cols -= 1;
    }
    let gap = if cols - 1 != 0 { (width - 20 * cols) / (cols - 1) } else { 0 }; // Removed `mut` as it is not reassigned
    if gap > 4 {
        gap = 4;
    }
    let lead = (width - (20 + gap) * cols + gap + 1) / 2; // Removed `mut` as it is not reassigned
    prev_year += 1;
}

fn print_row(row: i32, cols: i32, lead: i32, gap: i32) {
    let months = MONTHS.lock().unwrap(); // Modified: Ensured `MONTHS` is in scope
    let from = row * cols;
    let to = from + cols;
    space(lead);
    for c in from..to {
        let i = months[c as usize].name.len(); // Ensure `c` is cast to `usize`
        space((20 - i) / 2);
        print!("{}", months[c as usize].name);
        space(20 - i - (20 - i) / 2 + if c == to - 1 { 0 } else { gap });
    }
    println!();

    space(lead);
    for c in from..to {
        for i in 0..7 {
            print!("{}", WD[i as usize]); // Ensure `i` is cast to `usize`
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
        let mut break_outer = true;
        for c in from..to {
            if months[c as usize].at < months[c as usize].days { // Ensure `c` is cast to `usize`
                break_outer = false;
                break;
            }
        }
        if break_outer {
            break;
        }

        space(lead);
        for c in from..to {
            let mut i = 0;
            while i < months[c as usize].start_wday { // Ensure `c` is cast to `usize`
                space(3);
                i += 1;
            }
            while i < 7 && months[c as usize].at < months[c as usize].days { // Ensure `c` is cast to `usize`
                print!("{:2}", months[c as usize].at + 1);
                months[c as usize].at += 1;
                if i < 6 || c < to - 1 {
                    print!(" ");
                }
                i += 1;
            }
            while i <= 7 && c < to - 1 { // Ensure `c` is cast to `usize`
                space(3);
                i += 1;
            }
            if c < to - 1 {
                space(gap - 1);
            }
            months[c as usize].start_wday = 0; // Ensure `c` is cast to `usize`
        }
        println!();
    }
    println!();
}

fn print_year(year: i32, width: i32, cols: i32, lead: i32, gap: i32) {
    let buf = format!("{}", year);
    space((width - buf.len() as i32) / 2);
    println!("{}", buf);
    println!();
    let mut row = 0;
    while row * cols < 12 {
        print_row(row, cols, lead, gap);
        row += 1;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut year_set = false;
    let mut width = 80;
    let mut year = 1969;
    let cols = 0; // Removed `mut` as it is not reassigned
    let lead = 0; // Removed `mut` as it is not reassigned
    let gap = 0; // Removed `mut` as it is not reassigned

    let mut i = 1;
    while i < args.len() {
        if args[i] == "-w" {
            i += 1;
            if i == args.len() || args[i].parse::<i32>().unwrap_or(0) < 20 {
                writeln!(std::io::stderr(), "bad args\nUsage: {} year [-w width (>= 20)]\n", args[0]).unwrap(); // Modified: Use `writeln!` macro
                process::exit(1);
            }
            width = args[i].parse::<i32>().unwrap();
        } else if !year_set {
            if let Ok(y) = args[i].parse::<i32>() {
                if y > 0 {
                    year = y;
                    year_set = true;
                } else {
                    year = 1969;
                }
            } else {
                year = 1969;
            }
        } else {
            writeln!(std::io::stderr(), "bad args\nUsage: {} year [-w width (>= 20)]\n", args[0]).unwrap(); // Modified: Use `writeln!` macro
            process::exit(1);
        }
        i += 1;
    }

    init_months(year, width);
    print_year(year, width, cols, lead, gap);
}