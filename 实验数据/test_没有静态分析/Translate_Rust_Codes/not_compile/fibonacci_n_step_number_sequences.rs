fn anynacci(seed_array: &[i32], how_many: usize) -> Vec<i32> {
    let mut result = Vec::with_capacity(how_many);
    let initial_cardinality = seed_array.len(); // Use the length of the seed array as the initial cardinality

    // Initialize the result vector with the seed array values
    for i in 0..initial_cardinality {
        result.push(seed_array[i]);
    }

    // Generate the sequence up to the desired length
    for i in initial_cardinality..how_many {
        let mut sum = 0;
        // Sum the last `initial_cardinality` values from the `result` vector
        for j in 0..initial_cardinality {
            // Ensure we do not access out-of-bounds indices
            if i >= j {
                sum += result[i - j];
            }
        }
        result.push(sum);
    }

    result
}

fn main() {
    // Corrected seed arrays for Tribonacci, Tetranacci, and Lucas
    let fibo = vec![1, 1];
    let tribo = vec![1, 1, 2];
    let tetra = vec![1, 1, 2, 4];
    let luca = vec![2, 1];

    let fibonacci = anynacci(&fibo, 10);
    let tribonacci = anynacci(&tribo, 10);
    let tetranacci = anynacci(&tetra, 10);
    let lucas = anynacci(&luca, 10);

    // Ensure the header is aligned correctly with the sequence values
    println!("Fibonacci\tTribonacci\tTetranacci\tLucas");

    for i in 0..10 {
        // Adjust the number of tabs or spaces between sequence values to ensure they align correctly with the C output
        println!("{}\t\t{}\t\t{}\t\t{}", fibonacci[i], tribonacci[i], tetranacci[i], lucas[i]);
    }
}