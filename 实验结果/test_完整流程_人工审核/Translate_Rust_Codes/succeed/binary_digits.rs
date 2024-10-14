fn bin(x: u32) -> String {
    // Modified: Removed unnecessary import
    // use std::f64;

    // Modified: Ensured the expression inside the `else` branch returns a value of type `usize`
    // and used bitwise operations for efficiency
    let bits = if x == 0 { 1 } else { 32 - x.leading_zeros() as usize };

    let mut ret = vec!['0'; bits];
    let mut x = x;
    for i in 0..bits {
        ret[bits - i - 1] = if x & 1 == 1 { '1' } else { '0' };
        x >>= 1;
    }

    // Modified: Removed the null character `'\0'` as it is not typically used in Rust strings
    // ret.push('\0');

    ret.into_iter().collect()
}

fn main() {
    for i in 0..20 {
        let binstr = bin(i);
        println!("{}", binstr);
    }
}