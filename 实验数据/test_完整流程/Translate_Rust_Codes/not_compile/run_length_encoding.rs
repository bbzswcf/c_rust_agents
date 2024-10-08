use std::io::{self, Write};
use std::convert::TryInto;

struct Stream;

trait StreamImpl {
    fn get(&mut self) -> i32;
    fn put(&mut self, c: i32) -> i32;
}

struct StringStream {
    string: String,
    pos: usize,
}

impl StreamImpl for StringStream {
    fn get(&mut self) -> i32 {
        // Ensure the conversion from `char` to `i32` is correct
        let c = self.string.chars().nth(self.pos).map(|c| c as i32).unwrap_or(0);
        if c == 0 {
            return -1;
        }
        self.pos += 1;
        c
    }

    fn put(&mut self, c: i32) -> i32 {
        // Ensure the conversion from `i32` to `char` is correct
        let c = if c == -1 { char::from_u32(0).unwrap_or('\0') } else { char::from_u32(c as u32).unwrap_or('\0') };
        self.string.push(c);
        self.pos += 1;
        0
    }
}

struct FileStream {
    fp: io::Stdout,
}

impl StreamImpl for FileStream {
    fn get(&mut self) -> i32 {
        -1
    }

    fn put(&mut self, c: i32) -> i32 {
        // Ensure the conversion from `i32` to `u8` is correct
        let c = if c >= 0 && c <= 255 { c as u8 } else { 0 };
        self.fp.write_all(&[c]).unwrap();
        0
    }
}

fn sget(in_stream: &mut dyn StreamImpl) -> i32 {
    in_stream.get()
}

fn sput(out_stream: &mut dyn StreamImpl, c: i32) -> i32 {
    out_stream.put(c)
}

fn file_put(out_stream: &mut dyn StreamImpl, c: i32) -> i32 {
    out_stream.put(c)
}

fn output(out: &mut dyn StreamImpl, buf: &[u8], len: usize) {
    // Ensure the conversion from `usize` to `i32` is correct
    out.put(128 + len.try_into().unwrap_or(0));
    for i in 0..len {
        // Ensure the conversion from `u8` to `i32` is correct
        out.put(buf[i] as i32);
    }
}

fn encode(in_stream: &mut dyn StreamImpl, out_stream: &mut dyn StreamImpl) {
    let mut buf = [0u8; 256];
    let mut len = 0;
    let mut repeat = false;
    let mut end = false;

    while !end {
        end = in_stream.get() == -1;
        if !end {
            buf[len] = in_stream.get() as u8;
            if len < buf.len() { len += 1; } else { break; } // Ensure `len` does not overflow
            if len <= 1 {
                continue;
            }
        }

        if repeat {
            if buf[len - 1] != buf[len - 2] {
                repeat = false;
            }
            if !repeat || len == 129 || end {
                out_stream.put(if end { len as i32 } else { (len - 1) as i32 });
                out_stream.put(buf[0] as i32);
                buf[0] = buf[len - 1];
                if len <= buf.len() { len = 1; } else { break; } // Ensure `len` does not overflow
            }
        } else {
            if buf[len - 1] == buf[len - 2] {
                repeat = true;
                if len > 2 {
                    output(out_stream, &buf[..len - 2], len - 2);
                    if len > 1 { buf[1] = buf[len - 1]; } // Ensure `len - 1` does not cause out-of-bounds access
                    buf[0] = buf[1];
                    if len <= buf.len() { len = 2; } else { break; } // Ensure `len` does not overflow
                }
                continue;
            }
            if len == 128 || end {
                output(out_stream, &buf[..len], len);
                if len <= buf.len() { len = 0; } else { break; } // Ensure `len` does not overflow
                repeat = false;
            }
        }
    }
    out_stream.put(-1);
}

fn decode(in_stream: &mut dyn StreamImpl, out_stream: &mut dyn StreamImpl) {
    loop {
        let c = in_stream.get();
        if c == -1 {
            return;
        }
        // Ensure the subtraction result is within the valid range for `i32`
        let cnt = if c > 128 { c.checked_sub(128).unwrap_or(0) } else { 0 };
        if cnt >= 0 { for _ in 0..cnt { out_stream.put(in_stream.get()); } } // Ensure `cnt` is within the valid range for iteration
        if c <= 128 {
            // Ensure the `c` is within the valid range for `i32`
            let cnt = if c >= 0 && c <= 128 { c } else { 0 };
            let c = in_stream.get();
            for _ in 0..cnt {
                out_stream.put(c);
            }
        }
    }
}

fn main() {
    let buf = String::new(); // Removed `mut` keyword since the variable does not need to be mutable
    let mut str_in = StringStream {
        string: "WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWBWWWWWWWWWWWWWW".to_string(),
        pos: 0,
    };
    let mut str_out = StringStream {
        string: buf,
        pos: 0,
    };
    let mut file = FileStream {
        fp: io::stdout(),
    };

    encode(&mut str_in, &mut str_out);
    decode(&mut str_out, &mut file);
}