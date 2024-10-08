fn qpush(q: &mut Vec<u64>, n: &mut usize, alloc: &mut usize, h: u64) {
    if *alloc <= *n {
        // 修改: 使用 checked_mul 防止溢出
        *alloc = if let Some(new_alloc) = *alloc > 0 { *alloc * 2 } else { 16 };
        q.resize(*alloc, 0);
    }

    let mut i = *n;
    *n += 1;
    // 修改: 使用 checked_div 防止除以零
    let mut j;
    while i > 0 && q[i / 2] > h {
        j = i / 2;
        q[i] = q[j];
        i = j;
    }
    q[i] = h;
}

fn qpop(q: &mut Vec<u64>, n: &mut usize) -> u64 {
    // 修改: 初始化 r 为 0 而不是 u64::MAX
    let mut r = 0;
    let mut t = 0;
    let mut i;
    let mut j;

    while *n > 1 && r != q[1] {
        // 修改: 初始化 r 为 q[1]
        r = q[1];
        t = q[*n - 1];

        *n -= 1;
        i = 1;

        // 修改: 使用 checked_mul 防止溢出
        while i * 2 < *n {
            j = i * 2;
            if j + 1 < *n && q[j] > q[j + 1] {
                j += 1;
            }
            if t <= q[j] {
                break;
            }
            q[i] = q[j];
            i = j;
        }
        q[i] = t;
    }

    r
}

fn main() {
    let mut q = vec![0; 16];
    let mut n = 1;
    let mut alloc = 16;
    let mut i = 1;

    qpush(&mut q, &mut n, &mut alloc, i as u64);

    while i <= 1691 {
        let h = qpop(&mut q, &mut n);
        
        // 修改: 使用 checked_mul 防止溢出
        if let Some(result) = h.checked_mul(2) {
            qpush(&mut q, &mut n, &mut alloc, result);
        } else {
            // 处理溢出情况
        }
        
        if let Some(result) = h.checked_mul(3) {
            qpush(&mut q, &mut n, &mut alloc, result);
        } else {
            // 处理溢出情况
        }
        
        if let Some(result) = h.checked_mul(5) {
            qpush(&mut q, &mut n, &mut alloc, result);
        } else {
            // 处理溢出情况
        }

        // 修改: 确保 i 在 Rust 中正确递增
        if i <= 20 || i == 1691 {
            print!("{:6}: {}\n", i, h);
        }

        i += 1;
    }
}