fn main() {
    const DE: usize = 0;
    const PE: usize = 1;
    const AB: usize = 2;

    let mut sum;
    let mut count_list = [1, 0, 0]; // Modified: Initialize count_list with 1 for deficient numbers (1 is deficient)

    for i in 2..=20000 {
        sum = 1; // Modified: Initialize sum with 1 because 1 is a divisor of every number

        // Modified: Iterate up to the square root of i to efficiently find all divisors
        for j in 2..=(i as f64).sqrt() as usize {
            if i % j == 0 { // Modified: Correct condition to include the divisor in the sum
                sum += j;
                let try_max = i / j; // Modified: Calculate try_max only when j is a divisor
                if j != try_max {
                    sum += try_max;
                }
            }
        }

        if sum < i {
            count_list[DE] += 1;
        } else if sum > i {
            count_list[AB] += 1;
        } else {
            count_list[PE] += 1;
        }
    }

    // Modified: Combine the print statements into a single `println!` call to match the C output format
    println!("\nThere are {} deficient, {} perfect, {} abundant numbers between 1 and 20000.\n", count_list[DE], count_list[PE], count_list[AB]);
}