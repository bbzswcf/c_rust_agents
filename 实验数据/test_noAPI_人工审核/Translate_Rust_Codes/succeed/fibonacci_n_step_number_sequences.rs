use std::alloc::{alloc, Layout};
use std::mem;
use std::ptr;

fn anynacci(seed_array: &[i32], how_many: usize) -> Result<Vec<i32>, &'static str> {
    let initial_cardinality = seed_array.iter().position(|&x| x == 0).ok_or("Seed array must be 0-ended")?;

    let mut result = Vec::with_capacity(how_many);
    result.extend_from_slice(&seed_array[..initial_cardinality]);

    for i in initial_cardinality..how_many {
        let mut sum = 0;
        for j in (i - initial_cardinality)..i {
            sum += result[j];
        }
        result.push(sum);
    }

    Ok(result)
}

fn main() -> Result<(), &'static str> {
    let fibo = [1, 1, 0];
    let tribo = [1, 1, 2, 0];
    let tetra = [1, 1, 2, 4, 0];
    let luca = [2, 1, 0];

    let fibonacci = anynacci(&fibo, 10)?;
    let tribonacci = anynacci(&tribo, 10)?;
    let tetranacci = anynacci(&tetra, 10)?;
    let lucas = anynacci(&luca, 10)?;

    println!("\nFibonacci\tTribonacci\tTetranacci\tLucas");

    for i in 0..10 {
        println!("\n{}\t\t{}\t\t{}\t\t{}", fibonacci[i], tribonacci[i], tetranacci[i], lucas[i]);
    }

    Ok(())
}