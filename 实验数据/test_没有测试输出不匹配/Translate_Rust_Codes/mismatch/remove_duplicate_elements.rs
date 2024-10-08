fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    // Ensure the function `plus_one` is called with an argument of type `i32`
    let result = plus_one(5);
    println!("Result of plus_one: {}", result);

    // Ensure the condition in the `if` statement is of type `bool`
    if true {
        println!("Condition is true");
    }

    // Use the constant directly from the standard library to ensure accuracy
    let x: f32 = std::f32::consts::PI;
    println!("Value of x: {}", x);

    let a = vec![1, 2, 1, 4, 5, 2, 15, 1, 3, 4];
    let (b, n) = nub_new(&a);

    // Use an iterator instead of a range loop
    for item in b.iter().take(n) {
        print!("{} ", item);
    }

    // Remove the empty string literal and use println!() for a newline
    println!();
}

fn elem(a: &[i32], e: i32) -> bool {
    for &item in a {
        if item == e {
            return true;
        }
    }
    false
}

// Change the parameter type to a mutable slice to avoid unnecessary allocation
fn nub(a: &mut [i32]) -> usize {
    let mut m = 0;
    for i in 0..a.len() {
        if !elem(&a[0..m], a[i]) {
            a[m] = a[i];
            m += 1;
        }
    }
    m
}

fn nub_new(a: &[i32]) -> (Vec<i32>, usize) {
    // Initialize vector `c` using safe alternatives
    let mut c = Vec::with_capacity(a.len());
    c.extend_from_slice(a);
    let m = nub(&mut c);

    // Initialize vector `b` using safe alternatives
    let mut b = Vec::with_capacity(m);
    b.extend_from_slice(&c[0..m]);
    (b, m)
}