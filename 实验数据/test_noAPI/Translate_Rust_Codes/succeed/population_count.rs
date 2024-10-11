fn main() {
    {
        // Modified: Ensure the numeric type is explicitly specified as u64
        let mut n: u64 = 1;
        for _ in 0..30 {
            print!("{} ", (n as u64).count_ones()); // Modified: Explicitly specify type as u64
            n *= 3;
        }
        println!();
    }

    let mut od = [0; 30];
    let mut ne = 0;
    let mut no = 0;
    print!("evil  : ");
    // Modified: Ensure the numeric type is explicitly specified as u64
    let n: u64 = 0;
    for n in 0_u64.. { // Modified: Explicitly specify type as u64
        if ((n as u64).count_ones() & 1) == 0 { // Modified: Explicitly specify type as u64
            if ne < 30 {
                print!("{} ", n);
                ne += 1;
            }
        } else {
            if no < 30 {
                od[no] = n;
                no += 1;
            }
        }
        if ne + no >= 60 {
            break;
        }
    }
    println!();
    print!("odious: ");
    for i in 0..30 {
        print!("{} ", od[i]);
    }
    println!();
}