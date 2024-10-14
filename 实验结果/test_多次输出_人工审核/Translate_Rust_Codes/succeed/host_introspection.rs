fn main() {
    let one: u32 = 1;

    // Best bet: usize typically is exactly one word.
    println!("word size = {} bits", std::mem::size_of::<usize>() * 8);

    // Check if the least significant bit is located in the lowest-address byte.
    let byte = one.to_ne_bytes();
    if byte[0] == 1 {
        println!("little endian");
    } else {
        println!("big endian");
    }
}