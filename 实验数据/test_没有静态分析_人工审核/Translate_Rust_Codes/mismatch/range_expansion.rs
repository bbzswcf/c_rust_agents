use std::str::FromStr;

// Modified: Introduced named lifetime parameter 'a
fn get_list<'a>(mut s: &'a str, e: &mut &'a str) -> bool {
    // Initialize `x` before using it
    let mut x = 0;
    loop {
        skip_space(s, e);
        if !get_rnge(s, e) && !get_number(&mut x, s, e) {
            break;
        }
        s = *e; // Modified: `s` is now mutable

        skip_space(s, e);
        if s.is_empty() {
            return true;
        }
        if s.starts_with(',') {
            *e = &s[1..]; // Modified: Correctly update the string slice to skip the comma
            continue;
        }
        break;
    }
    *e = s;
    println!("Syntax error at {}", s);
    false
}

// Modified: Introduced named lifetime parameter 'a
fn get_rnge<'a>(mut s: &'a str, e: &mut &'a str) -> bool {
    // Initialize `x` before using it
    let mut x = 0;
    // Initialize `y` before using it
    let mut y = 0;
    if !get_number(&mut x, s, e) {
        return false;
    }
    s = *e; // Modified: `s` is now mutable

    skip_space(s, e);
    if !s.starts_with('-') {
        *e = s;
        return false;
    }
    *e = &s[1..]; // Modified: Correctly update the string slice to skip the range delimiter
    if !get_number(&mut y, *e, e) {
        return false;
    }
    add_range(x, y)
}

fn add_number(x: i64) {
    print!("{} ", x);
}

fn add_range(x: i64, y: i64) -> bool {
    if y <= x {
        println!("Error: y should be greater than x");
        return false;
    }
    for i in x..=y {
        print!("{} ", i);
    }
    true
}

// Modified: Introduced named lifetime parameter 'a
fn skip_space<'a>(s: &'a str, e: &mut &'a str) {
    *e = s.trim_start(); // Modified: Correctly update the string slice to skip spaces
}

// Modified: Introduced named lifetime parameter 'a
fn get_number<'a>(x: &mut i64, s: &'a str, e: &mut &'a str) -> bool {
    if let Some(non_digit) = s.find(|c: char| !c.is_ascii_digit() && c != '-') {
        if non_digit < s.len() { // Ensure non_digit is within bounds
            *e = &s[non_digit..]; // Modified: Correctly update the string slice to skip the non-digit character
            if let Ok(num) = i64::from_str(&s[..non_digit]) {
                *x = num;
                return true;
            }
        }
    }
    false
}

fn main() {
    // Modified: Correctly initialized `e` as a mutable reference to a string slice
    let mut e = "";
    if get_list("-6,-3--1,3-5,7-11,14,15,17-20", &mut e) {
        println!("Ok");
    }

    // Modified: Correctly initialized `e` as a mutable reference to a string slice
    let mut e = "";
    get_list("-6 -3--1,3-5,7-11,14,15,17-20", &mut e);
}