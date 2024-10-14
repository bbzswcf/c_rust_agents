fn bell_index(row: usize, col: usize) -> usize {
    row * (row - 1) / 2 + col
}

fn get_bell(bell_tri: &[i32], row: usize, col: usize) -> i32 {
    let index = bell_index(row, col);
    bell_tri[index]
}

fn set_bell(bell_tri: &mut [i32], row: usize, col: usize, value: i32) {
    let index = bell_index(row, col);
    bell_tri[index] = value;
}

fn bell_triangle(n: usize) -> Vec<i32> {
    let length = n * (n + 1) / 2;
    let mut tri = vec![0; length];

    set_bell(&mut tri, 1, 0, 1);
    for i in 2..=n {
        // Store the result of get_bell in a temporary variable to avoid mutable borrow conflict
        let temp_value = get_bell(&tri, i - 1, i - 2);
        set_bell(&mut tri, i, 0, temp_value);
        for j in 1..i {
            // Store the results of get_bell calls in temporary variables to avoid mutable borrow conflict
            let value1 = get_bell(&tri, i, j - 1);
            let value2 = get_bell(&tri, i - 1, j - 1);
            let value = value1 + value2;
            set_bell(&mut tri, i, j, value);
        }
    }

    tri
}

fn main() {
    const ROWS: usize = 15;
    let bt = bell_triangle(ROWS);

    println!("First fifteen Bell numbers:");
    for i in 1..=ROWS {
        println!("{:2}: {}", i, get_bell(&bt, i, 0));
    }

    println!("\nThe first ten rows of Bell's triangle:");
    for i in 1..=10 {
        print!("{}", get_bell(&bt, i, 0));
        for j in 1..i {
            print!(", {}", get_bell(&bt, i, j));
        }
        println!();
    }
}