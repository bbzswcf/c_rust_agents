fn main() {
    let mut k = String::new();
    for i in 0..16 {
        for j in (32 + i..128).step_by(16) {
            k = match j {
                32 => "Spc".to_string(),
                127 => "Del".to_string(),
                _ => format!("{}", j as u8 as char),
            };
            print!("{:3} : {:<3}   ", j, k);
        }
        println!();
    }
}