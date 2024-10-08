fn main() {
    let mut current = 0; // the current number 
    let mut square; // the square of the current number

    // the strategy of take the rest of division by 1e06 is
    // to take the a number how 6 last digits are 269696
    while {
        square = current * current;
        square % 1000000 != 269696
    } {
        current += 1;
    }

    // output
    // Modified: Removed the unnecessary comparison with i32::MAX
    // since square cannot exceed i32::MAX due to the nature of the calculation
    println!("The smallest number whose square ends in 269696 is {}", current);

    // the end
}