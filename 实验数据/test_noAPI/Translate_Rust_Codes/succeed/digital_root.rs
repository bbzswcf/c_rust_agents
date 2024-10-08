fn droot(x: i64, base: i32, pers: Option<&mut i32>) -> i32 {
    // Ensure base is not zero to avoid division by zero
    if base == 0 {
        panic!("Base cannot be zero");
    }

    let mut d = 0;
    if let Some(p) = pers {
        *p = 0;
        let mut x = x;
        while x >= base as i64 {
            let mut sum = 0;
            while x > 0 {
                // Ensure no overflow by casting base to i64
                sum += x % base as i64;
                x /= base as i64;
            }
            // Ensure sum is within valid range for i32
            d = sum as i32;
            x = d as i64;
            *p += 1;
        }
    } else if x > 0 {
        // Separate assignment and comparison to avoid logic error
        d = (x % (base - 1) as i64) as i32;
        if d == 0 {
            d = base - 1;
        }
    }
    d
}

fn main() {
    let x: [i64; 4] = [627615, 39390, 588225, 393900588225];
    for i in 0..4 {
        let mut pers = 0;
        let d = droot(x[i], 10, Some(&mut pers));
        println!("{}: pers {}, root {}", x[i], pers, d);
    }
}