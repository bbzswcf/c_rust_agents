fn main() {
    let mut current: i32 = 0; // the current number 
    let mut square: i32; // the square of the current number

    // the strategy of taking the remainder of division by 1e06 is
    // to find a number whose last 6 digits are 269696
    while {
        square = current * current;
        square % 1000000 != 269696 && square < i32::MAX
    } {
        current += 1;
    }

    // output
    if square == i32::MAX {
        println!("Condition not satisfied before INT_MAX reached.");
    } else {
        println!("The smallest number whose square ends in 269696 is {}", current);
    }
}