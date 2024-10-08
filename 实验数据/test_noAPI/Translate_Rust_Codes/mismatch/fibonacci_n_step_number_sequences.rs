fn anynacci(seed_array: &[i32], how_many: usize) -> Vec<i32> {
    let initial_cardinality = seed_array.len();
    let mut result = Vec::with_capacity(how_many); // Ensure `how_many` is a reasonable upper bound

    // Initialize the sequence with the seed values
    for &value in seed_array {
        result.push(value);
    }

    // Generate the sequence based on the initial cardinality
    for i in initial_cardinality..how_many {
        let mut next_value = 0;
        // Modified: Ensure that the index `i - initial_cardinality + j` does not go out of bounds
        for j in 0..initial_cardinality {
            next_value += result[i - initial_cardinality + j];
        }
        result.push(next_value);
    }

    result
}

fn main() {
    let fibo = [1, 1];
    let tribo = [1, 1, 2];
    let tetra = [1, 1, 2, 4];
    let luca = [2, 1];

    let fibonacci = anynacci(&fibo, 10);
    let tribonacci = anynacci(&tribo, 10);
    let tetranacci = anynacci(&tetra, 10);
    let lucas = anynacci(&luca, 10);

    println!("\nFibonacci\tTribonacci\tTetranacci\tLucas");

    for i in 0..10 {
        // Modified: Use fixed-width formatting to ensure proper alignment
        println!("{:10}\t{:10}\t{:10}\t{:10}", fibonacci[i], tribonacci[i], tetranacci[i], lucas[i]);
    }
}