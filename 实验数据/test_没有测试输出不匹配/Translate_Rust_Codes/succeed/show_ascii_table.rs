fn main() {
    let mut k = String::new();
    for i in 0..16 {
        for j in (32 + i..128).step_by(16) {
            match j {
                32 => k = "Spc".to_string(),
                127 => k = "Del".to_string(),
                _ => k = format!("{}", j as u8 as char),
            }
            print!("{:3} : {:<3}   ", j, k);
        }
        println!();
    }
}