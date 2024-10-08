fn check_isbn13(isbn: &str) -> bool {
    let mut count = 0;
    let mut sum = 0;

    for ch in isbn.chars() {
        if ch == ' ' || ch == '-' {
            continue;
        }
        if ch < '0' || ch > '9' {
            return false;
        }
        if count & 1 != 0 {
            sum += 3 * (ch as i32 - '0' as i32);
        } else {
            sum += ch as i32 - '0' as i32;
        }
        count += 1;
    }

    if count != 13 {
        return false;
    }

    sum % 10 == 0
}

fn main() {
    let isbns = ["978-1734314502", "978-1734314509", "978-1788399081", "978-1788399083"];
    for &isbn in &isbns {
        println!("{}: {}", isbn, if check_isbn13(isbn) { "good" } else { "bad" });
    }
}