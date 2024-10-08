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

fn get_list<'a>(mut s: &'a str, e: &mut &'a str) -> bool {
    let mut number: i64 = 0;
    let mut has_error = false;
    while let Some(c) = s.chars().next() {
        skip_space(s, e);
        if !get_rnge(s, e) && !get_number(&mut number, s, e) {
            has_error = true;
            break;
        }
        s = *e;

        skip_space(s, e);
        if s.is_empty() {
            print!("\n");
            return true;
        }
        if s.starts_with(',') {
            s = &s[1..];
            continue;
        }
        has_error = true;
        break;
    }
    *e = s;
    if has_error {
        // 修改: 递归调用get_list以处理错误
        return get_list(s, e);
    }
    true
}

fn get_rnge<'a>(mut s: &'a str, e: &mut &'a str) -> bool {
    let mut start: i64 = 0;
    let mut end_number: i64 = 0;
    let mut ee: &str = "";
    if !get_number(&mut start, s, &mut ee) {
        return false;
    }
    s = ee;

    skip_space(s, e);
    if !s.starts_with('-') {
        *e = s;
        return false;
    }
    s = &s[1..];
    if !get_number(&mut end_number, s, e) {
        return false;
    }
    add_range(start, end_number)
}

fn skip_space<'a>(s: &'a str, e: &mut &'a str) {
    *e = s.trim_start();
}

fn get_number<'a>(x: &mut i64, s: &'a str, e: &mut &'a str) -> bool {
    if let Some(pos) = s.find(|c: char| !c.is_digit(10) && c != '-') {
        *e = &s[pos..];
        if let Ok(num) = s[..pos].parse::<i64>() {
            *x = num;
            return true;
        }
    } else if let Ok(num) = s.parse::<i64>() {
        *x = num;
        *e = "";
        return true;
    }
    false
}

fn main() {
    let mut end = "";

    get_list("-6 -3--1,3-5,7-11,14,15,17-20", &mut end);

    if get_list("-6,-3--1,3-5,7-11,14,15,17-20", &mut end) {
        println!("Ok");
    }
}