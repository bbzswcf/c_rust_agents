fn is_prime(n: u32) -> bool {
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut p = 1;
    while p * p < n {
        p += 4;
        if n % p == 0 {
            return false;
        }
        p += 2;
        if n % p == 0 {
            return false;
        }
    }
    true
}

fn reverse(n: u32) -> u32 {
    let mut r = 0;
    let mut n = n;
    while n != 0 {
        r = r * 10 + n % 10;
        n /= 10;
    }
    r
}

fn is_emirp(n: u32) -> bool {
    let r = reverse(n);
    r != n && is_prime(n) && is_prime(r)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let argc = args.len();

    let mut x;
    let mut c = 0;

    match argc {
        1 => {
            x = 11;
            while c < 20 {
                if is_emirp(x) {
                    print!(" {}", x);
                    c += 1;
                }
                x += 2;
            }
        }
        2 => {
            x = 7701;
            while x < 8000 {
                if is_emirp(x) {
                    print!(" {}", x);
                }
                x += 2;
            }
        }
        _ => {
            x = 11;
            loop {
                if is_emirp(x) {
                    c += 1;
                    if c == 10000 {
                        print!("{}", x);
                        break;
                    }
                }
                x += 2;
            }
        }
    }

    print!("\n");
}