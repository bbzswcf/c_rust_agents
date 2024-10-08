fn main() {
    for i in 0..4 {
        for j in (32 + i * 16..32 + (i + 1) * 16) {
            let char_str = match j {
                32 => "Spc".to_string(),
                127 => "Del".to_string(),
                _ => std::char::from_u32(j as u32).map(|c| c.to_string()).unwrap_or(format!("{:3}", j)),
            };
            print!("{:3} : {:>3}", j, char_str);
        }
        println!();
    }
}