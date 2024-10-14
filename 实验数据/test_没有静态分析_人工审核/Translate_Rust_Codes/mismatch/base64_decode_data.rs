const BASE64: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn find_index(val: u8) -> i32 {
    if b'A' <= val && val <= b'Z' {
        return (val - b'A') as i32;
    }
    if b'a' <= val && val <= b'z' {
        return (val - b'a' + 26) as i32;
    }
    if b'0' <= val && val <= b'9' {
        return (val - b'0' + 52) as i32;
    }
    if val == b'+' {
        return 62;
    }
    if val == b'/' {
        return 63;
    }
    -1 // Return -1 for any character not in the Base64 alphabet
}

fn decode(source: &[u8], sink: &mut [u8]) -> Result<usize, &'static str> {
    let length = source.len();
    let mut it = source.iter();
    let mut sink_it = sink.iter_mut();
    let mut decoded_len = 0;

    // Handle empty source slice
    if length == 0 {
        return Ok(0); // Return 0 for successful decoding with no output
    }

    // Ensure the length is a multiple of 4
    if length % 4 != 0 {
        return Err("Source length is not a multiple of 4");
    }

    while let Some(&b1) = it.next() {
        let b2 = match it.next() {
            Some(&b) => b,
            None => return Err("Unexpected end of source"),
        };
        let b3 = match it.next() {
            Some(&b) => b,
            None => return Err("Unexpected end of source"),
        };
        let b4 = match it.next() {
            Some(&b) => b,
            None => return Err("Unexpected end of source"),
        };

        let i1 = find_index(b1);
        let i2 = find_index(b2);

        // Handle invalid Base64 characters
        if i1 == -1 || i2 == -1 {
            return Err("Invalid Base64 character encountered");
        }

        // Handle padding characters
        let mut acc = i1 << 2; // six bits came from the first byte
        acc |= i2 >> 4; // two bits came from the first byte
        match sink_it.next() {
            Some(s) => *s = acc as u8,
            None => return Err("Sink iterator exhausted"),
        }
        decoded_len += 1;

        if b3 != b'=' {
            let i3 = find_index(b3);

            // Handle invalid Base64 characters
            if i3 == -1 {
                return Err("Invalid Base64 character encountered");
            }

            acc = (i2 & 0xF) << 4; // four bits came from the second byte
            acc += i3 >> 2; // four bits came from the second byte
            match sink_it.next() {
                Some(s) => *s = acc as u8,
                None => return Err("Sink iterator exhausted"),
            }
            decoded_len += 1;

            if b4 != b'=' {
                let i4 = find_index(b4);

                // Handle invalid Base64 characters
                if i4 == -1 {
                    return Err("Invalid Base64 character encountered");
                }

                acc = (i3 & 0x3) << 6; // two bits came from the third byte
                acc |= i4; // six bits came from the third byte
                match sink_it.next() {
                    Some(s) => *s = acc as u8,
                    None => return Err("Sink iterator exhausted"),
                }
                decoded_len += 1;
            }
        }
    }

    // Ensure the sink iterator does not exceed its bounds
    if sink_it.next().is_some() {
        return Err("Sink iterator exceeded expected bounds");
    }

    Ok(decoded_len) // Return the number of bytes successfully decoded
}

fn main() {
    let data = b"VG8gZXJyIGlzIGh1bWFuLCBidXQgdG8gcmVhbGx5IGZvdWwgdGhpbmdzIHVwIHlvdSBuZWVkIGEgY29tcHV0ZXIuCiAgICAtLVBhdWwgUi5FaHJsaWNo";
    let mut decoded = [0u8; 1024];

    println!("{}\n", String::from_utf8_lossy(data));
    match decode(data, &mut decoded) {
        Ok(len) => println!("{}\n", String::from_utf8_lossy(&decoded[..len])),
        Err(e) => println!("Error: {}\n", e),
    }
}