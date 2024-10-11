use std::io;

type Ham = u64;

fn qpush(q: &mut Vec<Ham>, h: Ham) {
    let mut i: usize;
    let mut j: usize;

    if q.capacity() <= q.len() {
        q.reserve(if q.capacity() > 0 { q.capacity() * 2 } else { 16 });
    }

    q.push(h);
    i = q.len() - 1;
    while i > 0 {
        j = i / 2;
        if q[j] <= h {
            break;
        }
        q.swap(i, j);
        i = j;
    }
}

fn qpop(q: &mut Vec<Ham>) -> Ham {
    let mut i: usize;
    let mut j: usize;
    let mut r: Ham;
    let mut t: Ham;

    loop {
        r = q[0];
        if q.len() <= 1 || r != q[0] {
            break;
        }

        t = q.pop().unwrap();
        i = 0;
        while i * 2 < q.len() {
            j = i * 2;
            if j + 1 < q.len() && q[j] > q[j + 1] {
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
    let mut i: usize = 1;
    let mut h: Ham;

    qpush(&mut q, i as Ham);

    while i <= 1691 {
        h = qpop(&mut q);
        qpush(&mut q, h * 2);
        qpush(&mut q, h * 3);
        qpush(&mut q, h * 5);

        if i <= 20 || i == 1691 {
            println!("{:6}: {}", i, h);
        }

        i += 1;
    }

    Ok(())
}