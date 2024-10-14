fn droot(mut x: i64, base: i32, pers: &mut i32) -> i32 {
    // Modified: Removed unused assignment of `d`
    // let mut d = 0;

    // Modified: Removed `is_some` check since `pers` is a mutable reference to `i32`
    // Instead, directly use the reference without checking for `Option` type
    for _ in 0.. {
        if x < base as i64 {
            break;
        }
        *pers += 1;
        // Modified: Removed unnecessary reassignment of `d`
        // d = 0;
        let mut temp_x = x;
        let mut d = 0; // Added: Initialize `d` here to accumulate the sum
        while temp_x > 0 {
            d += (temp_x % base as i64) as i32;
            temp_x /= base as i64;
        }
        x = d as i64;
    }
    // Modified: Separated the assignment from the condition
    let mut d = (x % (base - 1) as i64) as i32;
    if x != 0 && d == 0 {
        d = base - 1;
    }
    d
}

fn main() {
    let x = [627615, 39390, 588225, 393900588225];
    for i in 0..4 {
        let mut pers = 0;
        let d = droot(x[i], 10, &mut pers);
        println!("{}: pers {}, root {}", x[i], pers, d);
    }
}