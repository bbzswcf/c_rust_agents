const BASE64: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

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

fn decode(source: &[u8], sink: &mut [u8]) -> Result<(), i32> {
    let length = source.len();
    let mut it = source.iter();
    let end = source.iter().last().unwrap();
    let mut sink_it = sink.iter_mut();
    let mut acc: i32;

    if length % 4 != 0 {
        return Err(1);
    }

    while it.clone().next() != Some(&end) {
        let b1 = *it.next().unwrap();
        let b2 = *it.next().unwrap();
        let b3 = *it.next().unwrap(); // might be the first padding byte
        let b4 = *it.next().unwrap(); // might be the first or second padding byte

        let i1 = find_index(b1);
        let i2 = find_index(b2);

        acc = i1 << 2; // six bits came from the first byte
        acc |= i2 >> 4; // two bits came from the first byte
        *sink_it.next().unwrap() = acc as u8; // output the first byte

        if b3 != b'=' {
            let i3 = find_index(b3);

            acc = (i2 & 0xF) << 4; // four bits came from the second byte
            acc += i3 >> 2; // four bits came from the second byte
            *sink_it.next().unwrap() = acc as u8; // output the second byte

            if b4 != b'=' {
                let i4 = find_index(b4);

                acc = (i3 & 0x3) << 6; // two bits came from the third byte
                acc |= i4; // six bits came from the third byte
                *sink_it.next().unwrap() = acc as u8; // output the third byte
            }
        }
    }

    *sink_it.next().unwrap() = b'\0'; // add the sigil for end of string
    Ok(())
}

fn main() {
    let data = b"VG8gZXJyIGlzIGh1bWFuLCBidXQgdG8gcmVhbGx5IGZvdWwgdGhpbmdzIHVwIHlvdSBuZWVkIGEgY29tcHV0ZXIuCiAgICAtLVBhdWwgUi5FaHJsaWNo";
    let mut decoded = [0u8; 1024];

    println!("{}\n", String::from_utf8_lossy(data));
    if let Err(e) = decode(data, &mut decoded) {
        println!("Decoding failed with error code: {}", e);
    } else {
        println!("{}\n", String::from_utf8_lossy(&decoded));
    }
}