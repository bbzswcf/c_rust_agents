// Function to find the index of a base64 character
fn find_index(val: u8) -> Result<u8, &'static str> {
    if b'A' <= val && val <= b'Z' {
        return Ok(val - b'A');
    }
    if b'a' <= val && val <= b'z' {
        return Ok(val - b'a' + 26);
    }
    if b'0' <= val && val <= b'9' {
        return Ok(val - b'0' + 52);
    }
    if val == b'+' {
        return Ok(62);
    }
    if val == b'/' {
        return Ok(63);
    }
    Err("Invalid base64 character")
}

// Function to decode base64 encoded data
fn decode(source: &[u8]) -> Result<Vec<u8>, &'static str> {
    let length = source.len();
    if length % 4 != 0 {
        return Err("Invalid base64 length");
    }

    // Modified: Correctly calculate the required size for the sink buffer
    let mut required_size = (length / 4) * 3;
    if source[length - 1] == b'=' {
        required_size -= 1;
    }
    if source[length - 2] == b'=' {
        required_size -= 1;
    }

    let mut sink = vec![0u8; required_size]; // Modified: Dynamically allocate the buffer
    let mut sink_index = 0;
    let mut i = 0;

    // Use a while loop with manual increment to avoid iterator overhead
    while i < length {
        let b1 = source[i];
        let b2 = source[i + 1];
        let b3 = source[i + 2];
        let b4 = source[i + 3];

        let i1 = find_index(b1)?;
        let i2 = find_index(b2)?;

        // Initialize acc only when it is needed
        let mut acc = (i1 << 2) | (i2 >> 4); // six bits came from the first byte, two bits from the second
        sink[sink_index] = acc; // output the first byte
        sink_index += 1;

        if b3 != b'=' {
            let i3 = find_index(b3)?;

            acc = ((i2 & 0xF) << 4) | (i3 >> 2); // four bits came from the second byte, four from the third
            sink[sink_index] = acc; // output the second byte
            sink_index += 1;

            if b4 != b'=' {
                let i4 = find_index(b4)?;

                acc = ((i3 & 0x3) << 6) | i4; // two bits came from the third byte, six from the fourth
                sink[sink_index] = acc; // output the third byte
                sink_index += 1;
            }
        }

        i += 4;
    }

    Ok(sink)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = b"VG8gZXJyIGlzIGh1bWFuLCBidXQgdG8gcmVhbGx5IGZvdWwgdGhpbmdzIHVwIHlvdSBuZWVkIGEgY29tcHV0ZXIuCiAgICAtLVBhdWwgUi5FaHJsaWNo";

    println!("{}", String::from_utf8_lossy(data));
    let decoded = decode(data)?;

    // Modified: Directly use the result of String::from_utf8_lossy to print the decoded data
    println!("{}", String::from_utf8_lossy(&decoded));

    Ok(())
}