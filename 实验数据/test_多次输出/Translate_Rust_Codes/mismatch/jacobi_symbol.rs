fn jacobi(mut a: u64, mut n: u64) -> i32 {
    if a >= n {
        a %= n;
    }
    let mut result = 1;
    while a != 0 {
        while a & 1 == 0 {
            a >>= 1;
            if n & 7 == 3 || n & 7 == 5 {
                result = -result;
            }
        }
        (a, n) = (n, a);
        if a & 3 == 3 && n & 3 == 3 {
            result = -result;
        }
        a %= n;
    }
    if n == 1 {
        result
    } else {
        0
    }
}

fn print_table(kmax: u32, nmax: u32) {
    print!("n\\k|");
    for k in 0..=kmax {
        print!("{:>3}", k);
    }
    println!("\n----{}", "---".repeat((kmax + 1) as usize));
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