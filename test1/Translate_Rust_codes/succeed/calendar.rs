use std::convert::TryInto; // Added: Import the TryInto trait to bring the try_into method into scope
use std::process;

const WD: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

struct Month {
    name: &'static str,
    days: i32,
    start_wday: i32,
    at: i32,
}

struct Calendar {
    months: [Month; 12],
}

impl Calendar {
    fn new() -> Self {
        Calendar {
            months: [
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
            ],
        }
    }

    fn init_months(&mut self, year: &mut i32, width: &mut i32, cols: &mut i32, lead: &mut i32, gap: &mut i32) {
        if (*year % 4 == 0 && *year % 100 != 0) || *year % 400 == 0 {
            self.months[1].days = 29;
        }

        *year -= 1;
        self.months[0].start_wday = (*year * 365 + *year / 4 - *year / 100 + *year / 400 + 1) % 7;

        for i in 1..12 {
            self.months[i].start_wday = (self.months[i - 1].start_wday + self.months[i - 1].days) % 7;
        }

        *cols = (*width + 2) / 22;
        while 12 % *cols != 0 {
            *cols -= 1;
        }
        *gap = if *cols - 1 != 0 { (*width - 20 * *cols) / (*cols - 1) } else { 0 };
        if *gap > 4 {
            *gap = 4;
        }
        *lead = (*width - (20 + *gap) * *cols + *gap + 1) / 2;
        *year += 1;
    }

    fn print_row(&mut self, row: i32, cols: i32, lead: i32, gap: i32) {
        let from = row * cols;
        let to = from + cols;
        space(lead);
        for c in from..to {
            let i = self.months[c as usize].name.len();
            space(((20 - i) / 2).try_into().unwrap()); // Modified: Convert result to i32
            print!("{}", self.months[c as usize].name);
            space((20 - i - (20 - i) / 2 + if c == to - 1 { 0 } else { gap as usize }).try_into().unwrap()); // Modified: Convert result to i32 and ensure all operands are of the same type
        }
        println!();

        space(lead);
        for c in from..to {
            for (i, wday) in WD.iter().enumerate() {
                print!("{}{}", wday, if i == 6 { "" } else { " " });
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
                if self.months[c as usize].at < self.months[c as usize].days {
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
                while i < self.months[c as usize].start_wday {
                    space(3);
                    i += 1;
                }
                while i < 7 && self.months[c as usize].at < self.months[c as usize].days {
                    print!("{:2}", self.months[c as usize].at + 1);
                    self.months[c as usize].at += 1;
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
                self.months[c as usize].start_wday = 0;
            }
            println!();
        }
        println!();
    }

    fn print_year(&mut self, year: i32, width: i32, cols: i32, lead: i32, gap: i32) {
        let buf = format!("{}", year);
        space((width - buf.len() as i32) / 2);
        println!("{}\n", buf);
        let mut row = 0;
        while row * cols < 12 {
            self.print_row(row, cols, lead, gap);
            row += 1;
        }
    }
}

fn space(n: i32) {
    for _ in 0..n {
        print!(" ");
    }
}

fn main() {
    let mut year = 1969;
    let mut width = 80;
    let mut cols = 0;
    let mut lead = 0;
    let mut gap = 0;
    let mut year_set = false;

    let args: Vec<String> = std::env::args().collect();
    let mut i = 1;
    while i < args.len() {
        if args[i] == "-w" {
            i += 1;
            if i == args.len() || args[i].parse::<i32>().unwrap() < 20 {
                eprintln!("bad args\nUsage: {} year [-w width (>= 20)]", args[0]);
                process::exit(1);
            }
            width = args[i].parse::<i32>().unwrap();
        } else if !year_set {
            if let Ok(y) = args[i].parse::<i32>() {
                if y > 0 {
                    year = y;
                    year_set = true;
                }
            }
        } else {
            eprintln!("bad args\nUsage: {} year [-w width (>= 20)]", args[0]);
            process::exit(1);
        }
        i += 1;
    }

    let mut calendar = Calendar::new();
    calendar.init_months(&mut year, &mut width, &mut cols, &mut lead, &mut gap);
    calendar.print_year(year, width, cols, lead, gap);
}