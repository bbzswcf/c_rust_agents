const ONES: [&str; 20] = [
    "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", 
    "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", 
    "seventeen", "eighteen", "nineteen"
];
const TENS: [&str; 10] = [
    "", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"
];
const LLIONS: [&str; 5] = [
    "", "thousand", "million", "billion", "trillion"
];
const MAXILLION: usize = LLIONS.len() * 3 - 3;

fn say_hundred(s: &str, len: usize, depth: usize, mut has_lead: bool) -> bool {
    let mut c = [0; 3];
    let mut i: isize = -3;
    for j in 0..3 {
        if (len as isize + i + j as isize) >= 0 {
            // 修改: 使用 `unwrap_or(Some('0')).unwrap()` 避免 panic
            let ch = s.chars().nth((len as isize + i + j as isize) as usize).unwrap_or(Some('0')).unwrap();
            c[j] = ch as u8 - b'0';
        } else {
            c[j] = 0;
        }
    }
    if c[0] + c[1] + c[2] == 0 {
        return false;
    }

    if c[0] != 0 {
        print!("{} hundred", ONES[c[0] as usize]);
        has_lead = true;
    }

    if has_lead && (c[1] != 0 || c[2] != 0) {
        print!(
            "{}",
            if depth == 0 && c[0] != 0 && (c[1] != 0 || c[2] != 0) {
                "and "
            } else if c[0] != 0 {
                " "
            } else {
                ""
            }
        );
    }

    if c[1] < 2 {
        if c[1] != 0 || c[2] != 0 {
            print!("{}", ONES[(c[1] * 10 + c[2]) as usize]);
        }
    } else {
        if c[1] != 0 {
            print!("{}", TENS[c[1] as usize]);
            if c[2] != 0 {
                print!("-");
            }
        }
        if c[2] != 0 {
            print!("{}", ONES[c[2] as usize]);
        }
    }

    true
}

fn say_maxillion(s: &str, len: usize, depth: usize, mut has_lead: bool) -> bool {
    let n = len / 3;
    let r = len % 3;
    let mut s = s;
    let e = s.get(r..).unwrap_or_default(); // 修改: 使用 `unwrap_or_default()` 避免 panic
    let r = if r == 0 { 3 } else { r };

    loop {
        if say_hundred(s, r, n, has_lead) && n != 0 {
            has_lead = true;
            print!(" {}", LLIONS[n]);
            if depth == 0 {
                print!(", ");
            } else {
                print!(" ");
            }
        }
        s = e;
        let e = e.get(3..).unwrap_or_default(); // 修改: 使用 `unwrap_or_default()` 避免 panic
        if n == 0 {
            break;
        }
        let n = n - 1;
    }

    true
}

fn say_number(s: &str) {
    let mut s = s.trim();
    let mut got_sign = 0;

    // 修改: 使用 `unwrap_or(Some('0')).unwrap()` 避免 panic
    let ch = s.chars().next().unwrap_or(Some('0')).unwrap();
    if ch < '0' || ch > '9' {
        if s.starts_with('-') {
            got_sign = -1;
        } else if s.starts_with('+') {
            got_sign = 1;
        } else {
            println!("not a number");
            return;
        }
        s = &s[1..];
    } else {
        got_sign = 1;
    }

    while s.starts_with('0') {
        s = &s[1..];
        if s.is_empty() {
            println!("zero");
            return;
        }
    }

    let len = s.len();
    if len == 0 {
        println!("not a number");
        return;
    }

    for c in s.chars() {
        if c < '0' || c > '9' {
            println!("(not a number)");
            return;
        }
    }

    if got_sign == -1 {
        print!("minus ");
    }

    let n = len / MAXILLION;
    let r = len % MAXILLION;
    let mut s = s;
    let mut end = if r == 0 {
        s.get(MAXILLION..).unwrap_or_default() // 修改: 使用 `unwrap_or_default()` 避免 panic
    } else {
        s.get(r..).unwrap_or_default() // 修改: 使用 `unwrap_or_default()` 避免 panic
    };
    let mut has_lead = false;

    loop {
        if say_maxillion(s, r, n, has_lead) {
            for _ in 0..n {
                print!(" {}", LLIONS[MAXILLION / 3]);
            }
            if n != 0 {
                print!(", ");
            }
        }
        if n == 0 {
            break;
        }
        let n = n - 1;
        let r = MAXILLION;
        let s = end;
        let end = end.get(MAXILLION..).unwrap_or_default(); // 修改: 使用 `unwrap_or_default()` 避免 panic
    }

    println!();
}

fn main() {
    say_number("-42");
    say_number("1984");
    say_number("10000");
    say_number("1024");
    say_number("1001001001001");
    say_number("123456789012345678901234567890123456789012345678900000001");
}