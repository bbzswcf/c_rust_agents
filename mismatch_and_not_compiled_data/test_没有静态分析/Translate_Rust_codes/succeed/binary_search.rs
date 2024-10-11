fn bsearch(a: &[i32], x: i32) -> i32 {
    let mut i = 0;
    let mut j = a.len() - 1;
    while i <= j {
        let k = i + ((j - i) / 2);
        if a[k] == x {
            return k as i32;
        } else if a[k] < x {
            i = k + 1;
        } else {
            j = k - 1;
        }
    }
    -1
}

fn bsearch_r(a: &[i32], x: i32, i: usize, j: usize) -> i32 {
    if j < i {
        return -1;
    }
    let k = i + ((j - i) / 2);
    if a[k] == x {
        return k as i32;
    } else if a[k] < x {
        return bsearch_r(a, x, k + 1, j);
    } else {
        return bsearch_r(a, x, i, k - 1);
    }
}

fn main() {
    let a = [-31, 0, 1, 2, 2, 4, 65, 83, 99, 782];
    let n = a.len();
    let x = 2;
    let i = bsearch(&a, x);
    println!("{} is at index {}", x, i);
    let x = 5;
    let i = bsearch_r(&a, x, 0, n - 1);
    println!("{} is at index {}", x, i);
}