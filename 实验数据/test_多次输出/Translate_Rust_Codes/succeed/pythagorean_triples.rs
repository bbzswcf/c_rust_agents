fn new_tri(in_arr: &[u64; 3], total: &mut u64, prim: &mut u64, max_peri: u64, u: &[[i64; 9]; 3]) {
    let p = in_arr[0] + in_arr[1] + in_arr[2];

    if p > max_peri {
        return;
    }

    *prim += 1;
    *total += max_peri / p;

    let mut t = [0; 3];
    for i in 0..3 {
        t[0] = (u[i][0] * in_arr[0] as i64 + u[i][1] * in_arr[1] as i64 + u[i][2] * in_arr[2] as i64) as u64;
        t[1] = (u[i][3] * in_arr[0] as i64 + u[i][4] * in_arr[1] as i64 + u[i][5] * in_arr[2] as i64) as u64;
        t[2] = (u[i][6] * in_arr[0] as i64 + u[i][7] * in_arr[1] as i64 + u[i][8] * in_arr[2] as i64) as u64;
        new_tri(&t, total, prim, max_peri, u);
    }
}

fn main() {
    let u = [
        [1, -2, 2, 2, -1, 2, 2, -2, 3],
        [1, 2, 2, 2, 1, 2, 2, 2, 3],
        [-1, 2, 2, -2, 1, 2, -2, 2, 3],
    ];

    let seed = [3, 4, 5];
    let mut max_peri = 10;

    while max_peri <= 100_000_000 {
        let mut total = 0;
        let mut prim = 0;

        new_tri(&seed, &mut total, &mut prim, max_peri, &u);

        println!("Up to {}: {} triples, {} primitives.", max_peri, total, prim);

        max_peri *= 10;
    }
}