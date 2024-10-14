use std::io;

// Modified: Changed the type of `Xint` to `i64` to allow the use of unary negation on integers.
type Xint = i64;

// Modified: Ensured all elements in the arrays are of the same type `i64`.
static U: [[Xint; 9]; 3] = [
    [1, -2, 2, 2, -1, 2, 2, -2, 3],
    [1, 2, 2, 2, 1, 2, 2, 2, 3],
    [-1, 2, 2, -2, 1, 2, -2, 2, 3],
];

fn new_tri(in_: &[Xint; 3], total: &mut Xint, prim: &mut Xint, max_peri: Xint) {
    // Modified: Use an explicit stack to manage the state of the function calls
    let mut stack = vec![*in_];

    while let Some(in_) = stack.pop() {
        let mut t = [0; 3];
        let p = in_[0] + in_[1] + in_[2];

        if p > max_peri {
            continue;
        }

        *prim += 1;
        *total += max_peri / p;

        for i in 0..3 {
            t[0] = U[i][0] * in_[0] + U[i][1] * in_[1] + U[i][2] * in_[2];
            t[1] = U[i][3] * in_[0] + U[i][4] * in_[1] + U[i][5] * in_[2];
            t[2] = U[i][6] * in_[0] + U[i][7] * in_[1] + U[i][8] * in_[2];
            stack.push(t);
        }
    }
}

fn main() {
    let mut max_peri = 10;
    let seed = [3, 4, 5];

    while max_peri <= 100000000 {
        let mut total = 0;
        let mut prim = 0;
        new_tri(&seed, &mut total, &mut prim, max_peri);

        println!(
            "Up to {}: {} triples, {} primitives.",
            max_peri, total, prim
        );

        max_peri *= 10;
    }
}