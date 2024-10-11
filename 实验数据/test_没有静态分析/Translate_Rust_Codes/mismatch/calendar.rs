use std::convert::TryInto;
use std::process;

const WIDTH: i32 = 80;
const YEAR: i32 = 1969;

fn space(n: i32) {
    for _ in 0..n {
        print!(" ");
    }
}

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

fn init_months(year: i32, months: &mut [Month; 12]) {
    if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
        months[1].days = 29;
    }

    let year = year - 1;
    months[0].start_wday = (year * 365 + year / 4 - year / 100 + year / 400 + (if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 1 } else { 0 })) % 7;

    for i in 1..12 {
        months[i].start_wday = (months[i - 1].start_wday + months[i - 1].days) % 7;
    }

    let cols = (WIDTH + 1) / 21;
    let gap = if cols > 1 { (WIDTH - 20 * cols + 1) / (cols - 1) } else { 0 };
    let lead = (WIDTH - (20 + gap) * cols + gap + 1) / 2;

    for month in months.iter_mut() {
        month.at = 0;
    }
}

fn print_row(row: i32, months: &mut [Month; 12], cols: i32, lead: i32, gap: i32) {
    let from = row * cols;
    let to = from + cols;

    space(lead);
    for c in from..to {
        let i = months[c as usize].name.len();
        space(((20 - i as i32) / 2).try_into().unwrap());
        print!("{}", months[c as usize].name);
        space((20 - i as i32 - (20 - i as i32) / 2 + if c as i32 == to - 1 { 0 } else { gap - 1 }).try_into().unwrap());
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
            while i < 7 && c < to - 1 {
                space(3);
                i += 1;
            }
            if c < to - 1 {
                space(gap - 1);
            }
        }
        println!();
    }
    println!();
}

fn print_year(year: i32, months: &mut [Month; 12], cols: i32, lead: i32, gap: i32) {
    let year_str = format!("{}", year);
    space((WIDTH - year_str.len() as i32) / 2);
    println!("{}", year_str);
    println!();

    let mut row = 0;
    while row * cols < 12 {
        print_row(row, months, cols, lead, gap);
        row += 1;
    }
}

fn main() {
    let mut args = std::env::args();
    let mut year = YEAR;
    let mut width = WIDTH;
    let mut year_set = false;

    args.next();

    while let Some(arg) = args.next() {
        if arg == "-w" {
            if let Some(width_arg) = args.next() {
                if let Ok(parsed_width) = width_arg.parse::<i32>() {
                    if parsed_width >= 20 {
                        width = parsed_width;
                    } else {
                        eprint!("bad args\nUsage: {} year [-w width (>= 20)]\n", args.next().unwrap());
                        process::exit(1);
                    }
                } else {
                    eprint!("bad args\nUsage: {} year [-w width (>= 20)]\n", args.next().unwrap());
                    process::exit(1);
                }
            } else {
                eprint!("bad args\nUsage: {} year [-w width (>= 20)]\n", args.next().unwrap());
                process::exit(1);
            }
        } else if !year_set {
            if let Ok(parsed_year) = arg.parse::<i32>() {
                if parsed_year > 0 {
                    year = parsed_year;
                    year_set = true;
                } else {
                    year = YEAR;
                }
            } else {
                year = YEAR;
            }
        } else {
            eprint!("bad args\nUsage: {} year [-w width (>= 20)]\n", args.next().unwrap());
            process::exit(1);
        }
    }

    let mut months = MONTHS;
    init_months(year, &mut months);
    print_year(year, &mut months, (width + 1) / 21, (width - (20 + ((width - 20) / 21)) * 21 + ((width - 20) / 21) + 1) / 2, if (width - 20) / 21 > 1 { (width - 20) / 21 } else { 0 });
}