fn main() {
    let mut count_list: [i32; 3] = [0, 0, 0]; // count_list: [0, 0, 0]

    for i in 1..=20000 {
        let mut sum = 0;
        for j in 1..=i / 2 {
            if i % j == 0 {
                sum += j;
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

    // 修改: 使用单个 println! 语句,确保输出格式与C输出一致
    println!("\nThere are {} deficient, {} perfect, {} abundant numbers between 1 and 20000.\n", count_list[0], count_list[1], count_list[2]);
}