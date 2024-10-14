fn bsearch(a: &[i32], x: i32) -> Option<usize> {
    let mut i = 0;
    let mut j = a.len() - 1;
    // Ensure that the loop does not run when there are no elements to search
    if a.len() == 0 { // No change needed: Correctly checks if the array is empty
        return None; // No change needed: Returns None immediately if the array is empty
    }
    while i <= j {
        let k = i + ((j - i) / 2);
        if a[k] == x {
            return Some(k);
        } else if a[k] < x {
            i = k + 1;
        } else {
            j = k - 1;
        }
    }
    // Ensure the function exits correctly when the element is not found
    None // No change needed: Correctly handles case where element is not found
}

fn bsearch_r(a: &[i32], x: i32, i: usize, j: usize) -> Option<usize> {
    // Ensure that the recursive call does not result in an out-of-bounds index
    if i > j { // No change needed: Correct base case for recursion
        return None;
    }
    let k = i + ((j - i) / 2);
    if a[k] == x {
        return Some(k);
    } else if a[k] < x {
        return bsearch_r(a, x, k + 1, j);
    } else {
        // Ensure that k - 1 is not out of bounds before making the recursive call
        if k > 0 { // Modified: Corrected condition to check if k - 1 is within bounds
            return bsearch_r(a, x, i, k - 1);
        } else {
            // Ensure the function exits correctly when the element is not found
            return None; // No change needed: Correctly handles case where element is not found
        }
    }
}

fn main() {
    let a = [-31, 0, 1, 2, 2, 4, 65, 83, 99, 782];
    let n = a.len();
    let x = 2;
    if let Some(i) = bsearch(&a, x) {
        println!("{} is at index {}", x, i);
    } else {
        println!("{} is not found", x);
    }
    let x = 5;
    if let Some(i) = bsearch_r(&a, x, 0, n - 1) {
        println!("{} is at index {}", x, i);
    } else {
        println!("{} is not found", x);
    }
}