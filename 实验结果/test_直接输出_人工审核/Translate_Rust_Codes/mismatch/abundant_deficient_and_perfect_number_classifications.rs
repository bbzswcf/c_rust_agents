fn main() {
    const DE: usize = 0;
    const PE: usize = 1;
    const AB: usize = 2;

    let mut sum;
    let mut try_max;
    let mut count_list = [1, 0, 0];

    for i in 2..=20000 {
        try_max = i / 2;
        sum = 1;

        for j in 2..try_max {
            if i % j != 0 {
                continue;
            }
            try_max = i / j;
            sum += j;
            if j != try_max {
                sum += try_max;
            }
        }

        if sum < i {
            count_list[DE] += 1;
            continue;
        }
        if sum > i {
            count_list[AB] += 1;
            continue;
        }
        count_list[PE] += 1;
    }

    println!("\nThere are {} deficient,", count_list[DE]);
    println!(" {} perfect,", count_list[PE]);
    println!(" {} abundant numbers between 1 and 20000.\n", count_list[AB]);
}