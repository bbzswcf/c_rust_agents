fn new_tri(in_: &[i64; 3], max_peri: &mut u64, total: &mut u64, prim: &mut u64, U: &[[i64; 9]; 3]) {
    let mut t = [0i64; 3];
    // 计算周长
    let p = in_[0] + in_[1] + in_[2];

    if p > *max_peri as i64 {
        return;
    }

    // 增加原始三角形的计数
    *prim += 1;

    // 使用 checked_add 防止溢出
    if let Some(new_total) = total.checked_add((*max_peri / p as u64) as u64) {
        *total = new_total;
    } else {
        // 处理溢出情况
        eprintln!("Overflow detected in total calculation");
        return;
    }

    for i in 0..3 {
        // 计算新的三角形
        t[0] = U[i][0] * in_[0] + U[i][1] * in_[1] + U[i][2] * in_[2];
        t[1] = U[i][3] * in_[0] + U[i][4] * in_[1] + U[i][5] * in_[2];
        t[2] = U[i][6] * in_[0] + U[i][7] * in_[1] + U[i][8] * in_[2];
        // 递归调用 new_tri
        new_tri(&t, max_peri, total, prim, U);
    }
}

// 定义 U 矩阵
const U: [[i64; 9]; 3] = [
    [1, -2, 2, 2, -1, 2, 2, -2, 3],
    [1, 2, 2, 2, 1, 2, 2, 2, 3],
    [-1, 2, 2, -2, 1, 2, -2, 2, 3],
];

fn main() {
    let seed = [3i64, 4i64, 5i64];
    let mut max_peri: u64 = 10; // 最大周长
    let mut total: u64 = 0; // 总三角形数
    let mut prim: u64 = 0; // 原始三角形数

    for _ in 0..8 {
        max_peri *= 10;
        total = 0;
        prim = 0;
        // 修改: 传递 U 的引用
        new_tri(&seed, &mut max_peri, &mut total, &mut prim, &U);

        print!("Up to {}: {} triples, {} primitives.\n", max_peri, total, prim);
    }
}