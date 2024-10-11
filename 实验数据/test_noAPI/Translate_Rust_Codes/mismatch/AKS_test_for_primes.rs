use std::process;

static mut C: [i64; 100] = [0; 100];

fn coef(n: usize) {
    if n >= 64 {
        process::abort();
    }

    unsafe {
        let mut i = 0;
        C[i] = 1; // Wrapped in unsafe block
        while i < n {
            C[0] = -C[0]; // Wrapped in unsafe block
            i += 1;
            let mut j = i;
            if j + 1 < C.len() { // Ensure index is within bounds
                C[j + 1] = 1; // Wrapped in unsafe block
            }
            while j > 0 {
                j -= 1;
                if j + 1 < C.len() { // Ensure index is within bounds
                    C[j + 1] = C[j].checked_sub(C[j + 1]).unwrap_or(0); // Handle subtraction overflow
                }
            }
        }
    }
}

fn is_prime(n: usize) -> bool {
    coef(n);
    unsafe {
        C[0] += 1; // Wrapped in unsafe block
        if n < C.len() { // Ensure index is within bounds
            C[n] -= 1; // Wrapped in unsafe block
        }
        let mut i = n;
        while i > 0 && i < C.len() && C[i] % (n as i64) == 0 { // Ensure index is within bounds
            i -= 1;
        }
        i == 0
    }
}

fn show(n: usize) {
    let mut n = n;
    loop {
        unsafe {
            if n < C.len() { // Ensure index is within bounds
                print!("{}{}x^{}", if C[n] < 0 { '-' } else { '+' }, C[n].abs(), n); // Wrapped in unsafe block
            }
        }
        if n == 0 {
            break;
        }
        n -= 1;
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