// Modified: Changed Xint type to i64 to allow negative values
type Xint = i64;

// Modified: Changed array elements to match the i64 type
static U: [[Xint; 9]; 3] = [
    [1, -2, 2, 2, -1, 2, 2, -2, 3],
    [1, 2, 2, 2, 1, 2, 2, 2, 3],
    [-1, 2, 2, -2, 1, 2, -2, 2, 3],
];

fn new_tri(in_: &[Xint; 3], max_peri: Xint) -> (Xint, Xint) {
    let mut total = 0;
    let mut prim = 0;
    let mut stack = vec![*in_];

    while let Some(t) = stack.pop() {
        let p = t[0] + t[1] + t[2];

        if p > max_peri {
            continue;
        }

        prim += 1;
        total += max_peri / p;

        for i in 0..3 {
            let new_t = [
                U[i][0] * t[0] + U[i][1] * t[1] + U[i][2] * t[2],
                U[i][3] * t[0] + U[i][4] * t[1] + U[i][5] * t[2],
                U[i][6] * t[0] + U[i][7] * t[1] + U[i][8] * t[2],
            ];
            stack.push(new_t);
        }
    }

    (total, prim)
}

// Modified: Changed return type to () since main does not return any errors
fn main() {
    let seed = [3, 4, 5];
    let mut max_peri = 10;

    while max_peri <= 100000000 {
        let (total, prim) = new_tri(&seed, max_peri);

        println!(
            "Up to {}: {} triples, {} primitives.",
            max_peri, total, prim
        );

        // Modified: Ensure max_peri does not overflow by using checked multiplication
        if let Some(new_max_peri) = max_peri.checked_mul(10) {
            max_peri = new_max_peri;
        } else {
            break;
        }
    }
}