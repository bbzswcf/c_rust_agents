use std::io::{self, Write};

fn skip_space(s: &mut &str) {
    while s.chars().next().map_or(false, char::is_whitespace) {
        *s = &s[1..];
    }
}

fn get_number(s: &str) -> Option<(i32, &str)> {
    let mut end = s;
    // Modified: Used `str::parse` for parsing integers
    let num = s.chars().take_while(|c| c.is_digit(10)).collect::<String>().parse().ok()?;
    if end == s {
        None
    } else {
        Some((num, end))
    }
}

fn get_list(s: &str) -> Result<(), &str> {
    let mut s = s;
    loop {
        skip_space(&mut s);
        // Modified: Created a temporary variable to hold the result of `get_rnge`
        let rnge_result = get_rnge(&mut s);
        if rnge_result.is_err() {
            break;
        }

        skip_space(&mut s);
        // Modified: Ensured mutable borrow is released before immutable borrow
        if s.is_empty() {
            println!();
            return Ok(());
        }
        if s.starts_with(',') {
            s = &s[1..];
            continue;
        }
        break;
    }
    println!("\nSyntax error at {}", s);
    // Modified: Returned a new string slice to avoid referencing a local variable
    Err("Syntax error")
}

// Modified: Introduced a named lifetime parameter
fn get_rnge<'a>(s: &'a mut &'a str) -> Result<(), &'a str> {
    // Modified: Used `map_err` to convert the error type
    let (x, s1) = get_number(s).ok_or(s).map_err(|e| *e)?;
    *s = s1;

    skip_space(s);
    if !s.starts_with('-') {
        return Err(s);
    }
    *s = &s[1..];

    let (y, s2) = get_number(s).ok_or(s).map_err(|e| *e)?;
    *s = s2;

    if !add_range(x, y) {
        return Err(s);
    }
    Ok(())
}

fn add_number(x: i32) {
    print!("{} ", x);
    io::stdout().flush().unwrap();
}

fn add_range(x: i32, y: i32) -> bool {
    if y <= x {
        return false;
    }
    for num in x..=y {
        print!("{} ", num);
    }
    io::stdout().flush().unwrap();
    true
}

fn main() {
    let input1 = "-6,-3--1,3-5,7-11,14,15,17-20";
    let input2 = "-6 -3--1,3-5,7-11,14,15,17-20";

    if get_list(input1).is_ok() {
        println!("Ok");
    }

    get_list(input2).unwrap_err();
}