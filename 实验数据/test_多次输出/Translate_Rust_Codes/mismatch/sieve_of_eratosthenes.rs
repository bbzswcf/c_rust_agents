fn eratosthenes(n: usize) -> (Vec<bool>, usize) {
    if n < 2 {
        return (Vec::new(), 0);
    }

    let mut sieve = vec![false; n + 1];
    sieve[0] = true;
    sieve[1] = true;
    let m = (n as f64).sqrt() as usize;
    let mut primes_count = n - 1;

    for i in 2..=m {
        if !sieve[i] {
            for j in (i * i..=n).step_by(i) {
                if !sieve[j] {
                    sieve[j] = true;
                    primes_count -= 1;
                }
            }
        }
    }

    (sieve, primes_count)
}

fn sieve(a: &mut [bool], n: usize) {
    for i in 2..=n {
        a[i] = true;
    }

    for i in 2..=n {
        println!("\ni:{}", i);
        if a[i] {
            for j in i.. {
                let index = i * j;
                if index > n {
                    break;
                }
                println!("\nj:{}", j);
                println!("\nBefore a[{}*{}]: {}", i, j, a[index]);
                a[index] = false;
                println!("\nAfter a[{}*{}]: {}", i, j, a[index]);
            }
        }
    }

    println!("\nPrimes numbers from 1 to {} are : ", n);
    for i in 2..=n {
        if a[i] {
            print!("{}, ", i);
        }
    }
    println!("\n\n");
}

fn main() {
    let n = 10;
    let mut array = vec![false; n + 1];
    sieve(&mut array, n);
}