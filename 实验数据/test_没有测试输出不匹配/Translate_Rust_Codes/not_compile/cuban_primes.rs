type Llong = i64;

struct PrimeArray {
    ptr: Vec<Llong>,
    size: usize,
    capacity: usize,
}

fn allocate() -> PrimeArray {
    let capacity = 10;
    let mut ptr = Vec::with_capacity(capacity);
    ptr.resize(capacity, 0_i64); // Ensure the second argument is of type Llong (i64)
    PrimeArray {
        ptr,
        size: 0,
        capacity,
    }
}

fn push_back(primes: &mut PrimeArray, p: Llong) {
    if primes.size >= primes.capacity {
        let new_capacity = (3 * primes.capacity) / 2 + 1;
        primes.ptr.resize(new_capacity, 0_i64); // Ensure the second argument is of type Llong (i64)
        primes.capacity = new_capacity;
    }
    primes.ptr[primes.size] = p; // Ensure primes.ptr[primes.size] is of type Llong (i64)
    primes.size += 1; // Ensure primes.size is of type usize
}

fn main() {
    const CUTOFF: Llong = 200; // Ensure constants are of type Llong (i64)
    const BIGUN: Llong = 10000;
    const CHUNKS: Llong = 50;
    const LITTLE: Llong = BIGUN / CHUNKS;

    let mut primes = allocate();
    let mut c: Llong = 0_i64; // Ensure `c` has a concrete type `Llong` (i64)
    let mut show_each = true;
    let mut u: Llong = 0_i64; // Ensure `u` has a concrete type `Llong` (i64)
    let mut v: Llong = 1_i64; // Ensure `v` has a concrete type `Llong` (i64)

    push_back(&mut primes, 3);
    push_back(&mut primes, 5);

    println!("The first {} cuban primes:", CUTOFF);
    for i in 1.. {
        let mut found = false;
        if let Some(new_v) = v.checked_add(u + 6) {
            v = new_v;
        } else {
            // Handle overflow
            break;
        }
        if let Some(new_u) = u.checked_add(6) {
            u = new_u;
        } else {
            // Handle overflow
            break;
        }
        let mx = (v as f64).sqrt().ceil() as Llong; // Ensure `mx` is of type Llong (i64)

        for j in 0..primes.size {
            if primes.ptr[j] > mx {
                break;
            }
            if v % primes.ptr[j] == 0 {
                found = true;
                break;
            }
        }
        if !found {
            c += 1;
            if show_each {
                let mut z = primes.ptr[primes.size - 1] + 2; // Ensure `z` is of type Llong (i64)
                while z <= v - 2 {
                    let mut fnd = false;
                    for j in 0..primes.size {
                        if primes.ptr[j] > mx {
                            break;
                        }
                        if z % primes.ptr[j] == 0 {
                            fnd = true;
                            break;
                        }
                    }
                    if !fnd {
                        push_back(&mut primes, z);
                    }
                    if let Some(new_z) = z.checked_add(2) { // Ensure `new_z` is of type Llong (i64)
                        z = new_z;
                    } else {
                        // Handle overflow
                        break;
                    }
                }
                push_back(&mut primes, v);
                print!("{:11}", v);
                if c % 10 == 0 { // Ensure `c` is of type Llong (i64)
                    println!();
                }
                if c == CUTOFF { // Ensure `c` and `CUTOFF` are of type Llong (i64)
                    show_each = false;
                    println!("\nProgress to the {}th cuban prime: ", BIGUN);
                }
            }
            if c % LITTLE == 0 { // Ensure `c` and `LITTLE` are of type Llong (i64)
                print!(".");
                if c == BIGUN { // Ensure `c` and `BIGUN` are of type Llong (i64)
                    break;
                }
            }
        }
    }
    println!("\nThe {}th cuban prime is {}", c, v);
}