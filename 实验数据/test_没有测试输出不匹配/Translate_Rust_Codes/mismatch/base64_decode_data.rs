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
    -1
}

fn decode(source: &[u8], sink: &mut [u8]) -> i32 {
    let length = source.len();
    let mut it = source.iter();
    let end = match source.iter().last() {
        Some(val) => val,
        None => return 1, // Handle the error appropriately
    };
    let mut acc;

    if length % 4 != 0 {
        return 1;
    }

    let mut sink_it = sink.iter_mut();

    while it.clone().next() != Some(&end) {
        let b1 = match it.next() {
            Some(val) => *val,
            None => return 1, // Handle the error appropriately
        };
        let b2 = match it.next() {
            Some(val) => *val,
            None => return 1, // Handle the error appropriately
        };
        let b3 = match it.next() {
            Some(val) => *val,
            None => return 1, // Handle the error appropriately
        };         // might be the first padding byte
        let b4 = match it.next() {
            Some(val) => *val,
            None => return 1, // Handle the error appropriately
        };         // might be the first or second padding byte

        let i1 = find_index(b1);
        let i2 = find_index(b2);

        acc = i1 << 2;                  // six bits came from the first byte
        acc |= i2 >> 4;                 // two bits came from the first byte
        if let Some(sink_byte) = sink_it.next() {
            *sink_byte = acc as u8;
        } else {
            return 1; // Handle the error appropriately
        }                  // output the first byte

        if b3 != b'=' {
            let i3 = find_index(b3);

            acc = (i2 & 0xF) << 4;      // four bits came from the second byte
            acc += i3 >> 2;             // four bits came from the second byte
            if let Some(sink_byte) = sink_it.next() {
                *sink_byte = acc as u8;
            } else {
                return 1; // Handle the error appropriately
            }              // output the second byte

            if b4 != b'=' {
                let i4 = find_index(b4);

                acc = (i3 & 0x3) << 6;  // two bits came from the third byte
                acc |= i4;              // six bits came from the third byte
                if let Some(sink_byte) = sink_it.next() {
                    *sink_byte = acc as u8;
                } else {
                    return 1; // Handle the error appropriately
                }          // output the third byte
            }
        }
    }

    if let Some(sink_byte) = sink_it.next() {
        *sink_byte = b'\0';
    } else {
        return 1; // Handle the error appropriately
    }   // add the sigil for end of string
    0
}

fn main() {
    let data = "VG8gZXJyIGlzIGh1bWFuLCBidXQgdG8gcmVhbGx5IGZvdWwgdGhpbmdzIHVwIHlvdSBuZWVkIGEgY29tcHV0ZXIuCiAgICAtLVBhdWwgUi5FaHJsaWNo";
    let mut decoded = [0u8; 1024];

    println!("{}\n", data);
    decode(data.as_bytes(), &mut decoded);
    println!("{}\n", std::str::from_utf8(&decoded).unwrap());
}