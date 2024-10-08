use std::str::FromStr;

fn get_list(s: &str) -> Result<(), &'static str> {
    let mut chars = s.chars().peekable();
    loop {
        skip_space(&mut chars);
        if !get_rnge(&mut chars) && !get_number(&mut chars) {
            break;
        }

        skip_space(&mut chars);
        match chars.peek() {
            Some(&'\0') => {
                println!();
                return Ok(());
            }
            Some(&',') => {
                chars.next();
                continue;
            }
            _ => break,
        }
    }
    println!("\nSyntax error at {:?}", chars.collect::<String>());
    Err("Syntax error")
}

fn get_rnge(chars: &mut std::iter::Peekable<std::str::Chars>) -> bool {
    let x = match get_number(chars) {
        Some(num) => num,
        None => return false,
    };

    skip_space(chars);
    if chars.next() != Some('-') {
        return false;
    }

    let y = match get_number(chars) {
        Some(num) => num,
        None => return false,
    };

    add_range(x, y)
}

fn get_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<i32> {
    let mut num_str = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_digit(10) || c == '-' {
            num_str.push(chars.next().unwrap());
        } else {
            break;
        }
    }
    num_str.parse().ok()
}

fn skip_space(chars: &mut std::iter::Peekable<std::str::Chars>) {
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }
}

fn add_number(x: i32) {
    print!("{} ", x);
}

fn add_range(x: i32, y: i32) -> bool {
    if y <= x {
        return false;
    }
    for num in x..=y {
        print!("{} ", num);
    }
    true
}

fn main() {
    // This is correct
    if get_list("-6,-3--1,3-5,7-11,14,15,17-20").is_ok() {
        println!("Ok");
    }

    // This is not. Note the subtle error: "-6 -3" is parsed as range(-6, 3), so syntax error comes after that
    get_list("-6 -3--1,3-5,7-11,14,15,17-20");
}