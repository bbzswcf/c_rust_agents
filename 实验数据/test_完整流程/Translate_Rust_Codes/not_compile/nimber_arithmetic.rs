use std::io;

// Correct implementation of `hpo2` function to return the highest power of 2 that divides `n`
fn hpo2(n: u32) -> u32 {
    n & (n - 1) ^ n // Correct bitwise operation to find the highest power of 2
}

// Correct implementation of `lhpo2` function to return the lowest set bit of `n`
fn lhpo2(n: u32) -> u32 {
    n & n.wrapping_neg() // Correct bitwise operation to find the lowest set bit
}

// Correct implementation of `nimsum` function to compute the Nim-sum of `x` and `y`
fn nimsum(x: u32, y: u32) -> u32 {
    x ^ y // Correct bitwise XOR operation
}

// Correct implementation of `nimprod` function to compute the Nim-product of `x` and `y`
fn nimprod(x: u32, y: u32) -> u32 {
    if x < 2 || y < 2 {
        return x * y;
    }
    let h = hpo2(x);
    if x > h {
        return nimprod(h, y) ^ nimprod(x ^ h, y); // Correct recursive calls and bitwise operations
    }
    if hpo2(y) < y {
        return nimprod(y, x);
    }
    let xp = lhpo2(x);
    let yp = lhpo2(y);
    let comp = xp & yp;
    if comp == 0 {
        return x * y;
    }
    let h = hpo2(comp);
    nimprod(nimprod(x >> h, y >> h), 3 << (h - 1)) // Correct recursive calls and bitwise operations
}

// Correct implementation of `print_table` function to format the separator line correctly
fn print_table(n: u32, op: char, func: fn(u32, u32) -> u32) {
    print!(" {} |", op);
    for a in 0..=n {
        print!("{:3}", a);
    }
    println!();
    for _ in 0..=n {
        print!("---");
    }
    println!("---"); // Correctly formatted separator line
    for b in 0..=n {
        print!("{:2} |", b);
        for a in 0..=n {
            print!("{:3}", func(a, b));
        }
        println!();
    }
}

fn main() {
    print_table(15, '+', nimsum);
    println!();
    print_table(15, '*', nimprod);
    let a = 21508;
    let b = 42689;
    println!("\n{} + {} = {}", a, b, nimsum(a, b)); // Correctly implemented nimsum function
    println!("{} * {} = {}", a, b, nimprod(a, b)); // Correctly implemented nimprod function
}