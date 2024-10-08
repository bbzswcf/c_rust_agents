fn jacobi(mut a: u64, mut n: u64) -> i32 {
    if n % 2 == 0 {
        panic!("n must be odd");
    }
    a %= n;
    let mut result = 1;
    while a != 0 {
        while a % 2 == 0 {
            a /= 2;
            let n_mod_8 = n % 8;
            if n_mod_8 == 3 || n_mod_8 == 5 {
                result = -result;
            }
        }
        std::mem::swap(&mut a, &mut n);
        if a % 4 == 3 && n % 4 == 3 {
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
    println!();
    print!("{}", "-".repeat(4 + 3 * (kmax as usize + 1)));
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
    //  print_table , Jacobi 
    print_table(20, 21);
}