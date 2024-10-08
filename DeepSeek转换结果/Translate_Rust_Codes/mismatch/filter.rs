fn even_sel(x: i32) -> bool { x % 2 == 0 } // 偶数选择器
fn tri_sel(x: i32) -> bool { x % 3 != 0 } // 非3的倍数选择器

fn grep(in_vec: &[i32], outlen: &mut usize, sel: fn(i32) -> bool, inplace: bool) -> Vec<i32> {
    let mut out: Vec<i32> = if inplace {
        Vec::with_capacity(in_vec.len())
    } else {
        Vec::new() // 修改: 使用 Vec::new() 而不是 Vec::with_capacity(in_vec.len())
    };

    for &item in in_vec {
        if sel(item) {
            out.push(item);
        }
    }

    if !inplace {
        out.resize(out.len(), 0);
    }

    *outlen = out.len(); // 修改: 更新 outlen
    out
}

fn main() {
    let mut in_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut len = 0;

    let even = grep(&in_vec, &mut len, even_sel, false);
    print!("Filtered even:");
    // 修改: 使用 take(len) 而不是直接迭代
    for item in even.iter().take(len) {
        print!(" {}", item);
    }
    println!(); // 修改: 添加换行符

    // 修改: 重置 len 为 0, 以便在调用 grep 时重新计算
    len = 0;
    let tri = grep(&in_vec, &mut len, tri_sel, false);
    print!("Filtered not multiple of 3:");
    // 修改: 使用 take(len) 而不是直接迭代
    for item in tri.iter().take(len) {
        print!(" {}", item);
    }
    println!(); // 修改: 添加换行符
}