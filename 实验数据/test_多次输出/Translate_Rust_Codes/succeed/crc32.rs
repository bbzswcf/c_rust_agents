use std::sync::Once;

fn rc_crc32(crc: u32, buf: &[u8]) -> u32 {
    static mut TABLE: [u32; 256] = [0; 256];
    static INIT: Once = Once::new();

    unsafe {
        INIT.call_once(|| {
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
        });

        let mut crc = !crc;
        for &octet in buf {
            crc = (crc >> 8) ^ TABLE[(crc as u8 ^ octet) as usize];
        }
        !crc
    }
}

fn main() {
    let s = b"The quick brown fox jumps over the lazy dog";
    println!("{:X}", rc_crc32(0, s));
}