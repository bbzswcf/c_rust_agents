use std::process;

const WIDTH: i32 = 80;

static mut COLS: i32 = 0;
static mut LEAD: i32 = 0;
static mut GAP: i32 = 0;

const WDAYS: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

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

unsafe fn init_months(year: i32) {
    let mut i: i32; // Modified: Added explicit type annotation for `i`

    if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
        MONTHS[1].days = 29;
    }

    // Modified: Removed unused variable `year_temp`
    MONTHS[0].start_wday = (year - 1) * 365 + (year - 1) / 4 - (year - 1) / 100 + (year - 1) / 400 + 1 % 7;

    for i in 1..12 {
        MONTHS[i].start_wday = (MONTHS[i - 1].start_wday + MONTHS[i - 1].days) % 7;
    }

    COLS = (WIDTH + 2) / 22;
    while 12 % COLS != 0 {
        COLS -= 1;
    }
    GAP = if COLS - 1 != 0 { (WIDTH - 20 * COLS) / (COLS - 1) } else { 0 };
    if GAP > 4 {
        GAP = 4;
    }
    LEAD = (WIDTH - (20 + GAP) * COLS + GAP + 1) / 2;
}

unsafe fn print_row(row: i32) {
    let mut c: i32 = 0; // Modified: Initialized `c` before use
    let mut i: i32 = 0; // Modified: Initialized `i` before use
    let from = row * COLS;
    let to = from + COLS;
    space(LEAD);
    for c in from..to {
        i = MONTHS[c as usize].name.len() as i32;
        space((20 - i) / 2);
        print!("{}", MONTHS[c as usize].name);
        space(20 - i - (20 - i) / 2 + if c == to - 1 { 0 } else { GAP });
    }
    println!();

    space(LEAD);
    for c in from..to {
        for i in 0..7 {
            print!("{}{}", WDAYS[i as usize], if i == 6 { "" } else { " " });
        }
        if c < to - 1 {
            space(GAP);
        } else {
            println!();
        }
    }

    loop {
        for c in from..to {
            if MONTHS[c as usize].at < MONTHS[c as usize].days {
                break;
            }
        }
        if c == to {
            break;
        }

        space(LEAD);
        for c in from..to {
            for i in 0..MONTHS[c as usize].start_wday {
                space(3);
            }
            while i < 7 && MONTHS[c as usize].at < MONTHS[c as usize].days {
                i += 1;
                MONTHS[c as usize].at += 1;
                print!("{:2}", MONTHS[c as usize].at);
                if i < 7 || c < to - 1 {
                    print!(" ");
                }
            }
            while i <= 7 && c < to - 1 {
                i += 1;
                space(3);
            }
            if c < to - 1 {
                space(GAP - 1);
            }
            MONTHS[c as usize].start_wday = 0;
        }
        println!();
    }
    println!();
}

unsafe fn print_year(year: i32) {
    // Modified: Removed unused variable `row`
    let buf = format!("{}", year);
    space((WIDTH - buf.len() as i32) / 2);
    println!("{}", buf);
    for row in 0.. {
        if row * COLS >= 12 {
            break;
        }
        print_row(row);
    }
}

fn main() {
    let mut args = std::env::args();
    let mut year_set = false;
    let mut width = WIDTH;
    let mut year = 1969; // Modified: Moved variable declaration inside the function

    args.next(); // Skip the program name

    while let Some(arg) = args.next() {
        if arg.eq("-w") {
            if let Some(width_arg) = args.next() {
                if let Ok(parsed_width) = width_arg.parse::<i32>() {
                    if parsed_width >= 20 {
                        width = parsed_width; // Modified: Ensure the value assigned to `width` is used later in the code
                    } else {
                        eprint!("bad args\nUsage: {} year [-w width (>= 20)]\n", std::env::args().next().unwrap());
                        process::exit(1);
                    }
                } else {
                    eprint!("bad args\nUsage: {} year [-w width (>= 20)]\n", std::env::args().next().unwrap());
                    process::exit(1);
                }
            } else {
                eprint!("bad args\nUsage: {} year [-w width (>= 20)]\n", std::env::args().next().unwrap());
                process::exit(1);
            }
        } else if !year_set {
            if let Ok(parsed_year) = arg.parse::<i32>() {
                if parsed_year > 0 {
                    year = parsed_year;
                    year_set = true;
                } else {
                    year = 1969;
                }
            } else {
                year = 1969;
            }
        } else {
            eprint!("bad args\nUsage: {} year [-w width (>= 20)]\n", std::env::args().next().unwrap());
            process::exit(1);
        }
    }

    unsafe {
        init_months(year);
        print_year(year);
    }
}