fn bin(x: u32) -> String {
    // Modified: Corrected the calculation of the number of bits using `leading_zeros`
    let bits = if x == 0 { 1 } else { 32 - x.leading_zeros() as usize };
    
    // Modified: Corrected the vector initialization and loop to fill from the least significant bit
    let mut ret = vec!['0'; bits];
    let mut x = x;
    for i in 0..bits {
        ret[bits - i - 1] = if x & 1 == 1 { '1' } else { '0' };
        x >>= 1;
    }
    
    // Removed: Unnecessary null terminator
    // ret.push('\0');
    
    // Modified: Corrected the `collect` method to collect characters into a `String`
    ret.into_iter().collect()
}

fn main() {
    for i in 0..20 {
        let binstr = bin(i);
        println!("{}", binstr);
    }
}