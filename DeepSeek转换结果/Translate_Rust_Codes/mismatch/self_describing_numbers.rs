fn self_desc(mut xx: u64) -> bool {
    let mut d: usize = 10; // d  10
    let mut x = xx; // x  xx
    let mut cnt: [u8; 10] = [0; 10];
    let mut dig: [u8; 10] = [0; 10];

    //  cnt  dig 
    while x > 0 && d > 0 {
        d -= 1;
        dig[d] = (x % 10) as u8;
        x /= 10; //  x 
        cnt[dig[d] as usize] += 1;
    }

    //  cnt 
    while d < 10 {
        if dig[d] != cnt[d] {
            return false;
        }
        d += 1;
    }

    true
}

fn main() {
    for i in 1..=10_000_000 {
        if self_desc(i as u64) {
            println!("{}", i);
        }
    }
}