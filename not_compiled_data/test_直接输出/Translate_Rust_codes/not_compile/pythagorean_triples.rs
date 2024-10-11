use std::io;

type Xint = u64;

static U: [[Xint; 9]; 3] = [
    [1, -2, 2, 2, -1, 2, 2, -2, 3],
    [1, 2, 2, 2, 1, 2, 2, 2, 3],
    [-1, 2, 2, -2, 1, 2, -2, 2, 3],
];

static mut TOTAL: Xint = 0;
static mut PRIM: Xint = 0;
static mut MAX_PERI: Xint = 0;

fn new_tri(in_: &[Xint; 3]) {
    let mut t = [0; 3];
    let p = in_[0] + in_[1] + in_[2];

    unsafe {
        if p > MAX_PERI {
            return;
        }

        PRIM += 1;
        TOTAL += MAX_PERI / p;

        for i in 0..3 {
            t[0] = U[i][0] * in_[0] + U[i][1] * in_[1] + U[i][2] * in_[2];
            t[1] = U[i][3] * in_[0] + U[i][4] * in_[1] + U[i][5] * in_[2];
            t[2] = U[i][6] * in_[0] + U[i][7] * in_[1] + U[i][8] * in_[2];
            new_tri(&t);
        }
    }
}

fn main() {
    let seed = [3, 4, 5];

    unsafe {
        for _ in 0..8 {
            MAX_PERI *= 10;
            TOTAL = 0;
            PRIM = 0;
            new_tri(&seed);

            println!(
                "Up to {}: {} triples, {} primitives.",
                MAX_PERI, TOTAL, PRIM
            );
        }
    }
}