use std::process;

const WD: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

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

fn init_months() {
    unsafe {
        if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
            MONTHS[1].days = 29;
        }

        // Modified: Renamed local variable to avoid shadowing the static variable
        let mut prev_year = year - 1;
        MONTHS[0].start_wday = (prev_year * 365 + prev_year / 4 - prev_year / 100 + prev_year / 400 + 1) % 7;

        for i in 1..12 {
            MONTHS[i].start_wday = (MONTHS[i - 1].start_wday + MONTHS[i - 1].days) % 7;
        }

        cols = (width + 2) / 22;
        while 12 % cols != 0 {
            cols -= 1;
        }
        gap = if cols - 1 != 0 { (width - 20 * cols) / (cols - 1) } else { 0 };
        if gap > 4 {
            gap = 4;
        }
        lead = (width - (20 + gap) * cols + gap + 1) / 2;
        year += 1;
    }
}

fn print_row(row: i32) {
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
                print!("{}{}", WD[i as usize], if i == 6 { "" } else { " " });
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
                if MONTHS[c as usize].at < MONTHS[c as usize].days {
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

fn print_year() {
    unsafe {
        // Modified: Wrapped the use of mutable static variable `year` in an unsafe block
        let buf = format!("{}", year);
        // Modified: Wrapped the use of mutable static variable `width` in an unsafe block
        space((width - buf.len() as i32) / 2);
        println!("{}\n", buf);
        let mut row = 0;
        // Modified: Wrapped the use of mutable static variable `cols` in an unsafe block
        while row * cols < 12 {
            print_row(row);
            row += 1;
        }
    }
}

static mut width: i32 = 80;
static mut year: i32 = 1969;
static mut cols: i32 = 0;
static mut lead: i32 = 0;
static mut gap: i32 = 0;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut year_set = false;

    for i in 1..args.len() {
        if args[i] == "-w" {
            if i + 1 < args.len() {
                unsafe {
                    width = args[i + 1].parse::<i32>().unwrap_or(80);
                }
                if unsafe { width } < 20 {
                    eprintln!("bad args\nUsage: {} year [-w width (>= 20)]", args[0]);
                    process::exit(1);
                }
                continue;
            } else {
                eprintln!("bad args\nUsage: {} year [-w width (>= 20)]", args[0]);
                process::exit(1);
            }
        } else if !year_set {
            unsafe {
                year = args[i].parse::<i32>().unwrap_or(1969);
            }
            year_set = true;
        } else {
            eprintln!("bad args\nUsage: {} year [-w width (>= 20)]", args[0]);
            process::exit(1);
        }
    }

    init_months();
    print_year();
}