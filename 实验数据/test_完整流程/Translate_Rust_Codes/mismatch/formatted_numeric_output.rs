fn main() {
    let r = 7.125;
    println!("{:>10.3}", -r); // Modified: Use {:>10.3} for negative numbers to ensure the width includes space for the negative sign
    println!("{:>9.3}", r); // Modified: Use {:>9.3} for positive numbers to ensure the width is correctly calculated
    println!("{:>9.3}", r); // Modified: Use {:>9.3} for positive numbers to ensure the width is correctly calculated
    println!("{:>10.3}", -r); // Modified: Use {:>10.3} for negative numbers to ensure the width includes space for the negative sign
    println!("{:>9.3}", r); // Modified: Use {:>9.3} for positive numbers to ensure the width is correctly calculated
    println!("{:>9.3}", r); // Modified: Use {:>9.3} for positive numbers to ensure the width is correctly calculated
}