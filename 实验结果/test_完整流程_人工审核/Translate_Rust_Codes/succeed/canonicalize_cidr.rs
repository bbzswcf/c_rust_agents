use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
struct Cidr {
    address: u32,
    mask_length: u8,
}

impl Cidr {
    fn parse(str: &str) -> Option<Self> {
        let parts: Vec<&str> = str.split('/').collect();
        if parts.len() != 2 {
            return None;
        }

        let ip_parts: Vec<&str> = parts[0].split('.').collect();
        if ip_parts.len() != 4 {
            return None;
        }

        let a: u8 = ip_parts[0].parse().ok()?;
        let b: u8 = ip_parts[1].parse().ok()?;
        let c: u8 = ip_parts[2].parse().ok()?;
        let d: u8 = ip_parts[3].parse().ok()?;
        let m: u8 = parts[1].parse().ok()?;

        if m < 1 || m > 32 {
            return None;
        }

        let mask = !((1 << (32 - m)) - 1);
        let address = ((a as u32) << 24) + ((b as u32) << 16) + ((c as u32) << 8) + (d as u32);
        let address = address & mask;

        Some(Cidr {
            address,
            mask_length: m,
        })
    }

    fn format(&self) -> String {
        let address = self.address;
        let d = address & 0xFF;
        let address = address >> 8;
        let c = address & 0xFF;
        let address = address >> 8;
        let b = address & 0xFF;
        let address = address >> 8;
        let a = address & 0xFF;
        format!("{}.{}.{}.{}/{}", a, b, c, d, self.mask_length)
    }
}

fn main() {
    let tests = [
        "87.70.141.1/22",
        "36.18.154.103/12",
        "62.62.197.11/29",
        "67.137.119.181/4",
        "161.214.74.21/24",
        "184.232.176.184/18",
    ];

    for test in tests.iter() {
        if let Some(cidr) = Cidr::parse(test) {
            let out = cidr.format();
            println!("{:<18} -> {}", test, out);
        } else {
            eprintln!("{}: invalid CIDR", test);
        }
    }
}