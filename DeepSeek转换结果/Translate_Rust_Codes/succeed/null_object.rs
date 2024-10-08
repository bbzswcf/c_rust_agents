fn main() {
    let object: *const i8 = std::ptr::null();

    if object.is_null() {
        println!("object is null");
    }
}