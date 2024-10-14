fn coef(n: i32, c: &mut [i64; 100]) {
    let mut i: usize;
    let mut j: usize;

    if n < 0 || n > 63 {
        std::process::abort(); // gracefully deal with range issue
    }

    c[0] = 1;
    for (i = 0; i < n as usize; c[0] = -c[0], i += 1) {
        c[1 + (j = i)] = 1;
        for (j = i; j > 0; j -= 1) {
            c[j] = c[j - 1] - c[j];
        }
    }
}

fn is_prime(n: i32) -> bool {
    let mut c = [0i64; 100];
    let mut i: usize;

    coef(n, &mut c);
    c[0] += 1;
    c[n as usize] -= 1;
    i = n as usize;
    while i > 0 && c[i - 1] % n as i64 == 0 {
        i -= 1;
    }

    i == 0
}

fn show(n: i32, c: &[i64; 100]) {
    let mut n = n as usize;
    loop {
        print!("{}{}x^{}", if c[n] >= 0 { '+' } else { '-' }, c[n].abs(), n);
        if n == 0 {
            break;
        }
        n -= 1;
    }
}

fn main() {
    let mut c = [0i64; 100];

    for n in 0..10 {
        coef(n, &mut c);
        print!("(x-1)^{} = ", n);
        show(n, &c);
        println!();
    }

    println!("\nprimes (never mind the 1):");
    for n in 1..=63 {
        if is_prime(n) {
            print!(" {}", n);
        }
    }

    println!();
}