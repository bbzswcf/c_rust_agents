fn droot(x: i64, base: i32, pers: &mut Option<i32>) -> i32 {
    let mut d = 0;
    if let Some(p) = pers {
        *p = 0;
        while x >= base as i64 {
            let mut temp_x = x;
            d = 0;
            while temp_x > 0 {
                d += (temp_x % base as i64) as i32;
                temp_x /= base as i64;
            }
            x = d as i64;
            *p += 1;
        }
    } else if x > 0 && (d = (x % (base - 1) as i64) as i32) == 0 {
        d = base - 1;
    }
    d
}

fn main() {
    let x = [627615, 39390, 588225, 393900588225];
    for &num in x.iter() {
        let mut pers = None;
        let d = droot(num, 10, &mut pers);
        println!("{}: pers {:?}, root {}", num, pers, d);
    }
}