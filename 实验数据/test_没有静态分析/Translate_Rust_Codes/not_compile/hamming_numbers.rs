use std::io;

type Ham = u64;

fn qpush(q: &mut Vec<Ham>, n: &mut usize, h: Ham) {
    let mut i: usize;
    let mut j: usize;

    // Modified: Check against capacity instead of length
    if *n >= q.capacity() {
        // Resize to a more appropriate size based on expected growth pattern
        q.resize(if q.len() > 0 { q.len() * 2 } else { 16 }, 0);
    }

    i = *n;
    *n += 1;
    while i > 0 {
        j = i / 2;
        if q[j] <= h {
            break;
        }
        q[i] = q[j];
        i = j;
    }
    q[i] = h;
}

fn qpop(q: &mut Vec<Ham>, n: &mut usize) -> Ham {
    let mut i: usize;
    let mut j: usize;
    let mut r: Ham;
    let mut t: Ham;

    loop {
        r = q[0];
        // Modified: Break the loop when *n == 0
        if *n == 0 {
            break;
        }

        t = q[*n - 1];
        *n -= 1;
        i = 0;
        // Modified: Ensure i * 2 + 1 is within bounds
        while i * 2 + 1 < *n {
            j = i * 2;
            // Modified: Ensure j + 1 is within bounds
            if j + 1 < *n && q[j] > q[j + 1] {
                j += 1;
            }
            if t <= q[j] {
                break;
            }
            q[i] = q[j];
            i = j;
        }
        q[i] = t;
    }

    r
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut q: Vec<Ham> = Vec::new();
    let mut n: usize = 0;
    // Modified: Correct initial value for i to match C implementation
    let mut i: usize = 1;
    let mut h: Ham;

    // Modified: Push the first value as 1 to match C implementation
    qpush(&mut q, &mut n, 1);

    while i <= 1691 {
        h = qpop(&mut q, &mut n);
        // Modified: Use checked multiplication to handle large values
        if let Some(h2) = h.checked_mul(2) {
            qpush(&mut q, &mut n, h2);
        }
        if let Some(h3) = h.checked_mul(3) {
            qpush(&mut q, &mut n, h3);
        }
        if let Some(h5) = h.checked_mul(5) {
            qpush(&mut q, &mut n, h5);
        }

        if i <= 20 || i == 1691 {
            // Modified: Correct format specifier for alignment to match C implementation
            println!("{:6}: {}", i, h);
        }

        i += 1;
    }

    Ok(())
}