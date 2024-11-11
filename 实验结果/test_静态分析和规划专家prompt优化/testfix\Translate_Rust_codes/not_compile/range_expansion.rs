use std::str::FromStr;
fn get_list(s: &str) -> Result<(), &'static str> {
    let mut chars = s.chars();
    loop {
        skip_space(&mut chars);
        if get_rnge(&mut chars).is_none() && get_number(&mut chars).is_none() {
            break;
        }
        skip_space(&mut chars);
        match chars.next() {
            Some('\0') => {
                print!("\n");
                return Ok(());
            }
            Some(',') => continue,
            _ => break,
        }
    }
    println!("\nSyntax error at {:?}", chars.as_str());
    Err("Syntax error")
}
fn get_rnge(chars: &mut std::str::Chars) -> bool {
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
fn get_number(chars: &mut std::str::Chars) -> Option<i64> {
    let s: String = chars.take_while(|c| c.is_digit(10) || *c == '-').collect();
    if s.is_empty() {
        None
    } else {
        s.parse::<i64>().ok()
    }
}
fn skip_space(chars: &mut std::str::Chars) {
    while let Some(c) = chars.clone().next() {
        if c.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }
}
fn add_number(x: i64) {
    print!("{} ", x);
}
fn add_range(x: i64, y: i64) -> bool {
    if y <= x {
        return false;
    }
    for num in x..=y {
        print!("{} ", num);
    }
    true
}
fn main() {
    // this is correct
    if get_list("-6,-3--1,3-5,7-11,14,15,17-20").is_ok() {
        println!("Ok");
    }
    // this is not. note the subtle error: "-6 -3" is parsed as range(-6, 3), so syntax error comes after that
    get_list("-6 -3--1,3-5,7-11,14,15,17-20").unwrap_err();
}