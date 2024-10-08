use std::io;

// Modified: Changed the type of Xint to i64 to allow the use of the unary `-` operator
type Xint = i64;

// Modified: Changed the array elements to match the type i64
static U: [[Xint; 9]; 3] = [
    [1, -2, 2, 2, -1, 2, 2, -2, 3],
    [1, 2, 2, 2, 1, 2, 2, 2, 3],
    [-1, 2, 2, -2, 1, 2, -2, 2, 3],
];

fn new_tri(in_: &[Xint; 3], total: &mut Xint, prim: &mut Xint, max_peri: Xint) {
    let mut stack = vec![*in_];

    while let Some(current) = stack.pop() {
        let p = current[0] + current[1] + current[2];

        if p > max_peri {
            continue;
        }

        *prim += 1;
        *total += max_peri / p;

        for i in 0..3 {
            let mut t = [0; 3];
            t[0] = U[i][0] * current[0] + U[i][1] * current[1] + U[i][2] * current[2];
            t[1] = U[i][3] * current[0] + U[i][4] * current[1] + U[i][5] * current[2];
            t[2] = U[i][6] * current[0] + U[i][7] * current[1] + U[i][8] * current[2];
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

        println!("Up to {}: {} triples, {} primitives.", max_peri, total, prim);

        max_peri *= 10;
    }
}