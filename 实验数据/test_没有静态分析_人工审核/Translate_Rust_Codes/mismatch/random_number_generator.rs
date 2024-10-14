fn evolve(mut state: u64, rule: i32) {
    let n = std::mem::size_of::<u64>() * 8;
    let b = |x: u64| 1u64 << x;

    for _ in 0..10 {
        let mut b_val = 0;
        for q in (0..8).rev() {
            let mut st = state;
            // Correctly calculate the new state based on the rule and current state
            // Ensure the bitwise operations correctly reflect the intended behavior
            b_val |= ((rule as u64) & (1 << (st & 7))) >> (st & 7) << q;

            // Update state correctly based on the rule and current state without resetting it to 0
            for i in 0..n {
                // Correctly calculate the shift amount based on the current state
                let shift_amount = (i as i32 - 1).max(0);
                let shifted_right = st >> shift_amount;
                let shift_amount_left = (n as i32 - i as i32 - 1).max(0);
                let shifted_left = st << shift_amount_left;
                // Correctly evaluate the rule based on the current state and surrounding bits
                if (rule as u64) & (b(7) & (shifted_right | shifted_left)) != 0 {
                    state |= b(i as u64);
                }
            }
        }
        // Output the correct state of the automaton after each iteration
        print!(" {}", b_val);
    }
    println!();
}

fn main() {
    // Ensure that the initial state and rule passed to the evolve function are correct
    evolve(1, 30);
}