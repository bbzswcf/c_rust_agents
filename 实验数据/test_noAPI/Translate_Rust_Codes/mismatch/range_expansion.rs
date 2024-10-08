fn get_list<'a>(s: &'a str, e: &mut &'a str) -> bool {
    let mut s = s;
    loop {
        skip_space(&mut s);
        if !get_rnge(&s, e) && !get_number(&mut 0, &s, e) {
            break;
        }
        s = *e;

        skip_space(&mut s);
        if s.is_empty() {
            println!();
            return true;
        }
        if s.starts_with(',') {
            // Ensure s is not empty before slicing
            if !s.is_empty() {
                s = &s[1..];
            }
            continue;
        }
        break;
    }
    *e = s;
    // Print specific part of the string that caused the error
    if let Some(error_index) = s.find(|c: char| !c.is_digit(10) && c != '-' && c != ',') {
        // Modified: Avoid potential panic by using `get` instead of `unwrap`
        if let Some(error_char) = s.chars().nth(error_index) {
            println!("Syntax error at index {}: '{}'", error_index, error_char);
        } else {
            println!("Syntax error at end of string");
        }
    } else {
        println!("Syntax error at end of string");
    }
    false
}

// Modified: Changed function signature to accept immutable references
fn get_rnge<'a>(s: &'a str, e: &mut &'a str) -> bool {
    let mut x = 0;
    let mut ee = s;
    // Avoid borrowing `ee` as mutable more than once at a time
    if !get_number(&mut x, &ee, &mut ee) {
        return false;
    }
    let mut s = ee;

    skip_space(&mut s);
    if !s.starts_with('-') {
        *e = s;
        return false;
    }
    s = &s[1..];
    let mut y = 0;
    if !get_number(&mut y, &s, e) {
        return false;
    }
    add_range(x, y)
}

fn add_number(x: i32) {
    print!("{} ", x);
}

fn add_range(x: i32, y: i32) -> bool {
    // Handle invalid range more gracefully
    if y <= x {
        println!("Invalid range: {} to {}", x, y);
        return false;
    }
    for i in x..=y {
        print!("{} ", i);
    }
    true
}

fn skip_space(s: &mut &str) {
    // Handle all types of whitespace characters
    while !s.is_empty() && s.chars().next().map_or(false, |c| c.is_whitespace()) {
        *s = &s[1..];
    }
}

// Modified: Changed function signature to accept immutable references
fn get_number<'a>(x: &mut i32, s: &'a str, e: &mut &'a str) -> bool {
    if let Some(end) = s.find(|c: char| !c.is_digit(10) && c != '-') {
        *e = &s[end..];
        // Handle negative numbers and leading zeros
        if let Ok(parsed) = s[..end].parse::<i32>() {
            *x = parsed;
        } else {
            *x = 0;
        }
        true
    } else {
        false
    }
}

fn main() {
    let mut end = "";

    // Modified: Pass a mutable reference to `end`
    if get_list("-6,-3--1,3-5,7-11,14,15,17-20", &mut end) {
        println!("Ok");
    }

    // Modified: Pass a mutable reference to `end`
    get_list("-6 -3--1,3-5,7-11,14,15,17-20", &mut end);
}