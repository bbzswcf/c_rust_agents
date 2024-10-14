fn bin(x: u32) -> String {
    let bits = if x == 0 { 1 } else { (x as f64).log10() / 2_f64.log10() + 1.0 }.ceil() as usize;
    let mut ret = String::with_capacity(bits);
    let mut x = x;
    for _ in 0..bits {
        ret.insert(0, if x & 1 == 1 { '1' } else { '0' });
        x >>= 1;
    }
    ret
}

fn main() {
    for i in 0..20 {
        let binstr = bin(i);
        println!("{}", binstr);
    }
}