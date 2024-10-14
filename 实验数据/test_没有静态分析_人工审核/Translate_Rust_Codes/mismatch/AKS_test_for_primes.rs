use std::process;

static mut C: [i64; 100] = [0; 100];

fn coef(n: usize) {
    let mut i: usize;
    let mut j: usize;

    if n >= 64 {
        process::abort(); // gracefully deal with range issue
    }

    unsafe {
        C[0] = 1; // Initialization for the first iteration
        for i in 0..n {
            for j in (1..=i).rev() {
                C[j] = C[j - 1] - C[j]; // Correctly calculate binomial coefficients
            }
            if i > 0 {
                C[0] = -C[0]; // Update C[0] correctly for subsequent iterations
            }
        }
    }
}

fn is_prime(n: usize) -> bool {
    let mut i: usize;

    coef(n);
    unsafe {
        C[0] += 1; // Adjust C[0] before the loop
        C[n] -= 1;
        i = n;
        while i > 0 && C[i] % (n as i64) == 0 {
            i -= 1; // Correctly check if all coefficients are divisible by n
        }
    }

    i == 0
}

fn show(n: usize) {
    let mut n = n;
    unsafe {
        loop {
            print!("{}{}x^{}", if C[n] > 0 { '+' } else { '-' }, C[n].abs(), n); // Correctly format the output
            if n == 0 {
                break;
            }
            n -= 1;
        }
    }
}

fn main() {
    for n in 0..10 {
        coef(n);
        print!("(x-1)^{} = ", n);
        show(n);
        print!("\n");
    }

    print!("\nprimes (never mind the 1):");
    for n in 1..=63 {
        if is_prime(n) {
            print!(" {}", n); // Correctly print prime numbers
        }
    }

    print!("\n");
}