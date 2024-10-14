fn main() {
    let object: Option<&str> = None;

    if object.is_none() {
        println!("object is null");
    }
}