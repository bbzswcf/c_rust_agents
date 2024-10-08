#[derive(Debug)]
struct Cidr {
    address: u32,
    mask_length: u32,
}

fn cidr_parse(str: &str) -> Option<Cidr> {
    let mut parts = str.split('/');
    let ip_part = parts.next()?;
    let mask_part = parts.next()?;

    let ip_parts: Vec<u32> = ip_part.split('.').map(|s| s.parse().unwrap_or(0)).collect();
    if ip_parts.len() != 4 {
        return None;
    }

    let a = ip_parts[0];
    let b = ip_parts[1];
    let c = ip_parts[2];
    let d = ip_parts[3];

    let m: u32 = mask_part.parse().unwrap_or(0);

    if m < 1 || m > 32 || a > u8::MAX as u32 || b > u8::MAX as u32 || c > u8::MAX as u32 || d > u8::MAX as u32 {
        return None;
    }

    let mask = !(u32::MAX >> m);
    let address = (a << 24) + (b << 16) + (c << 8) + d;
    let address = address & mask;

    Some(Cidr {
        address,
        mask_length: m,
    })
}

fn cidr_format(cidr: &Cidr) -> String {
    let address = cidr.address;
    let d = address & u8::MAX as u32;
    let address = address >> 8;
    let c = address & u8::MAX as u32;
    let address = address >> 8;
    let b = address & u8::MAX as u32;
    let address = address >> 8;
    let a = address & u8::MAX as u32;

    format!("{}.{}.{}.{}/{}", a, b, c, d, cidr.mask_length)
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
        if let Some(cidr) = cidr_parse(test) {
            let out = cidr_format(&cidr);
            println!("{:<18} -> {}", test, out);
        } else {
            eprintln!("{}: invalid CIDR", test);
        }
    }
}