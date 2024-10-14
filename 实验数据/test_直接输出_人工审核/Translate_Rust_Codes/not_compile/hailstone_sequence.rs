fn hailstone(n: i32, arry: Option<&mut Vec<i32>>) -> i32 {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut hmax = 0;
    let mut jatmax = 0;
    let mut n;

    for j in 1..100000 {
        n = hailstone(j, None);
        if hmax < n {
            hmax = n;
            jatmax = j;
        }
    }

    n = hailstone(27, None);
    let mut arry = Vec::with_capacity(n as usize);
    n = hailstone(27, Some(&mut arry));

    println!(
        "[ {}, {}, {}, {}, ...., {}, {}, {}, {}] len={}",
        arry[0], arry[1], arry[2], arry[3],
        arry[n as usize - 4], arry[n as usize - 3], arry[n as usize - 2], arry[n as usize - 1], n
    );
    println!("Max {} at j= {}", hmax, jatmax);

    Ok(())
}