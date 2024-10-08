fn main() {
    let mut current = 0; // the current number 
    let mut square; // the square of the current number

    // the strategy of take the rest of division by 1e06 is
    // to take the a number how 6 last digits are 269696
    while {
        square = current * current;
        square % 1000000 != 269696 && square < i32::MAX // Modified: Replaced std::i32::MAX with i32::MAX for better readability
    } {
        current += 1;
    }

    // output
    if square == i32::MAX { // Modified: Changed comparison to check if square has reached the maximum value
        println!("Condition not satisfied before INT_MAX reached.");
    } else {
        println!("The smallest number whose square ends in 269696 is {}", current);
    }

    // the end
}