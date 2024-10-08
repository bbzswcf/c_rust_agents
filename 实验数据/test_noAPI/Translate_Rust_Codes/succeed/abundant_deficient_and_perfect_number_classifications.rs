fn main() {
    const DE: usize = 0;
    const PE: usize = 1;
    const AB: usize = 2;

    let mut sum;
    let mut try_max;
    let mut count_list = [0, 0, 0];

    for i in 1..=20000 {
        // Modified: Calculate try_max based on i - 1 to consider all proper divisors
        try_max = i - 1;
        // Modified: Initialize sum with 0 to exclude the number itself
        sum = 0;

        for j in 1..=try_max {
            if i % j == 0 {
                sum += j;
            }
        }

        // Modified: Correct the condition for deficient numbers
        if sum < i {
            count_list[DE] += 1;
            continue;
        }
        // Modified: Correct the condition for perfect numbers
        if sum == i {
            count_list[PE] += 1;
            continue;
        }
        // Modified: Correct the condition for abundant numbers
        if sum > i {
            count_list[AB] += 1;
            continue;
        }
    }

    println!(
        "\nThere are {} deficient, {} perfect, {} abundant numbers between 1 and 20000.",
        count_list[DE], count_list[PE], count_list[AB]
    );
}