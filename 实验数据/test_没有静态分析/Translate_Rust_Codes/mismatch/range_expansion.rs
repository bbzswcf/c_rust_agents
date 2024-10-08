fn get_list<'a>(s: &'a str, e: &mut &'a str) -> bool {
    let mut s = s;
    skip_space(&mut s);
    if s.is_empty() {
        println!("Syntax error at {}", s);
        return false;
    }
    loop {
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

fn get_rnge<'a>(s: &'a str, e: &mut &'a str) -> bool {
    let mut s = s;
    let x;
    if !get_number(&mut s, e) {
        return false;
    }
    match s[..s.find(|c: char| !c.is_digit(10)).unwrap_or(s.len())].parse() {
        Ok(num) => x = num,
        Err(e) => {
            println!("Parse error: {}", e);
            return false;
        }
    }
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
    match s[..s.find(|c: char| !c.is_digit(10)).unwrap_or(s.len())].parse() {
        Ok(num) => y = num,
        Err(e) => {
            println!("Parse error: {}", e);
            return false;
        }
    }
    if y <= x {
        println!("Syntax error at {}", s);
        return false;
    }
    add_range(x, y)
}

fn get_number<'a>(s: &mut &'a str, e: &mut &'a str) -> bool {
    skip_space(s);
    let mut end = s.find(|c: char| !c.is_digit(10)).unwrap_or(s.len());
    if s.starts_with('-') {
        // Check if '-' is followed by at least one digit
        if end == 1 || !s[1..].chars().next().unwrap().is_digit(10) {
            return false; // No digits after '-'
        }
        end += 1; // Include '-' in the parsed number
    } else if end == 0 {
        return false; // No digits found
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