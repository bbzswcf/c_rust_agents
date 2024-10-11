use std::process;

static mut C: [i64; 100] = [0; 100];

fn coef(n: usize) {
    if n > 63 {
        process::abort(); // gracefully deal with range issue
    }

    unsafe {
        C[0] = 1;
        for i in 1..=n {
            C[i] = 1;
            for j in (1..i).rev() {
                C[j] = C[j - 1] + C[j]; // Corrected: Use addition to match Pascal's triangle
            }
        }
        // Adjust coefficients for (x-1)^n
        for i in 0..=n {
            if i % 2 != 0 {
                C[i] = -C[i];
            }
        }
    }
}

fn is_prime(n: usize) -> bool {
    coef(n);
    unsafe {
        for i in 1..n {
            if C[i] % (n as i64) != 0 {
                return false; // Corrected: Check if all coefficients (except the first and last) are divisible by n
            }
        }
    }
    true
}

fn show(n: usize) {
    unsafe {
        for i in (0..=n).rev() {
            if C[i] != 0 { // Handle zero coefficients
                if i != n {
                    if C[i] > 0 {
                        print!("+"); // Handle positive coefficients
                    } else {
                        print!("-"); // Handle negative coefficients
                    }
                } else {
                    if C[i] < 0 {
                        print!("-"); // Handle negative leading coefficient
                    }
                }
                if i == 0 {
                    print!("{}", C[i].abs()); // Handle constant term
                } else if i == 1 {
                    print!("{}x", C[i].abs()); // Handle x term
                } else {
                    print!("{}x^{}", C[i].abs(), i); // Handle other terms
                }
            }
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

    println!("\nprimes (never mind the 1):");
    for n in 1..=63 {
        if is_prime(n) {
            print!(" {}", n);
        }
    }

    print!("\n");
}