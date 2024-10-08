use std::env;

fn digit_sum(n: u64, sum: u64) -> u64 {
    let mut sum = sum + 1;
    let mut n = n;
    while n > 0 && n % 10 == 0 {
        sum -= 9;
        n /= 10;
    }
    sum
}

fn divisible(n: u64, d: u64) -> bool {
    if (d & 1) == 0 && (n & 1) == 1 {
        return false;
    }
    n % d == 0
}

fn main() {
    env::set_var("LC_ALL", "");

    let mut previous = 1u64;
    let mut gap = 0u64;
    let mut sum = 0u64;
    let mut niven_index = 0;
    let mut gap_index = 1;

    print!("Gap index  Gap    Niven index    Niven number\n");
    for niven in 1u64.. {
        sum = digit_sum(niven, sum);
        if divisible(niven, sum) {
            if niven > previous + gap {
                gap = niven - previous;
                // 修改: 确保格式化字符串与C输出一致
                print!("{} {} {} {}\n", format!("{:>9}", gap_index), format!("{:>4}", gap), format!("{:>15}", niven_index), format!("{:>14}", previous.to_string().as_str()));
                gap_index += 1;
            }
            previous = niven;
            niven_index += 1;
        }
        if gap_index > 20 {
            break;
        }
    }
}