fn main() {
    let mut current: i32 = 0; // the current number 
    let mut square: i32; // the square of the current number

    // the strategy of take the rest of division by 1e06 is
    // to take the a number how 6 last digits are 269696
    while {
        square = current * current;
        square % 1000000 != 269696 && square < std::i32::MAX
    } {
        current += 1;
    }

    // output
    if square > std::i32::MAX {
        println!("Condition not satisfied before INT_MAX reached.");
    } else {
        println!("The smallest number whose square ends in 269696 is {}", current);
    }

    // the end
}