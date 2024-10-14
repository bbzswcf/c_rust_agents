fn droot(x: i64, base: i32, pers: &mut Option<i32>) -> i32 {
    let mut d = 0;
    if let Some(p) = pers {
        *p = 0;
        let mut x = x;
        while x >= base as i64 {
            let mut sum = 0;
            while x > 0 {
                sum += x % base as i64;
                x /= base as i64;
            }
            x = sum;
            *p += 1;
        }
        d = x as i32;
    } else if x > 0 && (d = (x % (base - 1) as i64) as i32) == 0 {
        d = base - 1;
    }
    d
}

fn main() {
    let x = [627615, 39390, 588225, 393900588225];
    for i in 0..4 {
        let mut pers = Some(0);
        let d = droot(x[i], 10, &mut pers);
        println!("{}: pers {:?}, root {}", x[i], pers, d);
    }
}