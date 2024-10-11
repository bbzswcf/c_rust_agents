use std::io::{self, Write};

trait Stream {
    fn get(&self) -> i32;
    fn put(&mut self, c: i32) -> i32;
}

struct StringStream {
    string: String,
    pos: usize,
}

impl Stream for StringStream {
    fn get(&self) -> i32 {
        // Modified: Ensure the character is correctly converted to i32 using char::to_digit
        let c = self.string.chars().nth(self.pos).map(|c| c.to_digit(10).unwrap_or(0).unwrap_or(0) as i32).unwrap_or(-1);
        if c == -1 {
            return -1;
        }
        c
    }

    fn put(&mut self, c: i32) -> i32 {
        if c == -1 {
            self.string.push('\0');
            self.pos = 0;
        } else {
            // Modified: Ensure the character is correctly converted from i32 to char using char::from_u32
            if let Some(ch) = std::char::from_u32(c as u32) {
                self.string.push(ch);
            } else {
                self.string.push('\0');
            }
            self.pos += 1;
        }
        0
    }
}

struct FileStream {
    fp: io::Stdout,
}

impl Stream for FileStream {
    fn get(&self) -> i32 {
        -1 // FileStream does not support get operation
    }

    fn put(&mut self, c: i32) -> i32 {
        // Modified: Ensure the character is correctly converted from i32 to u8 using as u8
        if c >= 0 && c <= 255 {
            let buf = [c as u8];
            self.fp.write_all(&buf).unwrap();
        }
        0
    }
}

fn output(out: &mut dyn Stream, buf: &[u8], len: usize) {
    // Modified: Ensure the length is correctly cast to i32 using as i32
    out.put(128 + len as i32);
    for &b in buf.iter().take(len) {
        out.put(b as i32);
    }
}

fn encode(in_stream: &dyn Stream, out_stream: &mut dyn Stream) {
    let mut buf = [0u8; 256];
    let mut len = 0;
    let mut repeat = false;
    let mut end = false;

    while !end {
        let c = in_stream.get();
        end = c == -1;
        if !end {
            // Modified: Ensure the character is correctly converted from i32 to u8 using as u8
            buf[len] = c as u8;
            len += 1;
            if len <= 1 {
                continue;
            }
        }

        if repeat {
            if buf[len - 1] != buf[len - 2] {
                repeat = false;
            }
            if !repeat || len == 129 || end {
                // Modified: Ensure the length is correctly cast to i32 using as i32
                out_stream.put(if end { len as i32 } else { (len - 1) as i32 });
                // Modified: Ensure the character is correctly converted from u8 to i32 using as i32
                out_stream.put(buf[0] as i32);
                buf[0] = buf[len - 1];
                len = 1;
            }
        } else {
            if buf[len - 1] == buf[len - 2] {
                repeat = true;
                if len > 2 {
                    output(out_stream, &buf, len - 2);
                    buf[0] = buf[len - 1];
                    len = 2;
                }
                continue;
            }
            if len == 128 || end {
                output(out_stream, &buf, len);
                len = 0;
                repeat = false;
            }
        }
    }
    out_stream.put(-1);
}

fn decode(in_stream: &dyn Stream, out_stream: &mut dyn Stream) {
    loop {
        let c = in_stream.get();
        if c == -1 {
            break;
        }
        if c > 128 {
            // Modified: Ensure the subtraction result is correctly handled
            let cnt = c - 128;
            for _ in 0..cnt {
                out_stream.put(in_stream.get());
            }
        } else {
            let cnt = c;
            let c = in_stream.get();
            for _ in 0..cnt {
                // Modified: Ensure the character is correctly passed to the put method
                out_stream.put(c);
            }
        }
    }
}

fn main() {
    let buf = String::new();
    let str_in = StringStream {
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

    encode(&str_in, &mut str_out);
    decode(&str_out, &mut file);
}