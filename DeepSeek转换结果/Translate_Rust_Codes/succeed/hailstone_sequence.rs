fn hailstone(n: i32, mut arry: Option<&mut Vec<i32>>) -> i32 {
    let mut hs = 1;
    let mut current_n = n;

    while current_n != 1 {
        hs += 1;
        if let Some(ref mut arry) = arry {
            arry.push(current_n);
        }
        current_n = if current_n & 1 != 0 { 3 * current_n + 1 } else { current_n / 2 };
    }
    if let Some(ref mut arry) = arry {
        arry.push(current_n);
    }
    hs
}

fn main() {
    let mut hmax = 0;
    let mut jatmax = 0;

    for j in 1..100000 {
        let n = hailstone(j, None);
        if hmax < n {
            hmax = n;
            jatmax = j;
        }
    }

    let n1 = hailstone(27, None);
    let mut arry = Vec::with_capacity(n1 as usize);
    let n2 = hailstone(27, Some(&mut arry));

    print!(
        "[ {}, {}, {}, {}, ...., {}, {}, {}, {}] len={}\n",
        arry[0],
        arry[1],
        arry[2],
        arry[3],
        arry[arry.len() - 4],
        arry[arry.len() - 3],
        arry[arry.len() - 2],
        arry[arry.len() - 1],
        n2
    );
    println!("Max {} at j= {}", hmax, jatmax);
}