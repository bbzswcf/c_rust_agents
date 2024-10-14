fn main() {
    let previous: u64 = 1234;
    let gap: u64 = 678998;
    let mut sum: u64 = 0;
    let mut niven_index: i32 = 56778;
    let mut gap_index: i32 = 1000;

    println!("Gap index  Gap    Niven index    Niven number");
    println!("{:9} {:4} {:14} {:15}", gap_index, gap, niven_index, previous);
    gap_index += 1;
}