fn main() {
    let r = 7.125;
    // Ensure the negative sign is aligned correctly
    println!("{:>9.3}", -r);
    // Ensure the positive number is aligned correctly
    println!("{:>9.3}", r);
    // Ensure the alignment is correct
    println!("{:<9.3}", r);
    // Ensure the leading zeros are correctly formatted for negative numbers
    println!("{:09.3}", -r);
    // Ensure the leading zeros are correctly formatted for positive numbers
    println!("{:09.3}", r);
    // Ensure the leading zeros and alignment are correctly formatted for positive numbers
    println!("{:>09.3}", r);
}