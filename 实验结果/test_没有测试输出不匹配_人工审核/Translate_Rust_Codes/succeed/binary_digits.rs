use std::f64;

fn bin(x: u32) -> String {
    let bits = if x == 0 { 1 } else { (f64::log10(x as f64) / f64::log10(2.0)).ceil() as usize + 1 };
    let mut ret = vec!['0'; bits];
    let mut x = x;
    for i in 0..bits {
        ret[bits - i - 1] = if x & 1 == 1 { '1' } else { '0' };
        x >>= 1;
    }
    ret.push('\0');
    ret.into_iter().collect()
}

fn main() {
    for i in 0..20 {
        let binstr = bin(i);
        println!("{}", binstr);
    }
}