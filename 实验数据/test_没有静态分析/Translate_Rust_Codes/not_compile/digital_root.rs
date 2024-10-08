fn main() {
    // Modified: Changed string type to i32, assuming we need an integer
    let x: i32 = 42;
    println!("x = {}", x);

    let y = 1; // Modified: Ensure the divisor is not zero
    let result = 10 / y;
    println!("Result is: {}", result);

    // Removed: Unused variable z

    let s = String::from("hello");
    println!("{}", s);
    // Modified: Avoid double borrowing by cloning the string
    println!("{}", s.clone());

    let arr = [1, 2, 3, 4, 5];
    // Modified: Ensure the index is within the bounds of the array
    if arr.len() > 10 {
        println!("{}", arr[10]);
    } else {
        println!("Index out of bounds");
    }
}