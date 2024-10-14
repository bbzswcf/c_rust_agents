fn main() {
    let mut current: i32 = 0;
    let mut square: i32;

    while {
        square = current * current;
        // Modified: Removed the always-false condition `square < i32::MAX`
        square % 1000000 != 269696
    } {
        current += 1;
    }

    // Modified: Removed the always-false condition `square > i32::MAX`
    println!("The smallest number whose square ends in 269696 is {}", current);
}