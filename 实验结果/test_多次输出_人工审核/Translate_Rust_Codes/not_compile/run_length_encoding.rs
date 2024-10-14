use std::fs::File;
use std::io::{self, Write};
use std::ptr;

struct Stream {
    get: fn(&mut dyn StreamImpl) -> i32,
    put: fn(&mut dyn StreamImpl, i32) -> i32,
}

trait StreamImpl {
    fn get(&mut self) -> i32;
    fn put(&mut self, c: i32) -> i32;
}

struct StringStream {
    get: fn(&mut dyn StreamImpl) -> i32,
    put: fn(&mut dyn StreamImpl, i32) -> i32,
    string: Vec<u8>,
    pos: usize,
}

impl StreamImpl for StringStream {
    fn get(&mut self) -> i32 {
        if self.pos >= self.string.len() {
            return -1;
        }
        let c = self.string[self.pos] as i32;
        self.pos += 1;
        c
    }

    fn put(&mut self, c: i32) -> i32 {
        if c == -1 {
            self.string[self.pos] = b'\0';
            self.pos = 0;
        } else {
            self.string[self.pos] = c as u8;
            self.pos += 1;
        }
        0
    }
}

struct FileStream {
    get: fn(&mut dyn StreamImpl) -> i32,
    put: fn(&mut dyn StreamImpl, i32) -> i32,
    fp: Box<dyn Write>,
}

impl StreamImpl for FileStream {
    fn get(&mut self) -> i32 {
        -1
    }

    fn put(&mut self, c: i32) -> i32 {
        if c == -1 {
            return -1;
        }
        self.fp.write(&[c as u8]).unwrap();
        0
    }
}

fn output(out: &mut dyn StreamImpl, buf: &[u8], len: usize) {
    (out.put)(out, 128 + len as i32);
    for i in 0..len {
        (out.put)(out, buf[i] as i32);
    }
}

fn encode(in_stream: &mut dyn StreamImpl, out_stream: &mut dyn StreamImpl) {
    let mut buf = [0u8; 256];
    let mut len = 0;
    let mut repeat = false;
    let mut end = false;

    while !end {
        let c = (in_stream.get)(in_stream);
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
                (out_stream.put)(out_stream, if end { len as i32 } else { (len - 1) as i32 });
                (out_stream.put)(out_stream, buf[0] as i32);
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
    (out_stream.put)(out_stream, -1);
}

fn decode(in_stream: &mut dyn StreamImpl, out_stream: &mut dyn StreamImpl) {
    loop {
        let c = (in_stream.get)(in_stream);
        if c == -1 {
            return;
        }
        if c > 128 {
            let cnt = c - 128;
            for _ in 0..cnt {
                let c = (in_stream.get)(in_stream);
                (out_stream.put)(out_stream, c);
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
    let buf = vec![0u8; 256];
    let mut str_in = StringStream {
        get: StreamImpl::get,
        put: StreamImpl::put,
        string: b"WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWBWWWWWWWWWWWWWW".to_vec(),
        pos: 0,
    };
    let mut str_out = StringStream {
        get: StreamImpl::get,
        put: StreamImpl::put,
        string: buf,
        pos: 0,
    };
    let mut file = FileStream {
        get: StreamImpl::get,
        put: StreamImpl::put,
        fp: Box::new(io::stdout()),
    };

    encode(&mut str_in, &mut str_out);
    decode(&mut str_out, &mut file);

    Ok(())
}