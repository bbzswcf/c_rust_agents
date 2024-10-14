fn eratosthenes(n: usize) -> Vec<bool> {
    if n < 2 {
        return vec![];
    }

    let mut sieve = vec![false; n + 1];
    sieve[0] = true;
    sieve[1] = true;
    let m = (n as f64).sqrt() as usize;

    for i in 2..=m {
        if !sieve[i] {
            for j in (i * i..=n).step_by(i) {
                if !sieve[j] {
                    sieve[j] = true;
                }
            }
        }
    }

    sieve
}

fn main() {
    let n = 10;
    let sieve = eratosthenes(n);

    println!("Primes numbers from 1 to {} are:", n);
    for i in 2..=n {
        if !sieve[i] {
            print!("{}, ", i);
        }
    }
    println!();
}