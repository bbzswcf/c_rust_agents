use std::io;

const TRANS: &str = "___#_##_";

fn evolve(cell: &mut [char], backup: &mut [char], len: usize) -> usize {
    let mut diff = 0;

    // Precompute all necessary indices and values before borrowing `cell` as mutable
    let indices: Vec<usize> = (0..len).map(|i| {
        (if v(cell, i as isize - 1) { 1 } else { 0 }) * 4
        + (if v(cell, i as isize) { 1 } else { 0 }) * 2
        + (if v(cell, i as isize + 1) { 1 } else { 0 })
    }).collect();

    for i in 0..len {
        // Ensure `index` does not exceed the bounds of the `TRANS` string
        let index = indices[i];
        if index < TRANS.len() {
            backup[i] = TRANS.chars().nth(index).unwrap();
        } else {
            backup[i] = '_'; // Safe fallback if index is out of bounds
        }
        if backup[i] != cell[i] {
            diff += 1;
        }
    }

    cell.copy_from_slice(backup);
    diff
}

fn v(cell: &[char], i: isize) -> bool {
    let len = cell.len() as isize;
    let index = (i + len) % len;
    cell[index as usize] != '_'
}

fn main() {
    let mut c = "_###_##_#_#_#_#__#__\n".chars().collect::<Vec<char>>();
    let mut b = "____________________\n".chars().collect::<Vec<char>>();

    // Ensure the slice lengths passed to the `evolve` function match the expected lengths
    while evolve(&mut c[1..c.len() - 2], &mut b[1..b.len() - 2], c.len() - 3) != 0 {
        println!("{}", c.iter().skip(1).collect::<String>());
    }
}