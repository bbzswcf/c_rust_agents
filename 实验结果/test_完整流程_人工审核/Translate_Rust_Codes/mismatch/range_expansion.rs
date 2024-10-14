use std::str::FromStr;

fn get_list<'a>(mut s: &'a str) -> bool {
    loop {
        skip_space(&mut s);
        if !get_rnge(&mut s) && !get_number(&mut 0, &mut s) { // Removed unused variable x
            break;
        }
        // Removed: Incorrectly updating `s` to an empty string

        skip_space(&mut s);
        if s.is_empty() {
            println!();
            return true;
        }
        if s.starts_with(',') {
            s = &s[1..]; // Correctly update `s` to the substring after the comma
            continue;
        }
        break;
    }
    println!("Syntax error at {}", s);
    false
}

fn get_rnge<'a>(mut s: &'a str) -> bool {
    let mut x = 0; // Initialize x before using it
    let mut y = 0; // Initialize y before using it
    if !get_number(&mut x, &mut s) {
        return false;
    }
    // Removed: Incorrectly updating `s` to an empty string

    skip_space(&mut s);
    if !s.starts_with('-') {
        return false;
    }
    s = &s[1..]; // Correctly update `s` to the substring after the dash
    if !get_number(&mut y, &mut s) {
        return false;
    }
    add_range(x, y)
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

fn skip_space<'a>(s: &mut &'a str) {
    let trimmed = s.trim_start();
    *s = trimmed; // Correctly update `s` to the trimmed string
}

fn get_number<'a>(x: &mut i64, s: &mut &'a str) -> bool {
    if let Some(non_digit) = s.find(|c: char| !c.is_digit(10) && c != '-') {
        // Modified: Handle negative signs correctly
        if s.starts_with('-') {
            *x = i64::from_str(&s[..non_digit]).unwrap_or(0);
        } else {
            *x = i64::from_str(&s[..non_digit]).unwrap_or(0);
        }
        *s = &s[non_digit..]; // Correctly update `s` to the substring after the non-digit character
    } else {
        *x = i64::from_str(s).unwrap_or(0); // Correctly update `x` to the parsed number
    }
    true
}

fn main() {
    if get_list("-6,-3--1,3-5,7-11,14,15,17-20") {
        println!("Ok");
    }

    get_list("-6 -3--1,3-5,7-11,14,15,17-20");
}