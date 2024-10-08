use std::io::{self, Write};

struct Stream {
    get: fn(&mut dyn StreamImpl) -> i32,
    put: fn(&mut dyn StreamImpl, i32) -> i32,
}

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
        let c = self.string.chars().nth(self.pos).map(|c| c as u32 as i32).unwrap_or(-1);
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

struct FileStream {
    fp: Box<dyn Write>,
}

impl StreamImpl for FileStream {
    fn get(&mut self) -> i32 {
        unimplemented!()
    }

    fn put(&mut self, c: i32) -> i32 {
        if let Err(_) = self.fp.write_all(&[c as u8]) {
            return -1;
        }
        0
    }
}

fn output(out: &mut dyn StreamImpl, buf: &[u8], len: usize) {
    out.put(128 + len as i32);
    for i in 0..len {
        out.put(buf[i] as i32);
    }
}

fn encode(in_stream: &mut dyn StreamImpl, out_stream: &mut dyn StreamImpl) {
    let mut buf = [0u8; 256];
    let mut len = 0;
    let mut repeat = false;
    let mut end = false;

    while !end {
        let c = in_stream.get();
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
                out_stream.put(if end { len as i32 } else { (len - 1) as i32 });
                out_stream.put(buf[0] as i32);
                buf[0] = buf[len - 1];
                len = 1;
            }
        } else {
            if buf[len - 1] == buf[len - 2] {
                repeat = true;
                if len > 2 {
                    output(out_stream, &buf[..len - 2], len - 2);
                    buf[0] = buf[len - 1];
                    buf[1] = buf[len - 1];
                    len = 2;
                }
                continue;
            }
            if len == 128 || end {
                output(out_stream, &buf[..len], len);
                len = 0;
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
            break;
        }
        if c > 128 {
            let cnt = c - 128;
            for _ in 0..cnt {
                let c = in_stream.get();
                out_stream.put(c);
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

fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut str_in = StringStream {
        string: "WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWBWWWWWWWWWWWWWW".to_string(),
        pos: 0,
    };
    let mut str_out = StringStream {
        string: buf,
        pos: 0,
    };
    let mut file = FileStream {
        fp: Box::new(io::stdout()),
    };

    encode(&mut str_in, &mut str_out);
    decode(&mut str_out, &mut file);

    Ok(())
}