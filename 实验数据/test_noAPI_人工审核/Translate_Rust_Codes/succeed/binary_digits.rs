use std::mem;

fn bin(x: u32) -> String {
    // Modified: Cast the result of the log10 calculation to usize to match the expected type
    let bits = if x == 0 { 1 } else { ((x as f64).log10() / 2.0_f64.log10() + 1.0) as usize };
    let mut ret = String::with_capacity(bits + 1);
    let mut x = x;
    // Modified: Build the string in the correct order directly to avoid unnecessary reversal
    for _ in 0..bits {
        ret.insert(0, if x & 1 == 1 { '1' } else { '0' });
        x >>= 1;
    }
    ret
}

// Modified: Simplified the return type of main to () since no error handling is required
fn main() {
    for i in 0..20 {
        let binstr = bin(i);
        println!("{}", binstr);
    }
}