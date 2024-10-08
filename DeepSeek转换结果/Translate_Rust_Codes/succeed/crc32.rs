fn rc_crc32(crc: u32, buf: &str) -> u32 {
    static mut TABLE: [u32; 256] = [0; 256];
    static mut HAVE_TABLE: i32 = 0;

    unsafe {
        if HAVE_TABLE == 0 {
            for i in 0..256 {
                let mut rem = i as u32;
                for _ in 0..8 {
                    if rem & 1 != 0 {
                        rem >>= 1;
                        rem ^= 0xedb88320;
                    } else {
                        rem >>= 1;
                    }
                }
                TABLE[i] = rem;
            }
            HAVE_TABLE = 1;
        }

        let mut crc = !crc;
        for octet in buf.bytes() {
            crc = (crc >> 8) ^ TABLE[(crc as u8 ^ octet) as usize];
        }
        !crc
    }
}

fn main() {
    let s = "The quick brown fox jumps over the lazy dog";
    print!("{:X}\n", rc_crc32(0, s));
}