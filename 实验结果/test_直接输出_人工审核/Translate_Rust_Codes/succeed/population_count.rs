fn popcount(mut n: u64) -> u32 {
    let mut count = 0;
    while n != 0 {
        count += n & 1;
        n >>= 1;
    }
    count as u32
}

fn main() {
    let mut n = 1u64;
    for _ in 0..30 {
        print!("{} ", popcount(n));
        n *= 3;
    }
    println!();

    let mut od = [0; 30];
    let mut ne = 0;
    let mut no = 0;
    print!("evil  : ");
    for n in 0.. {
        if ne + no >= 60 {
            break;
        }
        if (popcount(n as u64) & 1) == 0 {
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
    }
    println!();
    print!("odious: ");
    for i in 0..30 {
        print!("{} ", od[i]);
    }
    println!();
}