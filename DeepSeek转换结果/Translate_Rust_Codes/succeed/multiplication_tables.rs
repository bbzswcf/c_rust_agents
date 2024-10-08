fn main() {
    let n: i32 = 12;

    for j in 1..=n {
        print!("{:3}{}", j, if j != n { ' ' } else { '\n' });
    }
    for j in 0..=n {
        print!("{}", if j != n { "----" } else { "+\n" });
    }

    for i in 1..=n {
        for j in 1..=n {
            if j < i {
                print!("    ");
            } else {
                print!("{:3} ", i * j);
            }
        }
        println!("| {}", i);
    }
}