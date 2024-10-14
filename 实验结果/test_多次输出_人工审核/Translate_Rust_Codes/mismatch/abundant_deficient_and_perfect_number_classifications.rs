fn main() {
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
            count_list[0] += 1;
        } else if sum > i {
            count_list[2] += 1;
        } else {
            count_list[1] += 1;
        }
    }

    println!(
        "\nThere are {} deficient, {} perfect, {} abundant numbers between 1 and 20000.",
        count_list[0], count_list[1], count_list[2]
    );
}