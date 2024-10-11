use std::str::FromStr;

fn get_list(s: &str, e: &mut &str) -> bool {
    let mut s = s;
    loop {
        skip_space(&mut s);
        if !get_rnge(&s, e) && !get_number(&mut s, e) {
            break;
        }
        s = *e;

        skip_space(&mut s);
        if s.is_empty() {
            println!();
            return true;
        }
        if s.starts_with(',') {
            s = &s[1..];
            continue;
        }
        break;
    }
    *e = s;
    println!("\nSyntax error at {}", s);
    false
}

fn get_rnge(s: &str, e: &mut &str) -> bool {
    let mut s = s;
    let x;
    if !get_number(&mut s, &mut e) {
        return false;
    }
    x = i32::from_str(s).unwrap();
    s = *e;

    skip_space(&mut s);
    if !s.starts_with('-') {
        *e = s;
        return false;
    }
    s = &s[1..];
    let y;
    if !get_number(&mut s, e) {
        return false;
    }
    y = i32::from_str(s).unwrap();
    add_range(x, y)
}

fn get_number(s: &mut &str, e: &mut &str) -> bool {
    skip_space(s);
    let end = s.find(|c: char| !c.is_digit(10)).unwrap_or(s.len());
    if end == 0 {
        return false;
    }
    *e = &s[end..];
    true
}

fn skip_space(s: &mut &str) {
    while let Some(c) = s.chars().next() {
        if c.is_whitespace() {
            *s = &s[1..];
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
    for i in x..=y {
        print!("{} ", i);
    }
    true
}

fn main() {
    let mut end = "";

    if get_list("-6,-3--1,3-5,7-11,14,15,17-20", &mut end) {
        println!("Ok");
    }

    get_list("-6 -3--1,3-5,7-11,14,15,17-20", &mut end);
}