fn main() {
    let one: i32 = 1;

    println!("word size = {} bits", std::mem::size_of::<usize>() * 8);

    // Modified: Wrapped the dereference of the raw pointer in an `unsafe` block
    // to indicate awareness of the potential risks and undefined behavior
    unsafe {
        if *(&one as *const i32 as *const u8) == 1 {
            println!("little endian");
        } else {
            println!("big endian");
        }
    }
}