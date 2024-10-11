fn main() {
    let r = 7.125;
    // Modified: Correctly handle negative numbers with zero padding
    println!("{:09.3}", -r);
    println!("{:9.3}", r);
    println!("{:<9.3}", r);
    // Modified: Correctly handle negative numbers with zero padding
    println!("{:09.3}", -r);
    println!("{:09.3}", r);
    // Modified: Correctly align the number to the left without zero padding
    println!("{:<9.3}", r);
}