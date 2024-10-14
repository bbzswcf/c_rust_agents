const BASE64: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn find_index(val: u8) -> Option<u8> {
    if b'A' <= val && val <= b'Z' {
        return Some(val - b'A');
    }
    if b'a' <= val && val <= b'z' {
        return Some(val - b'a' + 26);
    }
    if b'0' <= val && val <= b'9' {
        return Some(val - b'0' + 52);
    }
    if val == b'+' {
        return Some(62);
    }
    if val == b'/' {
        return Some(63);
    }
    None
}

fn decode(source: &[u8], sink: &mut [u8]) -> Result<usize, String> {
    let length = source.len();
    if length % 4 != 0 {
        return Err("Input length is not a multiple of 4".to_string());
    }

    let mut it = source.iter().peekable();
    let mut sink_it = sink.iter_mut();
    let mut valid_bytes = 0;

    while let Some(&b1) = it.next() {
        let b2 = *it.next().ok_or("Unexpected end of input")?;
        let b3 = *it.next().ok_or("Unexpected end of input")?;
        let b4 = *it.next().ok_or("Unexpected end of input")?;

        let i1 = find_index(b1).ok_or("Invalid Base64 character")?;
        let i2 = find_index(b2).ok_or("Invalid Base64 character")?;

        let acc = (i1 << 2) | (i2 >> 4);
        if let Some(sink) = sink_it.next() {
            *sink = acc;
            valid_bytes += 1;
        } else {
            return Err("Output buffer too small".to_string());
        }

        if b3 != b'=' {
            let i3 = find_index(b3).ok_or("Invalid Base64 character")?;

            let acc = ((i2 & 0xF) << 4) | (i3 >> 2);
            if let Some(sink) = sink_it.next() {
                *sink = acc;
                valid_bytes += 1;
            } else {
                return Err("Output buffer too small".to_string());
            }

            if b4 != b'=' {
                let i4 = find_index(b4).ok_or("Invalid Base64 character")?;

                let acc = ((i3 & 0x3) << 6) | i4;
                if let Some(sink) = sink_it.next() {
                    *sink = acc;
                    valid_bytes += 1;
                } else {
                    return Err("Output buffer too small".to_string());
                }
            }
        }
    }

    Ok(valid_bytes)
}

fn main() {
    let data = "VG8gZXJyIGlzIGh1bWFuLCBidXQgdG8gcmVhbGx5IGZvdWwgdGhpbmdzIHVwIHlvdSBuZWVkIGEgY29tcHV0ZXIuCiAgICAtLVBhdWwgUi5FaHJsaWNo";
    let mut decoded = [0u8; 1024];

    println!("{}\n", data);
    match decode(data.as_bytes(), &mut decoded) {
        Ok(len) => println!("{}\n", std::str::from_utf8(&decoded[..len]).unwrap()),
        Err(e) => println!("Error: {}", e),
    }
}