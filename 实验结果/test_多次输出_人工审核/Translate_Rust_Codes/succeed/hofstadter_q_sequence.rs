fn main() {
    const N: usize = 100000;
    let mut q = vec![0; N + 1];

    q[1] = 1;
    q[2] = 1;

    for i in 3..=N {
        q[i] = q[i - q[i - 1]] + q[i - q[i - 2]];
    }

    for i in 1..=10 {
        print!("{}{}", q[i], if i == 10 { '\n' } else { ' ' });
    }

    println!("{}", q[1000]);

    let mut flip = 0;
    for i in 1..N {
        if q[i] > q[i + 1] {
            flip += 1;
        }
    }

    println!("flips: {}", flip);
}