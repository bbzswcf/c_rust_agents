use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
struct Cidr {
    address: u32,
    mask_length: u8,
}

impl FromStr for Cidr {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() != 2 {
            return Err("Invalid CIDR format");
        }

        let ip_parts: Vec<&str> = parts[0].split('.').collect();
        if ip_parts.len() != 4 {
            return Err("Invalid IP address format");
        }

        let a: u8 = ip_parts[0].parse().map_err(|_| "Invalid IP address format")?;
        let b: u8 = ip_parts[1].parse().map_err(|_| "Invalid IP address format")?;
        let c: u8 = ip_parts[2].parse().map_err(|_| "Invalid IP address format")?;
        let d: u8 = ip_parts[3].parse().map_err(|_| "Invalid IP address format")?;

        let m: u8 = parts[1].parse().map_err(|_| "Invalid mask length")?;
        if m < 1 || m > 32 {
            return Err("Invalid mask length");
        }

        let mask = !((1 << (32 - m)) - 1);
        let address = ((a as u32) << 24) + ((b as u32) << 16) + ((c as u32) << 8) + (d as u32);
        let address = address & mask;

        Ok(Cidr {
            address,
            mask_length: m,
        })
    }
}

impl fmt::Display for Cidr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let address = self.address;
        let d = address & 0xFF;
        let address = address >> 8;
        let c = address & 0xFF;
        let address = address >> 8;
        let b = address & 0xFF;
        let address = address >> 8;
        let a = address & 0xFF;
        write!(f, "{}.{}.{}.{}/{}", a, b, c, d, self.mask_length)
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
        match test.parse::<Cidr>() {
            Ok(cidr) => {
                println!("{:<18} -> {}", test, cidr);
            }
            Err(_) => {
                eprintln!("{}: invalid CIDR", test);
            }
        }
    }
}