fn main() {
    let mut sum;
    let mut try_max;
    let mut count_list = [1, 0, 0];

    for i in 2..=20000 {
        try_max = (i as f64).sqrt() as u32; // Initialize try_max to the square root of i for efficiency
        sum = 1;

        for j in 2..=try_max { // Loop up to the square root of i to find all divisors
            if i % j != 0 {
                continue;
            }
            sum += j + (i / j); // Add both divisors in the pair to the sum
            if j * j == i { // Correctly identify perfect squares
                sum -= j; // Adjust the sum to avoid double counting the square root
            }
        }

        if sum < i {
            count_list[0] += 1;
            continue;
        }
        if sum > i {
            count_list[2] += 1;
            continue;
        }
        count_list[1] += 1;
    }

    // Modified: Combined print statements to print all counts on a single line
    println!("\nThere are {} deficient, {} perfect, {} abundant numbers between 1 and 20000.\n", count_list[0], count_list[1], count_list[2]);
}