type Llong = i64;

struct PrimeArray {
    ptr: Vec<Llong>,
}

fn allocate() -> PrimeArray {
    // Initialize the vector with default values (0 in this case)
    PrimeArray {
        ptr: vec![0; 10],
    }
}

fn push_back(primes: &mut PrimeArray, p: Llong) {
    // The push method already handles capacity checks internally
    primes.ptr.push(p);
}

fn integer_sqrt(n: Llong) -> Llong {
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

fn main() {
    const CUTOFF: Llong = 200;
    const BIGUN: Llong = 10000;
    const CHUNKS: Llong = 50;
    const LITTLE: Llong = BIGUN / CHUNKS;

    let mut primes = allocate();
    // Initialize c to 0 to correctly count the number of cuban primes found
    let mut c = 0;
    // Initialize u and v to 0 and 1 respectively to match the logic of the loop
    let mut u = 0;
    let mut v = 1;

    push_back(&mut primes, 3);
    push_back(&mut primes, 5);

    println!("The first {} cuban primes:", CUTOFF);
    for i in 1.. {
        let mut found = false;
        v += u + 6;
        u += 6;
        // Ensure mx is calculated correctly based on the logic of the loop
        let mx = integer_sqrt(v);

        // Store the length of primes.ptr to avoid recomputation
        let primes_len = primes.ptr.len();
        for j in 0..primes_len {
            if v % primes.ptr[j] == 0 {
                found = true;
                break;
            }
            // Stop the check once the divisor exceeds the square root of v
            if primes.ptr[j] > mx {
                break;
            }
        }
        if !found {
            c += 1;
            let mut z = primes.ptr[primes_len - 1] + 2;
            while z <= v - 2 {
                let mut fnd = false;
                // Use the stored length to avoid recomputation
                for j in 0..primes_len {
                    if z % primes.ptr[j] == 0 {
                        fnd = true;
                        break;
                    }
                    // Stop the check once the divisor exceeds the square root of z
                    if primes.ptr[j] > integer_sqrt(z) {
                        break;
                    }
                }
                if !fnd {
                    push_back(&mut primes, z);
                }
                z += 2;
            }
            push_back(&mut primes, v);
            print!("{:11}", v);
            // Ensure the condition correctly checks for the correct modulo value
            if c % 10 == 0 {
                println!();
            }
            // Ensure the condition correctly checks for the correct cutoff value
            if c == CUTOFF {
                println!("\nProgress to the {}th cuban prime: ", BIGUN);
            }
            // Ensure the condition correctly checks for the correct modulo value
            if c % LITTLE == 0 {
                print!(".");
                // Ensure the condition correctly checks for the correct bigun value
                if c == BIGUN {
                    break;
                }
            }
        }
    }
    // Ensure the print statement correctly prints the correct values
    println!("\nThe {}th cuban prime is {}", c, v);
}