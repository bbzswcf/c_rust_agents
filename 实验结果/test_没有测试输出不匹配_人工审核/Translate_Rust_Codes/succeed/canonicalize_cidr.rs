use std::fmt::Write;
use std::str::FromStr;

#[derive(Debug)]
struct Cidr {
    address: u32,
    mask_length: u8,
}

fn cidr_parse(str: &str) -> Option<Cidr> {
    let parts: Vec<&str> = str.split('/').collect();
    if parts.len() != 2 {
        return None;
    }

    let address_parts: Vec<&str> = parts[0].split('.').collect();
    if address_parts.len() != 4 {
        return None;
    }

    let a = u8::from_str(address_parts[0]).ok()?;
    let b = u8::from_str(address_parts[1]).ok()?;
    let c = u8::from_str(address_parts[2]).ok()?;
    let d = u8::from_str(address_parts[3]).ok()?;
    let m = u8::from_str(parts[1]).ok()?;

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

fn cidr_format(cidr: &Cidr, str: &mut String) {
    let address = cidr.address;
    let d = address & 0xFF;
    let address = address >> 8;
    let c = address & 0xFF;
    let address = address >> 8;
    let b = address & 0xFF;
    let address = address >> 8;
    let a = address & 0xFF;

    write!(str, "{}.{}.{}.{}/{}", a, b, c, d, cidr.mask_length).unwrap();
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
            let mut out = String::new();
            cidr_format(&cidr, &mut out);
            println!("{:<18} -> {}", test, out);
        } else {
            eprintln!("{}: invalid CIDR", test);
        }
    }
}