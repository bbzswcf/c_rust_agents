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

// 修改: 修复未使用的变量警告
fn init_months(year: &mut i32, width: &mut i32, cols: &mut i32, lead: &mut i32, gap: &mut i32, months: &mut [Month; 12]) {
    if (*year % 4 == 0 && *year % 100 != 0) || *year % 400 == 0 {
        months[1].days = 29; // 修改: 修复闰年2月天数
    }

    *year -= 1;
    months[0].start_wday = (*year * 365 + *year / 4 - *year / 100 + *year / 400 + 1) % 7; // 修改: 修复起始星期计算

    for i in 1..12 {
        months[i].start_wday = (months[i - 1].start_wday + months[i - 1].days) % 7; // 修改: 修复起始星期计算
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

fn print_row(row: i32, lead: i32, cols: i32, gap: i32, months: &mut [Month; 12]) {
    let from = row * cols;
    let to = from + cols;
    space(lead);
    for c in from..to {
        let name_len = months[c as usize].name.len(); // 修改: 修复未使用的变量警告
        space((20 - name_len as i32) / 2);
        print!("{}", months[c as usize].name);
        space(20 - name_len as i32 - (20 - name_len as i32) / 2 + if c == to - 1 { 0 } else { gap });
    }
    println!();

    space(lead);
    for c in from..to {
        for (i, wday) in WD_AYS.iter().enumerate() {
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
            if months[c as usize].at < months[c as usize].days {
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
            while i < months[c as usize].start_wday {
                space(3);
                i += 1;
            }
            while i < 7 && months[c as usize].at < months[c as usize].days {
                print!("{:2}", months[c as usize].at + 1);
                months[c as usize].at += 1; // 修改: 修复日期递增逻辑
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
            months[c as usize].start_wday = 0; // 修改: 修复起始星期重置逻辑
        }
        println!();
    }
    println!();
}

fn print_year(year: i32, width: i32, cols: i32, lead: i32, gap: i32, months: &mut [Month; 12]) {
    let buf = format!("{}", year);
    space((width - buf.len() as i32) / 2);
    println!("{}", buf);
    println!();
    for row in 0..(12 / cols) {
        print_row(row, lead, cols, gap, months);
    }
}

fn main() {
    let mut year = 1969;
    let mut width = 80;
    let mut cols = 1; // 修改: 初始化cols为1
    let mut lead = 0; // 修改: 初始化lead为0
    let mut gap = 0; // 修改: 初始化gap为0
    let mut year_set = false; // 修改: 初始化year_set为false

    let args: Vec<String> = std::env::args().collect();
    let mut i = 1;
    while i < args.len() {
        if args[i] == "-w" {
            i += 1;
            if i == args.len() {
                eprint!("bad args\nUsage: {} year [-w width (>= 20)]\n", args[0]);
                std::process::exit(1);
            }
            match args[i].parse::<i32>() {
                Ok(w) if w >= 20 => width = w, // 修改: 确保宽度>=20
                _ => {
                    eprint!("Invalid width value. Must be an integer >= 20.\n");
                    std::process::exit(1);
                }
            }
        } else if !year_set {
            match args[i].parse::<i32>() {
                Ok(y) if y > 0 => year = y,
                _ => year = 1969,
            }
            year_set = true; // 修改: 设置year_set为true
        } else {
            eprint!("bad args\nUsage: {} year [-w width (>= 20)]\n", args[0]);
            std::process::exit(1);
        }
        i += 1;
    }

    let mut months = [
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

    init_months(&mut year, &mut width, &mut cols, &mut lead, &mut gap, &mut months);
    print_year(year, width, cols, lead, gap, &mut months);
}

static WD_AYS: [&'static str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];