fn droot(mut x: i64, base: i32) -> (i32, i32) {
    // Modified: Removed unnecessary Option for `pers` and used a simple integer
    let mut pers = 0;
    let mut d = 0;

    while x >= base as i64 {
        let mut temp_x = x;
        d = 0;
        while temp_x > 0 {
            // Modified: Simplified the expression to avoid unnecessary casting
            d += (temp_x % base as i64) as i32;
            temp_x /= base as i64;
        }
        x = d as i64;
        pers += 1; // Modified: Increment persistence each time the sum of digits is performed
    }

    // Modified: Corrected the digital root calculation by summing digits until a single digit is obtained
    while d >= base {
        let mut temp_d = d;
        d = 0;
        while temp_d > 0 {
            d += temp_d % base;
            temp_d /= base;
        }
    }

    (pers, d)
}

fn main() {
    let x = [627615, 39390, 588225, 393900588225];
    for i in 0..4 {
        let (pers, d) = droot(x[i], 10);
        println!("{}: pers {}, root {}", x[i], pers, d);
    }
}