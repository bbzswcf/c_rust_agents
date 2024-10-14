use std::error::Error;
use std::fmt;
use std::str;

#[derive(Debug)]
struct MemoryAllocationError;

impl fmt::Display for MemoryAllocationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Can't allocate memory!")
    }
}

impl Error for MemoryAllocationError {}

fn quib(strs: &[&str]) -> Result<String, MemoryAllocationError> {
    let size = strs.len();
    let mut len = 3 + if size > 1 { 2 * size + 1 } else { 0 };

    for s in strs {
        len += s.len();
    }

    let mut s = String::with_capacity(len);
    s.push('{');

    match size {
        0 => {}
        1 => s.push_str(strs[0]),
        _ => {
            for i in 0..size - 1 {
                s.push_str(strs[i]);
                if i < size - 2 {
                    s.push_str(", ");
                } else {
                    s.push_str(" and ");
                }
            }
            s.push_str(strs[size - 1]);
        }
    }

    s.push('}');
    Ok(s)
}

fn main() -> Result<(), Box<dyn Error>> {
    let test = ["ABC", "DEF", "G", "H"];

    for i in 0..5 {
        let s = quib(&test[..i])?;
        println!("{}", s);
    }

    Ok(())
}