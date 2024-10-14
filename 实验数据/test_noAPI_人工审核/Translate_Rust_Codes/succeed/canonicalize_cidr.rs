use std::io;

struct Cidr {
    address: u32,
    mask_length: u32,
}

fn cidr_parse(str: &str, cidr: &mut Cidr) -> bool {
    let mut parts = str.split('/');
    let ip_part = parts.next().unwrap();
    let mask_part = parts.next().unwrap();

    let ip_parts: Vec<&str> = ip_part.split('.').collect();
    if ip_parts.len() != 4 {
        return false;
    }

    let a: u32 = ip_parts[0].parse().unwrap_or(256);
    let b: u32 = ip_parts[1].parse().unwrap_or(256);
    let c: u32 = ip_parts[2].parse().unwrap_or(256);
    let d: u32 = ip_parts[3].parse().unwrap_or(256);
    let m: u32 = mask_part.parse().unwrap_or(0);

    if m < 1 || m > 32 || a > 255 || b > 255 || c > 255 || d > 255 {
        return false;
    }

    let mask = !((1 << (32 - m)) - 1);
    let address = (a << 24) + (b << 16) + (c << 8) + d;
    let address = address & mask;

    cidr.address = address;
    cidr.mask_length = m;
    true
}

fn cidr_format(cidr: &Cidr, str: &mut String) {
    let address = cidr.address;
    let d = address & 255;
    let address = address >> 8;
    let c = address & 255;
    let address = address >> 8;
    let b = address & 255;
    let address = address >> 8;
    let a = address & 255;

    str.clear();
    str.push_str(&format!("{}.{}.{}.{}/{}", a, b, c, d, cidr.mask_length));
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
        let mut cidr = Cidr { address: 0, mask_length: 0 };
        if cidr_parse(test, &mut cidr) {
            let mut out = String::new();
            cidr_format(&cidr, &mut out);
            println!("{:<18} -> {}", test, out);
        } else {
            eprintln!("{}: invalid CIDR", test);
        }
    }
}