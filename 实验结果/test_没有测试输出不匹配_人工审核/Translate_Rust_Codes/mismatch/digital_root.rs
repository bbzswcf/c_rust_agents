fn droot(mut x: i64, base: i32, pers: &mut i32) -> i32 {
    // Removed: Unused assignment warning by removing the unused variable `d`
    *pers = 0; // Initialize `pers` with a default value before the function call
    while x >= base as i64 { // Ensure `base` is cast to `i64` to match the type of `x`
        let mut temp_x = x;
        let mut d = 0; // Reintroduced `d` as it is needed for calculations
        while temp_x > 0 {
            d += temp_x % base as i64; // Ensure `temp_x` and `d` are of type `i64`
            temp_x /= base as i64; // Ensure `temp_x` and `base` are cast to `i64` to match the type of `temp_x`
        }
        x = d;
        *pers += 1;
    }
    let d = x % (base as i64 - 1); // Separate the assignment from the condition
    if x > 0 && d == 0 { // Ensure the condition is checked correctly
        d as i32 // Ensure the cast `d as i32` is necessary and correct
    } else {
        d as i32
    }
}

fn main() {
    let x: [i64; 4] = [627615, 39390, 588225, 393900588225]; // Ensure the array elements are of type `i64`
    for i in 0..4 {
        let mut pers = 0;
        let d = droot(x[i], 10, &mut pers); // Ensure the function call arguments match the expected types
        println!("{}: pers {}, root {}", x[i], pers, d); // Ensure the types used in the `println!` macro are compatible
    }
}