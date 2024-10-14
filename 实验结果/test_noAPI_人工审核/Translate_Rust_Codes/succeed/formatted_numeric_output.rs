fn main() {
    let r: f32 = 7.125;

    // Modified: Corrected zero-padding for negative numbers
    // Ensured that the negative sign is included in the zero-padding
    println!("{:011.3}", -r); // Adjusted width to 11 to accommodate the negative sign and zero-padding

    // Ensured right-alignment while maintaining zero-padding
    println!("{:09.3}", r);

    // Modified: Corrected alignment for left-aligned numbers without zero-padding
    // Ensured that the left-alignment does not add zero-padding
    println!("{:<9.3}", r);

    // Modified: Corrected zero-padding for negative numbers
    // Ensured that the negative sign is included in the zero-padding
    println!("{:011.3}", -r); // Adjusted width to 11 to accommodate the negative sign and zero-padding

    // Ensured right-alignment while maintaining zero-padding
    println!("{:09.3}", r);

    // Modified: Corrected alignment for left-aligned numbers without zero-padding
    // Ensured that the left-alignment does not add zero-padding
    println!("{:<9.3}", r);
}