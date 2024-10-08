fn main() {
    let one = 1;

    println!("word size = {} bits", std::mem::size_of::<usize>() * 8);

    // Modified: Wrapped the unsafe operation in an `unsafe` block
    unsafe {
        if *(&one as *const i32 as *const u8) == 1 {
            println!("little endian");
        } else {
            println!("big endian");
        }
    }
}