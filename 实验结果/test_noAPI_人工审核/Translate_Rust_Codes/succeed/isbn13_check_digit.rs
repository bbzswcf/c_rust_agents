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
        let digit = ch.to_digit(10).unwrap();
        if count % 2 == 1 {
            sum += 3 * digit;
        } else {
            sum += digit;
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
    for isbn in isbns.iter() {
        println!("{}: {}", isbn, if check_isbn13(isbn) { "good" } else { "bad" });
    }
}