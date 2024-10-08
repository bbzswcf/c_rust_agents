use std::str::FromStr;

fn get_list(s: &str) -> Result<(), &'static str> {
    let mut chars = s.chars().enumerate(); // Enumerate to track position
    let mut position = 0; // Track current position

    loop {
        skip_space(&mut chars, &mut position);

        // Handle optional results
        if !get_rnge(&mut chars, &mut position) && !get_number(&mut chars, &mut position).is_some() {
            break;
        }

        skip_space(&mut chars, &mut position);

        // Check if the iterator is exhausted
        if chars.next().is_none() {
            println!();
            return Ok(());
        }

        // Ensure comma is correctly identified and handled
        if chars.next().map(|(_, ch)| ch) == Some(',') {
            continue;
        }
        break;
    }

    // Print exact position of syntax error
    println!("\nSyntax error at position {}", position);
    Err("Syntax error")
}

fn get_rnge(chars: &mut std::iter::Enumerate<std::str::Chars>, position: &mut usize) -> bool {
    let x = match get_number(chars, position) {
        Some(num) => num,
        None => return false,
    };

    skip_space(chars, position);

    // Ensure range format is correct
    if chars.next().map(|(_, ch)| ch) != Some('-') {
        return false;
    }

    let y = match get_number(chars, position) {
        Some(num) => num,
        None => return false,
    };

    add_range(x, y)
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

fn skip_space(chars: &mut std::iter::Enumerate<std::str::Chars>, position: &mut usize) {
    // Skip all types of whitespace characters
    while let Some((i, ch)) = chars.next() {
        *position = i + 1;
        if ch.is_whitespace() {
            continue;
        } else {
            break;
        }
    }
}

fn get_number(chars: &mut std::iter::Enumerate<std::str::Chars>, position: &mut usize) -> Option<i64> {
    let mut num_str = String::new();
    let mut is_negative = false;

    // Handle negative numbers correctly
    if let Some((i, ch)) = chars.next() {
        *position = i + 1;
        if ch == '-' {
            is_negative = true;
        } else if ch.is_digit(10) {
            num_str.push(ch);
        } else {
            return None;
        }
    }

    // Continue reading digits
    while let Some((i, ch)) = chars.next() {
        *position = i + 1;
        if ch.is_digit(10) {
            num_str.push(ch);
        } else {
            break;
        }
    }

    if num_str.is_empty() {
        None
    } else {
        let num = num_str.parse::<i64>().ok()?;
        if is_negative {
            Some(-num)
        } else {
            Some(num)
        }
    }
}

fn main() {
    // Correct input
    if get_list("-6,-3--1,3-5,7-11,14,15,17-20").is_ok() {
        println!("Ok");
    }

    // Incorrect input with subtle error
    get_list("-6 -3--1,3-5,7-11,14,15,17-20");
}