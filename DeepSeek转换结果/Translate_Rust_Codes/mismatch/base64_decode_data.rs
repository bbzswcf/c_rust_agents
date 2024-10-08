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
    let mut sink_it = sink.iter_mut();
    let mut acc: i32;

    if length % 4 != 0 {
        return Err(1); // Invalid length
    }

    while let Some(b1) = it.next() {
        if let Some(b2) = it.next() {
            if let Some(b3) = it.next() {
                if let Some(b4) = it.next() {
                    let i1 = find_index(*b1);
                    let i2 = find_index(*b2);

                    if i1 == -1 || i2 == -1 {
                        return Err(2); // Invalid character
                    }

                    acc = i1 << 2;                  // six bits came from the first byte
                    acc |= i2 >> 4;                 // two bits came from the first byte
                    if let Some(sink_byte) = sink_it.next() {
                        *sink_byte = acc as u8;                  // output the first byte
                    } else {
                        return Err(4); // Sink buffer too small
                    }

                    if *b3 != b'=' {
                        let i3 = find_index(*b3);

                        if i3 == -1 {
                            return Err(2); // Invalid character
                        }

                        acc = (i2 & 0xF) << 4;      // four bits came from the second byte
                        acc += i3 >> 2;             // four bits came from the second byte
                        if let Some(sink_byte) = sink_it.next() {
                            *sink_byte = acc as u8;              // output the second byte
                        } else {
                            return Err(4); // Sink buffer too small
                        }

                        if *b4 != b'=' {
                            let i4 = find_index(*b4);

                            if i4 == -1 {
                                return Err(2); // Invalid character
                            }

                            acc = (i3 & 0x3) << 6;  // two bits came from the third byte
                            acc |= i4;              // six bits came from the third byte
                            if let Some(sink_byte) = sink_it.next() {
                                *sink_byte = acc as u8;          // output the third byte
                            } else {
                                return Err(4); // Sink buffer too small
                            }
                        }
                    }
                } else {
                    return Err(3); // Unexpected end of input
                }
            } else {
                return Err(3); // Unexpected end of input
            }
        } else {
            return Err(3); // Unexpected end of input
        }
    }

    Ok(())
}

fn main() {
    let data: &[u8] = b"VG8gZXJyIGlzIGh1bWFuLCBidXQgdG8gcmVhbGx5IGZvdWwgdGhpbmdzIHVwIHlvdSBuZWVkIGEgY29tcHV0ZXIuCiAgICAtLVBhdWwgUi5FaHJsaWNo";
    // 修改: 根据Base64编码规则,计算解码后的字节数
    let mut decoded = vec![0u8; (data.len() * 3) / 4];

    println!("{}", String::from_utf8_lossy(data));
    // Decode and handle the result
    match decode(data, &mut decoded) {
        Ok(_) => {
            match String::from_utf8(decoded) {
                Ok(s) => println!("{}\n", s), // 修改: 添加换行符以匹配C输出
                Err(_) => eprintln!("Error converting decoded bytes to string"),
            }
        },
        Err(e) => eprintln!("Error decoding: {}", e),
    }
}