use std::process::exit;
use std::alloc::{alloc, Layout};
use std::ptr::copy_nonoverlapping;
use std::str::from_utf8_unchecked;

fn quib(strs: &[&str]) -> Box<str> {
    let size = strs.len();
    let mut len = 3 + if size > 1 { 2 * size + 1 } else { 0 };

    for s in strs {
        len += s.len();
    }

    let layout = Layout::array::<u8>(len).unwrap();
    let ptr = unsafe { alloc(layout) };
    if ptr.is_null() {
        eprintln!("Can't allocate memory!");
        exit(1);
    }

    let mut offset = 0;
    unsafe {
        copy_nonoverlapping(b"{" as *const u8, ptr, 1);
        offset += 1;

        match size {
            0 => {}
            1 => {
                copy_nonoverlapping(strs[0].as_ptr(), ptr.add(offset), strs[0].len());
                offset += strs[0].len();
            }
            _ => {
                for i in 0..size - 1 {
                    let s = strs[i];
                    copy_nonoverlapping(s.as_ptr(), ptr.add(offset), s.len());
                    offset += s.len();
                    if i < size - 2 {
                        copy_nonoverlapping(b", " as *const u8, ptr.add(offset), 2);
                        offset += 2;
                    } else {
                        copy_nonoverlapping(b" and " as *const u8, ptr.add(offset), 5);
                        offset += 5;
                    }
                }
                let last = strs[size - 1];
                copy_nonoverlapping(last.as_ptr(), ptr.add(offset), last.len());
                offset += last.len();
            }
        }

        copy_nonoverlapping(b"}" as *const u8, ptr.add(offset), 1);
        offset += 1;

        let result = from_utf8_unchecked(std::slice::from_raw_parts(ptr, offset));
        Box::from_raw(result as *const str as *mut str)
    }
}

fn main() {
    let test = ["ABC", "DEF", "G", "H"];

    for i in 0..5 {
        let s = quib(&test[..i]);
        println!("{}", s);
    }
}