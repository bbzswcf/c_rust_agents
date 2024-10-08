fn anynacci(seed_array: &[i32], how_many: usize) -> Vec<i32> {
    let initial_cardinality = seed_array.iter().position(|&x| x == 0).unwrap();
    let mut result = vec![0; how_many];

    for i in 0..initial_cardinality {
        result[i] = seed_array[i];
    }

    for i in initial_cardinality..how_many {
        result[i] = (i - initial_cardinality..i).map(|j| result[j]).sum();
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