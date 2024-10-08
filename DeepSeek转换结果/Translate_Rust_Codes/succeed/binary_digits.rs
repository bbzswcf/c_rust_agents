fn bin(x: u32) -> String {
    let bits = if x == 0 { 1 } else { (x as f64).log2() as usize + 1 };
    let mut ret = vec!['0'; bits];
    let mut x = x;
    for i in 0..bits {
        ret[bits - i - 1] = if x & 1 == 1 { '1' } else { '0' };
        x >>= 1;
    }
    ret.into_iter().collect()
}

fn main() {
    for i in 0..20 {
        let binstr = bin(i);
        println!("{}", binstr);
    }
}