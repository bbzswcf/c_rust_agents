use std::io::{self, Write};

struct Stream {
    get: fn(&mut dyn Stream) -> i32,
    put: fn(&mut dyn Stream, i32) -> i32,
}

struct StringStream {
    get: fn(&mut dyn Stream) -> i32,
    put: fn(&mut dyn Stream, i32) -> i32,
    string: String,
    pos: usize,
}

struct FileStream {
    get: fn(&mut dyn Stream) -> i32,
    put: fn(&mut dyn Stream, i32) -> i32,
    fp: Box<dyn Write>,
}

impl Stream for StringStream {
    fn get(&mut self) -> i32 {
        let s = self as *mut StringStream as *mut dyn Stream;
        (self.get)(&mut *s)
    }

    fn put(&mut self, c: i32) -> i32 {
        let s = self as *mut StringStream as *mut dyn Stream;
        (self.put)(&mut *s, c)
    }
}

impl Stream for FileStream {
    fn get(&mut self) -> i32 {
        let s = self as *mut FileStream as *mut dyn Stream;
        (self.get)(&mut *s)
    }

    fn put(&mut self, c: i32) -> i32 {
        let s = self as *mut FileStream as *mut dyn Stream;
        (self.put)(&mut *s, c)
    }
}

fn sget(in_stream: &mut dyn Stream) -> i32 {
    let s = in_stream as *mut dyn Stream as *mut StringStream;
    let s = unsafe { &mut *s };
    let c = s.string.chars().nth(s.pos).map(|c| c as u8 as i32).unwrap_or(-1);
    if c == -1 {
        return -1;
    }
    s.pos += 1;
    c
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
    let s = out_stream as *mut dyn Stream as *mut FileStream;
    let s = unsafe { &mut *s };
    let buf = [c as u8];
    s.fp.write_all(&buf).unwrap();
    0
}

fn output(out: &mut dyn Stream, buf: &[u8], len: usize) {
    (out.put)(out, 128 + len as i32);
    for &b in buf.iter().take(len) {
        (out.put)(out, b as i32);
    }
}

fn encode(in_stream: &mut dyn Stream, out_stream: &mut dyn Stream) {
    let mut buf = [0u8; 256];
    let mut len = 0;
    let mut repeat = false;
    let mut end = false;
    let get = in_stream.get;
    let put = out_stream.put;

    while !end {
        end = ((get)(in_stream) == -1);
        if !end {
            buf[len] = (get)(in_stream) as u8;
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

fn decode(in_stream: &mut dyn Stream, out_stream: &mut dyn Stream) {
    loop {
        let c = (in_stream.get)(in_stream);
        if c == -1 {
            break;
        }
        if c > 128 {
            let cnt = c - 128;
            for _ in 0..cnt {
                (out_stream.put)(out_stream, (in_stream.get)(in_stream));
            }
        } else {
            let cnt = c;
            let c = (in_stream.get)(in_stream);
            for _ in 0..cnt {
                (out_stream.put)(out_stream, c);
            }
        }
    }
}

fn main() -> io::Result<()> {
    let buf = String::new();
    let mut str_in = StringStream {
        get: sget,
        put: sput,
        string: "WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWBWWWWWWWWWWWWWW".to_string(),
        pos: 0,
    };
    let mut str_out = StringStream {
        get: sget,
        put: sput,
        string: buf,
        pos: 0,
    };
    let mut file = FileStream {
        get: sget,
        put: file_put,
        fp: Box::new(io::stdout()),
    };

    encode(&mut str_in as &mut dyn Stream, &mut str_out as &mut dyn Stream);
    decode(&mut str_out as &mut dyn Stream, &mut file as &mut dyn Stream);

    Ok(())
}