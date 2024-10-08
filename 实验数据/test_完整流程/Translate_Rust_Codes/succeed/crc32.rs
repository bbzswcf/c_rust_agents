use std::io;

fn rc_crc32(crc: u32, buf: &str) -> u32 {
    static mut TABLE: [u32; 256] = [0; 256];
    static mut HAVE_TABLE: bool = false;

    unsafe {
        if !HAVE_TABLE {
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
            HAVE_TABLE = true;
        }

        let mut crc = !crc;
        let len = buf.len();
        let bytes = buf.as_bytes();
        for i in 0..len {
            let octet = bytes[i];
            crc = (crc >> 8) ^ TABLE[(crc as u8 ^ octet) as usize];
        }
        !crc
    }
}

fn main() {
    let s = "The quick brown fox jumps over the lazy dog";
    println!("{:X}", rc_crc32(0, s));
}