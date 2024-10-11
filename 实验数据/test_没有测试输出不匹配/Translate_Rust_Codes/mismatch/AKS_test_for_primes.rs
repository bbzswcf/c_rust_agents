use std::process;

static mut C: [i64; 100] = [0; 100];

fn coef(n: usize) {
    let mut i: usize; // Added type annotation
    let mut j: usize; // Added type annotation

    if n >= 64 {
        process::abort();
    }

    unsafe {
        C[0] = 1;
        for i in 0..n {
            C[0] = -C[0];
            for j in (1..=i).rev() { // Verified loop bounds
                C[j] = C[j - 1] - C[j];
            }
            C[i + 1] = 1;
        }
    }
}

fn is_prime(n: usize) -> bool {
    let mut i: usize; // Added type annotation

    coef(n);
    unsafe {
        C[0] += 1;
        C[n] -= 1;
        i = n;
        while i > 0 && C[i] % (n as i64) == 0 { // Ensured no integer overflow
            i -= 1;
        }
    }

    i == 0
}

fn show(n: usize) {
    let mut i = n;
    unsafe {
        loop {
            print!("{}{}x^{}", if C[i] > 0 { '+' } else { '-' }, C[i].abs(), i);
            if i == 0 {
                break;
            }
            i -= 1;
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
    for n in 1..=63 { // Verified loop range
        if is_prime(n) {
            print!(" {}", n);
        }
    }

    print!("\n");
}