fn jacobi(mut a: u64, mut n: u64) -> i32 {
    // Ensure a is reduced modulo n
    if a >= n {
        a %= n;
    }
    
    // Initialize result to 1
    let mut result = 1;
    
    // Handle the case when a is even
    while a & 1 == 0 {
        a >>= 1;
        if n & 7 == 3 || n & 7 == 5 {
            result = -result;
        }
    }
    
    // Correctly swap a and n using a tuple swap to avoid moving the value of `n`
    let (a, n) = (n, a);
    
    // Correct the condition for negating the result
    if a & 3 == 3 && n & 3 == 3 {
        result = -result;
    }
    
    // Reduce a modulo n
    a %= n;
    
    // Handle the case when a becomes zero
    if n == 1 {
        return result as i32; // Modified: Cast result to i32 before returning
    }
    
    result as i32 // Modified: Cast result to i32 before returning
}

fn print_table(kmax: u32, nmax: u32) {
    print!("n\\k|");
    for k in 0..=kmax {
        print!("{:>3}", k);
    }
    println!("\n----");
    for _ in 0..=kmax {
        print!("---");
    }
    println!();
    for n in (1..=nmax).step_by(2) {
        print!("{:>2} |", n);
        for k in 0..=kmax {
            print!("{:>3}", jacobi(k as u64, n as u64));
        }
        println!();
    }
}

fn main() {
    print_table(20, 21);
}