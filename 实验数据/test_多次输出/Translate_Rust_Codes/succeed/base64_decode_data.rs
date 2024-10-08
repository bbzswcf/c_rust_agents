const BASE64: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn find_index(val: u8) -> Option<u8> {
    if b'A' <= val && val <= b'Z' {
        Some(val - b'A')
    } else if b'a' <= val && val <= b'z' {
        Some(val - b'a' + 26)
    } else if b'0' <= val && val <= b'9' {
        Some(val - b'0' + 52)
    } else if val == b'+' {
        Some(62)
    } else if val == b'/' {
        Some(63)
    } else {
        None
    }
}

fn decode(source: &[u8], sink: &mut [u8]) -> Result<usize, &'static str> {
    let length = source.len();
    if length % 4 != 0 {
        return Err("Invalid base64 length");
    }

    let mut sink_index = 0;
    let mut it = source.iter();

    while let (Some(&b1), Some(&b2), Some(&b3), Some(&b4)) = (it.next(), it.next(), it.next(), it.next()) {
        let i1 = find_index(b1).ok_or("Invalid base64 character")?;
        let i2 = find_index(b2).ok_or("Invalid base64 character")?;

        let mut acc = (i1 << 2) | (i2 >> 4);
        sink[sink_index] = acc;
        sink_index += 1;

        if b3 != b'=' {
            let i3 = find_index(b3).ok_or("Invalid base64 character")?;
            acc = ((i2 & 0xF) << 4) | (i3 >> 2);
            sink[sink_index] = acc;
            sink_index += 1;

            if b4 != b'=' {
                let i4 = find_index(b4).ok_or("Invalid base64 character")?;
                acc = ((i3 & 0x3) << 6) | i4;
                sink[sink_index] = acc;
                sink_index += 1;
            }
        }
    }

    Ok(sink_index)
}

fn main() {
    let data = b"VG8gZXJyIGlzIGh1bWFuLCBidXQgdG8gcmVhbGx5IGZvdWwgdGhpbmdzIHVwIHlvdSBuZWVkIGEgY29tcHV0ZXIuCiAgICAtLVBhdWwgUi5FaHJsaWNo";
    let mut decoded = [0u8; 1024];

    println!("{}\n", String::from_utf8_lossy(data));
    match decode(data, &mut decoded) {
        Ok(len) => println!("{}\n", String::from_utf8_lossy(&decoded[..len])),
        Err(e) => eprintln!("Error: {}", e),
    }
}