use std::io::{self, Write};
use std::any::Any;

trait Stream: Any {
    fn get(&mut self) -> i32;
    fn put(&mut self, c: i32) -> i32;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

struct StringStream {
    string: String,
    pos: usize,
}

impl StringStream {
    fn new(s: String) -> Self {
        StringStream { string: s, pos: 0 }
    }
}

impl Stream for StringStream {
    fn get(&mut self) -> i32 {
        let c = self.string.chars().nth(self.pos).map(|ch| ch as u32 as i32).unwrap_or(-1); // Modified: Cast char to u32 first, then to i32
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
            self.string.push(char::from_u32(c as u32).unwrap_or('\0')); // Modified: Use char::from_u32 to safely cast i32 to char
            self.pos += 1;
        }
        0
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

struct FileStream {
    fp: Box<dyn Write>,
}

impl FileStream {
    fn new(fp: Box<dyn Write>) -> Self {
        FileStream { fp }
    }
}

impl Stream for FileStream {
    fn get(&mut self) -> i32 {
        -1 // FileStream does not support get operation
    }

    fn put(&mut self, c: i32) -> i32 {
        if c >= 0 && c <= 255 { // Modified: Ensure c is within valid u8 range
            if let Err(e) = self.fp.write(&[c as u8]) {
                eprintln!("Write error: {}", e);
            }
        }
        0
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

fn sget(in_stream: &mut dyn Stream) -> i32 {
    if let Some(s) = in_stream.as_any_mut().downcast_mut::<StringStream>() {
        let c = s.string.chars().nth(s.pos).map(|ch| ch as u32 as i32).unwrap_or(-1); // Modified: Cast char to u32 first, then to i32
        if c == -1 {
            return -1;
        }
        s.pos += 1;
        c
    } else {
        -1 // Modified: Return -1 if not a StringStream
    }
}

fn sput(out_stream: &mut dyn Stream, c: i32) -> i32 {
    if let Some(s) = out_stream.as_any_mut().downcast_mut::<StringStream>() {
        if c == -1 {
            s.string.push('\0');
            s.pos = 0;
        } else {
            s.string.push(char::from_u32(c as u32).unwrap_or('\0')); // Modified: Use char::from_u32 to safely cast i32 to char
            s.pos += 1;
        }
        0
    } else {
        -1 // Modified: Return -1 if not a StringStream
    }
}

fn file_put(out_stream: &mut dyn Stream, c: i32) -> i32 {
    if let Some(f) = out_stream.as_any_mut().downcast_mut::<FileStream>() {
        if c >= 0 && c <= 255 { // Modified: Ensure c is within valid u8 range
            if let Err(e) = f.fp.write(&[c as u8]) {
                eprintln!("Write error: {}", e);
            }
        }
        0
    } else {
        -1 // Modified: Return -1 if not a FileStream
    }
}

fn output(out: &mut dyn Stream, buf: &[u8], len: usize) {
    out.put(128 + len as i32);
    for i in 0..len {
        out.put(buf[i] as i32); // Modified: Ensure the cast from u8 to i32 is explicit and correct
    }
}

fn encode(in_stream: &mut dyn Stream, out_stream: &mut dyn Stream) {
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
                    buf[0] = buf[len - 1]; // Modified: Split the assignment to avoid type mismatch
                    buf[1] = buf[len - 1]; // Modified: Split the assignment to avoid type mismatch
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

fn decode(in_stream: &mut dyn Stream, out_stream: &mut dyn Stream) {
    loop {
        let c = in_stream.get();
        if c == -1 {
            break; // Modified: Added break condition to avoid infinite loop
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
    // Removed: Unnecessary initialization of buf
    let mut str_in = StringStream::new("WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWBWWWWWWWWWWWWWW".to_string());
    let mut str_out = StringStream::new(String::new());
    let mut file = FileStream::new(Box::new(io::stdout()));

    encode(&mut str_in, &mut str_out);
    decode(&mut str_out, &mut file);
}