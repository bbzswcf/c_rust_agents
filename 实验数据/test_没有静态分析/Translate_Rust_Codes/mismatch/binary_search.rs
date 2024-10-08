fn bsearch(a: &[i32], x: i32) -> Option<usize> {
    let mut i = 0;
    let mut j = a.len() - 1;
    while i <= j {
        let k = i + (j - i) / 2;
        if a[k] == x {
            // Check if the element before k is also x
            if k > 0 && a[k - 1] == x {
                j = k - 1;
            } else {
                return Some(k);
            }
        } else if a[k] < x {
            i = k + 1;
        } else {
            j = k - 1;
        }
    }
    None
}

fn bsearch_r(a: &[i32], x: i32, i: usize, j: usize) -> Option<usize> {
    if i > j {
        return None;
    }
    let k = i + (j - i) / 2;
    if a[k] == x {
        return Some(k);
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
    if let Some(i) = bsearch(&a, x) {
        println!("{} is at index {}", x, i);
    } else {
        println!("{} is at index -1", x);
    }
    let x = 5;
    if let Some(i) = bsearch_r(&a, x, 0, n - 1) {
        println!("{} is at index {}", x, i);
    } else {
        println!("{} is at index -1", x);
    }
}