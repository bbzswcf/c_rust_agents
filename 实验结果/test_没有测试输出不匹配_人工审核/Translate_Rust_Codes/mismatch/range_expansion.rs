use std::str::FromStr;

// Introduced a named lifetime parameter 'a for the function signature
fn get_list<'a>(s: &'a str, e: &mut &'a str) -> bool {
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
    println!("Syntax error at {}", s);
    false
}

// Introduced a named lifetime parameter 'a for the function signature
fn get_rnge<'a>(s: &'a str, e: &mut &'a str) -> bool {
    let mut s = s;
    let x;
    if !get_number(&mut s, e) { // Removed &mut from the second argument
        return false;
    }
    x = s.parse::<i64>().unwrap();
    s = *e;

    skip_space(&mut s);
    if !s.starts_with('-') {
        *e = s;
        return false;
    }
    s = &s[1..];
    let y;
    if !get_number(&mut s, e) { // Removed &mut from the second argument
        return false;
    }
    y = s.parse::<i64>().unwrap();
    add_range(x, y)
}

// Introduced a named lifetime parameter 'a for the function signature
fn get_number<'a>(s: &mut &'a str, e: &mut &'a str) -> bool {
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

fn add_number(x: i64) {
    print!("{} ", x);
}

fn add_range(x: i64, y: i64) -> bool {
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