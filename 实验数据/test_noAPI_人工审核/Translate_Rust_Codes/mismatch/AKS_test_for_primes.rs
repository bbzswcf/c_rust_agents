use std::process;

static mut C: [i64; 100] = [0; 100];

fn coef(n: i32) {
    let n = n as usize;
    if n >= 64 {
        process::abort();
    }

    unsafe {
        C[0] = 1;
        for i in 0..n {
            C[0] = -C[0];
            for j in (1..=i).rev() {
                C[j] = C[j - 1] - C[j];
            }
            C[i + 1] = 1;
        }
    }
}

fn is_prime(n: i32) -> bool {
    let n = n as usize;
    coef(n as i32);
    unsafe {
        C[0] += 1;
        C[n] -= 1;
        let mut i = n;
        while i > 0 && C[i] % (n as i64) == 0 {
            i -= 1;
        }
        i == 0
    }
}

fn show(n: i32) {
    let n = n as usize;
    unsafe {
        for i in (0..=n).rev() {
            print!("{}{}x^{}", if C[i] > 0 { '+' } else { '-' }, C[i].abs(), i);
        }
    }
}

fn main() {
    for n in 0..10 {
        coef(n);
        print!("(x-1)^{} = ", n);
        show(n);
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