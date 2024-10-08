fn bell_index(row: i32, col: i32) -> usize {
    (row * (row - 1) / 2 + col) as usize
}

fn get_bell(bell_tri: &[i32], row: i32, col: i32) -> i32 {
    let index = bell_index(row, col);
    bell_tri[index]
}

fn set_bell(bell_tri: &mut [i32], row: i32, col: i32, value: i32) {
    let index = bell_index(row, col);
    bell_tri[index] = value;
}

fn bell_triangle(n: i32) -> Vec<i32> {
    let length = (n * (n + 1) / 2) as usize;
    let mut tri = vec![0; length];

    set_bell(&mut tri, 1, 0, 1);
    for i in 2..=n {
        // Modified: Store the result of the immutable borrow in a temporary variable
        let temp_value = get_bell(&tri, i - 1, i - 2);
        set_bell(&mut tri, i, 0, temp_value);

        for j in 1..i {
            // Modified: Store the results of the immutable borrows in temporary variables
            let temp_value1 = get_bell(&tri, i, j - 1);
            let temp_value2 = get_bell(&tri, i - 1, j - 1);
            let value = temp_value1 + temp_value2;
            set_bell(&mut tri, i, j, value);
        }
    }

    tri
}

fn main() {
    const ROWS: i32 = 15;
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