fn anynacci(seed_array: &[i32], how_many: usize) -> Vec<i32> {
    let initial_cardinality = seed_array.iter().position(|&x| x == 0).unwrap_or(seed_array.len());
    let mut result = Vec::with_capacity(how_many);

    result.extend_from_slice(&seed_array[..initial_cardinality]);

    for i in initial_cardinality..how_many {
        let sum: i32 = result[i - initial_cardinality..i].iter().sum();
        result.push(sum);
    }

    result
}

fn main() {
    let fibo = [1, 1, 0];
    let tribo = [1, 1, 2, 0];
    let tetra = [1, 1, 2, 4, 0];
    let luca = [2, 1, 0];

    let fibonacci = anynacci(&fibo, 10);
    let tribonacci = anynacci(&tribo, 10);
    let tetranacci = anynacci(&tetra, 10);
    let lucas = anynacci(&luca, 10);

    println!("Fibonacci\tTribonacci\tTetranacci\tLucas");

    for i in 0..10 {
        println!("{}\t\t{}\t\t{}\t\t{}", fibonacci[i], tribonacci[i], tetranacci[i], lucas[i]);
    }
}