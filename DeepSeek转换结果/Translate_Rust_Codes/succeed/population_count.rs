fn main() {
    {
        let mut n: u64 = 1;
        for i in 0..30 {
            print!("{} ", n.count_ones());
            n *= 3;
        }
        println!();
    }

    // 修改: 将数组类型改为[u32; 30]以匹配n的类型
    let mut od: [u32; 30] = [0; 30];
    let mut ne: i32 = 0;
    let mut no: i32 = 0;
    print!("evil  : ");
    for n in 0.. {
        // 修改: 将n转换为u32类型以使用count_ones方法
        if ((n as u32).count_ones() & 1) == 0 {
            if ne < 30 {
                print!("{} ", n);
                ne += 1;
            }
        } else {
            if no < 30 {
                // 修改: 将n转换为u32类型以匹配数组类型
                od[no as usize] = n as u32;
                no += 1;
            }
        }
        if ne + no >= 60 {
            break;
        }
    }
    println!();
    print!("odious: ");
    for i in 0..30 {
        print!("{} ", od[i]);
    }
    println!();
}