fn anynacci(seed_array: &[i32], how_many: usize) -> Vec<i32> {
    let mut result = Vec::with_capacity(how_many);
    let initial_cardinality = seed_array.len();

    for i in 0..initial_cardinality {
        result.push(seed_array[i]);
    }

    for i in initial_cardinality..how_many {
        let sum: i32 = result[i - initial_cardinality..i].iter().sum();
        result.push(sum);
    }

    result
}

fn main() {
    let fibo = vec![1, 1];
    let tribo = vec![1, 1, 2];
    let tetra = vec![1, 1, 2, 4];
    let luca = vec![2, 1];

    let fibonacci = anynacci(&fibo, 10);
    let tribonacci = anynacci(&tribo, 10);
    let tetranacci = anynacci(&tetra, 10);
    let lucas = anynacci(&luca, 10);

    // 修改: 使用 {:<10} 格式化字符串，确保输出与C一致
    println!("{:<10} {:<10} {:<10} {:<10}", "Fibonacci", "Tribonacci", "Tetranacci", "Lucas");

    for i in 0..10 {
        // 修改: 使用 {:<10} 格式化字符串，确保输出与C一致
        println!("{:<10} {:<10} {:<10} {:<10}", fibonacci[i], tribonacci[i], tetranacci[i], lucas[i]);
    }
}