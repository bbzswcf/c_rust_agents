type Xint = i64;

static U: [[Xint; 9]; 3] = [
    [1_i64, -2_i64, 2_i64, 2_i64, -1_i64, 2_i64, 2_i64, -2_i64, 3_i64],
    [1_i64, 2_i64, 2_i64, 2_i64, 1_i64, 2_i64, 2_i64, 2_i64, 3_i64],
    [-1_i64, 2_i64, 2_i64, -2_i64, 1_i64, 2_i64, -2_i64, 2_i64, 3_i64],
];

fn new_tri(in_: &[Xint; 3], total: &mut Xint, prim: &mut Xint, max_peri: Xint) {
    let mut stack = vec![*in_];

    while let Some(mut t) = stack.pop() {
        if let Some(p) = t[0].checked_add(t[1]).and_then(|sum| sum.checked_add(t[2])) {
            if p > max_peri {
                continue;
            }

            *prim += 1;
            if let Some(new_total) = total.checked_add(max_peri.checked_div(p).unwrap_or(i64::MAX)) {
                *total = new_total;
            } else {
                eprintln!("Overflow detected in total calculation");
            }

            for i in 0..3 {
                if let Some(t0) = U[i][0].checked_mul(in_[0])
                    .and_then(|x| x.checked_add(U[i][1].checked_mul(in_[1]).unwrap_or(i64::MAX)))
                    .and_then(|x| x.checked_add(U[i][2].checked_mul(in_[2]).unwrap_or(i64::MAX)))
                {
                    t[0] = t0;
                } else {
                    eprintln!("Overflow detected in t[0] calculation");
                    continue;
                }

                if let Some(t1) = U[i][3].checked_mul(in_[0])
                    .and_then(|x| x.checked_add(U[i][4].checked_mul(in_[1]).unwrap_or(i64::MAX)))
                    .and_then(|x| x.checked_add(U[i][5].checked_mul(in_[2]).unwrap_or(i64::MAX)))
                {
                    t[1] = t1;
                } else {
                    eprintln!("Overflow detected in t[1] calculation");
                    continue;
                }

                if let Some(t2) = U[i][6].checked_mul(in_[0])
                    .and_then(|x| x.checked_add(U[i][7].checked_mul(in_[1]).unwrap_or(i64::MAX)))
                    .and_then(|x| x.checked_add(U[i][8].checked_mul(in_[2]).unwrap_or(i64::MAX)))
                {
                    t[2] = t2;
                } else {
                    eprintln!("Overflow detected in t[2] calculation");
                    continue;
                }

                stack.push(t.clone());
            }
        } else {
            eprintln!("Overflow detected in perimeter calculation");
        }
    }
}

fn main() {
    let seed = [3, 4, 5];
    let mut max_peri = 10;

    while max_peri <= 100000000 {
        let mut total = 0;
        let mut prim = 0;
        new_tri(&seed, &mut total, &mut prim, max_peri);

        println!(
            "Up to {}: {} triples, {} primitives.",
            max_peri, total, prim
        );

        if let Some(new_max_peri) = max_peri.checked_mul(10) {
            max_peri = new_max_peri;
        } else {
            eprintln!("Overflow detected in max_peri calculation");
            break;
        }
    }
}