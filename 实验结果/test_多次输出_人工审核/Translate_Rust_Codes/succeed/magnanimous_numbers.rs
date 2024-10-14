fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    if n % 3 == 0 {
        return n == 3;
    }
    let mut d = 5;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 2;
        if n % d == 0 {
            return false;
        }
        d += 4;
    }
    true
}

fn ord(n: i32) -> String {
    let m = n % 100;
    let suffix = if (4..=20).contains(&m) {
        "th"
    } else {
        match m % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        }
    };
    format!("{}{}", n, suffix)
}

fn is_magnanimous(n: u64) -> bool {
    if n < 10 {
        return true;
    }
    let mut p = 10;
    loop {
        let q = n / p;
        let r = n % p;
        if !is_prime(q + r) {
            return false;
        }
        if q < 10 {
            break;
        }
        p *= 10;
    }
    true
}

fn list_mags(from: i32, thru: i32, digs: i32, per_line: i32) {
    let mut i = 0;
    let mut c = 0;
    let res1 = ord(from);
    let res2 = ord(thru);
    if from < 2 {
        println!("\nFirst {} magnanimous numbers:", thru);
    } else {
        println!("\n{} through {} magnanimous numbers:", res1, res2);
    }
    while c < thru {
        if is_magnanimous(i) {
            c += 1;
            if c >= from {
                print!("{:width$} ", i, width = digs as usize);
                if c % per_line == 0 {
                    println!();
                }
            }
        }
        i += 1;
    }
}

fn main() {
    list_mags(1, 45, 3, 15);
    list_mags(241, 250, 1, 10);
    list_mags(391, 400, 1, 10);
}