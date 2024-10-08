fn main() {
    let object: *const char = std::ptr::null();

    if object.is_null() {
        println!("object is null");
    }
}