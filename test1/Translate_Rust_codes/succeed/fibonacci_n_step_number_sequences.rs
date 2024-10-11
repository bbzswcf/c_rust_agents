fn anynacci(seed_array: &[i32], how_many: usize) -> Vec<i32> {
    let mut result = Vec::with_capacity(how_many);
    let mut initial_cardinality = 0;

    for &num in seed_array {
        if num == 0 {
            break;
        }
        initial_cardinality += 1;
    }

    for i in 0..initial_cardinality {
        result.push(seed_array[i]);
    }

    for i in initial_cardinality..how_many {
        let mut sum = 0;
        for j in (i - initial_cardinality)..i {
            sum += result[j];
        }
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

    println!("\nFibonacci\tTribonacci\tTetranacci\tLucas");

    for i in 0..10 {
        println!("\n{}\t\t{}\t\t{}\t\t{}", fibonacci[i], tribonacci[i], tetranacci[i], lucas[i]);
    }
}