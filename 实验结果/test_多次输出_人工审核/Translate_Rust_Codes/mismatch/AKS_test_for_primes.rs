fn coef(n: usize, c: &mut [i64; 100]) {
    if n > 63 {
        panic!("n out of range");
    }

    c[0] = 1;
    for i in 0..n {
        c[0] = -c[0];
        for j in (1..=i).rev() {
            c[j] = c[j - 1] - c[j];
        }
        c[i + 1] = 1;
    }
}

fn is_prime(n: usize, c: &mut [i64; 100]) -> bool {
    coef(n, c);
    c[0] += 1;
    c[n] -= 1;

    for i in (0..=n).rev() {
        if c[i] % (n as i64) != 0 {
            return i == 0;
        }
    }
    false
}

fn show(n: usize, c: &[i64; 100]) {
    for i in (0..=n).rev() {
        print!("{}{}x^{}", if c[i] > 0 { "+" } else { "" }, c[i], i);
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
        if is_prime(n, &mut c) {
            print!(" {}", n);
        }
    }

    println!();
}