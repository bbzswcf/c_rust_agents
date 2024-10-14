fn main() {
    let r = 7.125;

    // Modified: Corrected formatting for negative numbers with zero padding
    // The format specifier `{:011.3}` correctly handles the negative sign by adjusting the width to account for the negative sign.
    println!("{:011.3}", -7.125);

    // Unchanged: Correct format specifiers for positive numbers
    println!("{:09.3}", r);

    // Modified: Corrected alignment for the last print statement
    // The format specifier `{:<9.3}` correctly aligns the number to the left without zero padding.
    println!("{:<9.3}", r);

    // Unchanged: Correct format specifiers for positive numbers
    println!("{:09.3}", r);
    println!("{:09.3}", r);

    // Modified: Corrected alignment for the second-to-last print statement
    // The format specifier `{:<9.3}` correctly aligns the number to the left without zero padding.
    println!("{:<9.3}", r);
}