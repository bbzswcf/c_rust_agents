use std::str::FromStr;

fn skip_space(s: &str) -> &str {
    s.trim_start()
}

fn get_number(s: &str) -> Option<(i32, &str)> {
    let end = s.find(|c: char| !c.is_digit(10)).unwrap_or(s.len());
    if end == 0 {
        None
    } else {
        s[..end].parse().ok().map(|num| (num, &s[end..]))
    }
}

fn get_list(s: &str) -> Result<(), &str> {
    let mut s = s;
    loop {
        s = skip_space(s);
        if s.is_empty() {
            println!();
            return Ok(());
        }

        if !get_rnge(s).or_else(|| get_number(s).map(|(num, rest)| { add_number(num); rest }))
            .map(|rest| s = rest)
            .is_some()
        {
            break;
        }

        s = skip_space(s);
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
    Err(s)
}

fn get_rnge(s: &str) -> Option<&str> {
    let (x, s) = get_number(s)?;
    let s = skip_space(s);
    if !s.starts_with('-') {
        return None;
    }
    let s = &s[1..];
    let (y, s) = get_number(s)?;
    if add_range(x, y) {
        Some(s)
    } else {
        None
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
    get_list("-6 -3--1,3-5,7-11,14,15,17-20").unwrap_err();
}