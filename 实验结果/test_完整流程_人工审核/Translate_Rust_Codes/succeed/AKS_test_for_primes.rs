fn coef(n: i32, c: &mut [i64; 100]) {
    if n < 0 || n > 63 {
        panic!("n must be between 0 and 63 inclusive");
    }

    c[0] = 1;
    for i in 1..=n {
        c[i as usize] = 1;
        for j in (1..i).rev() {
            if j as usize - 1 < 100 {
                c[j as usize] = c[(j - 1) as usize] - c[j as usize];
            }
        }
        c[0] = -c[0];
    }
}

fn show(n: i32, c: &[i64; 100]) {
    for i in (0..=n).rev() {
        let coef = c[i as usize];
        if coef != 0 {
            if coef > 0 && i != n {
                print!("+");
            }
            if coef == -1 && i > 0 {
                print!("-");
            } else if coef != 1 || i == 0 {
                print!("{}", coef);
            }
            if i > 0 {
                print!("x");
                if i > 1 {
                    print!("^{}", i);
                }
            }
        }
    }
}

fn is_prime(n: i32) -> bool {
    let mut c = [0i64; 100];
    coef(n, &mut c);

    c[0] += 1;
    c[n as usize] -= 1;

    for i in (1..=n).rev() {
        if c[i as usize] % n as i64 != 0 {
            return false;
        }
    }
    true
}

fn main() {
    let mut n: i32;
    let mut c = [0i64; 100];

    n = 0;
    while n < 10 {
        coef(n, &mut c);
        print!("(x-1)^{} = ", n);
        show(n, &c);
        print!("\n");
        n += 1;
    }

    print!("\nprimes (never mind the 1):");
    n = 1;
    while n <= 63 {
        if is_prime(n) {
            print!(" {}", n);
        }
        n += 1;
    }

    print!("\n");
}