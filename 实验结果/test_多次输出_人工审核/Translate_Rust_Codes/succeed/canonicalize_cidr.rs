use std::net::Ipv4Addr;
use std::str::FromStr;

#[derive(Debug)]
struct Cidr {
    address: u32,
    mask_length: u8,
}

impl Cidr {
    fn parse(str: &str) -> Option<Self> {
        let mut parts = str.split('/');
        let ip_str = parts.next()?;
        let mask_str = parts.next()?;

        let ip = Ipv4Addr::from_str(ip_str).ok()?;
        let mask_length = mask_str.parse::<u8>().ok()?;

        if !(1..=32).contains(&mask_length) {
            return None;
        }

        let address = u32::from(ip);
        let mask = !((1 << (32 - mask_length)) - 1);
        let address = address & mask;

        Some(Cidr {
            address,
            mask_length,
        })
    }

    fn format(&self, str: &mut String) {
        let address = self.address;
        let d = (address & 0xFF) as u8;
        let c = ((address >> 8) & 0xFF) as u8;
        let b = ((address >> 16) & 0xFF) as u8;
        let a = ((address >> 24) & 0xFF) as u8;

        str.clear();
        str.push_str(&format!("{}.{}.{}.{}/{}", a, b, c, d, self.mask_length));
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
            let mut out = String::new();
            cidr.format(&mut out);
            println!("{:<18} -> {}", test, out);
        } else {
            eprintln!("{}: invalid CIDR", test);
        }
    }
}