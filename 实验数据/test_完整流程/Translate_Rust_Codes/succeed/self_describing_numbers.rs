fn self_desc(xx: u64) -> bool {
    let mut cnt = [0u8; 10];
    let mut dig = [0u8; 10];
    let mut d = 0;

    let mut xx = xx;
    while xx > u32::MAX as u64 {
        let digit = (xx % 10) as u8;
        cnt[digit as usize] += 1;
        dig[d] = digit;
        d += 1;
        xx /= 10;
    }

    let mut x = xx as u32;
    while x > 0 {
        let digit = (x % 10) as u8;
        cnt[digit as usize] += 1;
        dig[d] = digit;
        d += 1;
        x /= 10;
    }

    let mut x = 0;
    while d > 0 && dig[x] == cnt[d - 1] {
        x += 1;
        d -= 1;
    }

    d == 0
}

fn main() {
    for i in 1..10_000_000 {
        if self_desc(i) {
            println!("{}", i);
        }
    }
}