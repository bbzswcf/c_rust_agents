fn main() {
    let mut k = [0u8; 4];
    for i in 0..16 {
        for j in (32 + i..128).step_by(16) {
            let s = match j {
                32 => "Spc",
                127 => "Del",
                _ => {
                    k[0] = j as u8;
                    std::str::from_utf8(&k[..1]).unwrap()
                }
            };
            print!("{:3} : {:3}   ", j, s);
        }
        println!();
    }
}