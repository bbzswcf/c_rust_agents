use std::io;

fn anynacci(seed_array: &[i32], how_many: usize) -> Vec<i32> {
    let mut result = Vec::with_capacity(how_many);
    let initial_cardinality = seed_array.len(); // Set initial_cardinality to the length of the seed array

    for i in 0..initial_cardinality {
        result.push(seed_array[i]);
    }

    for i in initial_cardinality..how_many {
        let mut sum = 0;
        for j in (i - initial_cardinality)..i { // Sum the previous elements from i - initial_cardinality to i - 1
            sum += result[j];
        }
        result.push(sum);
    }

    result
}

fn main() {
    let fibonacci_seed = [1, 1]; // Corrected: Fibonacci sequence starts with [1, 1]
    let tribonacci_seed = [1, 1, 1]; // Corrected: Tribonacci sequence starts with [1, 1, 1]
    let tetranacci_seed = [1, 1, 1, 1]; // Corrected: Tetranacci sequence starts with [1, 1, 1, 1]
    let lucas_seed = [2, 1]; // Corrected: Lucas sequence starts with [2, 1]

    let fibonacci = anynacci(&fibonacci_seed, 10);
    let tribonacci = anynacci(&tribonacci_seed, 10);
    let tetranacci = anynacci(&tetranacci_seed, 10);
    let lucas = anynacci(&lucas_seed, 10);

    println!("\nFibonacci\tTribonacci\tTetranacci\tLucas");

    for i in 0..10 {
        println!("{}\t\t{}\t\t{}\t\t{}", fibonacci[i], tribonacci[i], tetranacci[i], lucas[i]); // Corrected: Removed \n within println!
    }
}