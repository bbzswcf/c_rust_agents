fn evolve(state: u64, rule: i32) {
    let mut state = state;
    // 初始化 b 为 0
    let mut b: u8 = 0;
    // 初始化 st 为 0
    let mut st: u64 = 0;

    for _p in 0..10 {
        b = 0;
        for q in (0..8).rev() {
            st = state;
            // 修改: 确保 q 在 u8 的范围内 (0 到 7)
            if q < 8 {
                b |= ((st & 1) << q) as u8;
            }

            state = 0;
            // 修改: 确保 N 在 evolve 函数中定义
            for i in 1..=N {
                // 修改: 确保 state 在每次迭代中正确更新
                if rule & (1 << (7 & ((st >> (i - 1)) | (st << (N + 1 - i))))) != 0 {
                    state |= 1 << (i - 1);
                }
            }
        }
        print!(" {}", b);
    }
    print!("\n");
}

fn main() {
    evolve(1, 30);
}

// 修改: 定义 N 为 64
const N: i32 = 64;
// 定义 B 函数
const fn B(x: i32) -> u64 {
    1u64 << x
}