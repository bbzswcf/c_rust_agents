use std::io::{self, Write};

const WIDTH: i32 = 80;
const YEAR: i32 = 1969;

static WD_DAYS: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

struct Month {
    name: &'static str,
    days: i32,
    start_wday: i32,
    at: i32,
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

fn space(n: i32) {
    for _ in 0..n {
        print!(" ");
    }
}

fn init_months(width: i32, year: i32) {
    unsafe {
        if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
            MONTHS[1].days = 29;
        }

        let mut year = year - 1;
        MONTHS[0].start_wday = (year * 365 + year / 4 - year / 100 + year / 400 + 1) % 7;

        for i in 1..12 {
            MONTHS[i].start_wday = (MONTHS[i - 1].start_wday + MONTHS[i - 1].days) % 7;
        }

        let cols = (width + 2) / 22;
        let mut gap = if cols > 1 { (width - 20 * cols) / (cols - 1) } else { 0 };
        if gap > 4 {
            gap = 4;
        }
        let lead = (width - (20 + gap) * cols + gap + 1) / 2;
    }
}

fn print_row(row: i32, cols: i32, lead: i32, gap: i32) {
    unsafe {
        let from = row * cols;
        let to = from + cols;
        space(lead);
        for c in from..to {
            let i = MONTHS[c as usize].name.len();
            space((20 - i as i32) / 2);
            print!("{}", MONTHS[c as usize].name);
            space(20 - i as i32 - (20 - i as i32) / 2 + if c == to - 1 { 0 } else { gap });
        }
        println!();

        space(lead);
        for c in from..to {
            for i in 0..7 {
                print!("{}", WD_DAYS[i as usize]);
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
            let mut c = from;
            while c < to {
                if MONTHS[c as usize].at < MONTHS[c as usize].days {
                    break;
                }
                c += 1;
            }
            if c == to {
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
}

fn print_year(width: i32, year: i32) {
    let buf = format!("{}", year);
    space((width - buf.len() as i32) / 2);
    println!("{}\n", buf);
    let mut row = 0;
    let cols = (width + 2) / 22;
    let mut gap = if cols > 1 { (width - 20 * cols) / (cols - 1) } else { 0 };
    if gap > 4 {
        gap = 4;
    }
    let lead = (width - (20 + gap) * cols + gap + 1) / 2;
    while row * cols < 12 {
        print_row(row, cols, lead, gap);
        row += 1;
    }
}

fn main() {
    let mut args = std::env::args();
    let mut year_set = false;
    let mut width = WIDTH;
    let mut year = YEAR;

    args.next(); // Skip the program name
    while let Some(arg) = args.next() {
        if arg == "-w" {
            if let Some(width_arg) = args.next() {
                if let Ok(w) = width_arg.parse::<i32>() {
                    if w >= 20 {
                        width = w;
                    } else {
                        eprintln!("bad args\nUsage: {} year [-w width (>= 20)]", std::env::args().next().unwrap());
                        std::process::exit(1);
                    }
                } else {
                    eprintln!("bad args\nUsage: {} year [-w width (>= 20)]", std::env::args().next().unwrap());
                    std::process::exit(1);
                }
            } else {
                eprintln!("bad args\nUsage: {} year [-w width (>= 20)]", std::env::args().next().unwrap());
                std::process::exit(1);
            }
        } else if !year_set {
            if let Ok(y) = arg.parse::<i32>() {
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
            eprintln!("bad args\nUsage: {} year [-w width (>= 20)]", std::env::args().next().unwrap());
            std::process::exit(1);
        }
    }

    init_months(width, year);
    print_year(width, year);
}