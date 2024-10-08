fn qselect(v: &mut [i32], k: usize) -> i32 {
    macro_rules! SWAP {
        ($a:expr, $b:expr) => {
            {
                let tmp = v[$a];
                v[$a] = v[$b];
                v[$b] = tmp;
            }
        };
    }

    let len = v.len();
    let mut st = 0;

    for i in 0..len - 1 {
        if v[i] > v[len - 1] {
            continue;
        }
        SWAP!(i, st);
        st += 1;
    }

    SWAP!(len - 1, st);

    if k == st {
        v[st]
    } else if st > k {
        qselect(&mut v[..st], k)
    } else {
        qselect(&mut v[st..], k - st)
    }
}

fn main() {
    let x = [9, 8, 7, 6, 5, 0, 1, 2, 3, 4];
    let n = x.len();
    let mut y = [0; 10];

    for i in 0..10 {
        y.copy_from_slice(&x);
        println!("{}: {}", i, qselect(&mut y, i));
    }
}