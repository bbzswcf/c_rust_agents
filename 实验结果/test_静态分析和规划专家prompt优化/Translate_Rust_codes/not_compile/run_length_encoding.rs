use std::io::{self, Write};
trait Stream {
    fn get(&self) -> i32;
    fn put(&mut self, c: i32) -> i32;
}
struct StringStream {
    string: String,
    pos: usize,
}
struct FileStream {
    fp: io::Stdout,
}
impl Stream for StringStream {
    fn get(&self) -> i32 {
        let c = self.string.chars().nth(self.pos).map(|c| c as u8 as i32).unwrap_or(-1);
        if c == -1 {
            return -1;
        }
        self.pos += 1;
        c
    }
    fn put(&mut self, c: i32) -> i32 {
        if c == -1 {
            self.string.push('\0');
            self.pos = 0;
        } else {
            self.string.push(c as u8 as char);
            self.pos += 1;
        }
        0
    }
}
impl Stream for FileStream {
    fn get(&self) -> i32 {
        -1
    }
    fn put(&mut self, c: i32) -> i32 {
        let buf = [c as u8];
        self.fp.write_all(&buf).unwrap();
        0
    }
}
fn sput(out_stream: &mut dyn Stream, c: i32) -> i32 {
    let s = out_stream as *mut dyn Stream as *mut StringStream;
    let s = unsafe { &mut *s };
    if c == -1 {
        s.string.push('\0');
        s.pos = 0;
    } else {
        s.string.push(c as u8 as char);
        s.pos += 1;
    }
    0
}
fn file_put(out_stream: &mut dyn Stream, c: i32) -> i32 {
    let f = out_stream as *mut dyn Stream as *mut FileStream;
    let f = unsafe { &mut *f };
    let buf = [c as u8];
    f.fp.write_all(&buf).unwrap();
    0
}
fn output(out: &mut dyn Stream, buf: &[u8], len: usize) {
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
    let get = in_stream.get;
    let put = out_stream.put;
    while !end {
        let c = get(in_stream);
        end = c == -1;
        if !end {
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
                (put)(out_stream, if end { len as i32 } else { (len - 1) as i32 });
                (put)(out_stream, buf[0] as i32);
                buf[0] = buf[len - 1];
                len = 1;
            }
        } else {
            if buf[len - 1] == buf[len - 2] {
                repeat = true;
                if len > 2 {
                    output(out_stream, &buf, len - 2);
                    buf[0] = buf[1] = buf[len - 1];
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
    (put)(out_stream, -1);
}
fn decode(in_stream: &dyn Stream, out_stream: &mut dyn Stream) {
    loop {
        let c = in_stream.get();
        if c == -1 {
            break;
        }
        if c > 128 {
            let cnt = c - 128;
            for _ in 0..cnt {
                out_stream.put(in_stream.get());
            }
        } else {
            let cnt = c;
            let c = in_stream.get();
            for _ in 0..cnt {
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
    let str_out = StringStream {
        string: buf,
        pos: 0,
    };
    let file = FileStream {
        fp: io::stdout(),
    };
    encode(&str_in as &dyn Stream, &mut str_out as &mut dyn Stream);
    decode(&str_out as &dyn Stream, &mut file as &mut dyn Stream);
}