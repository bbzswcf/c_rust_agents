fn main() {
    let r = 7.125;
    println!("{:9.3}", -r);
    println!("{:9.3}", r);
    println!("{:<9.3}", r);
    println!("{:09.3}", -r);
    println!("{:09.3}", r);
    println!("{:<9.3}", r);
}