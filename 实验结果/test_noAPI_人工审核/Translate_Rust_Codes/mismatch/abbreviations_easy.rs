use std::error::Error;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::slice;
use std::str;

const COMMAND_TABLE: &str = "Add ALTer  BAckup Bottom  CAppend Change SCHANGE  CInsert CLAst COMPress COpy \
                             COunt COVerlay CURsor DELete CDelete Down DUPlicate Xedit EXPand EXTract Find \
                             NFind NFINDUp NFUp CFind FINdup FUp FOrward GET Help HEXType Input POWerinput \
                             Join SPlit SPLTJOIN  LOAD  Locate CLocate  LOWercase UPPercase  LPrefix MACRO \
                             MErge MODify MOve MSG Next Overlay PARSE PREServe PURge PUT PUTD  Query  QUIT \
                             READ  RECover REFRESH RENum REPeat  Replace CReplace  RESet  RESTore  RGTLEFT \
                             RIght LEft  SAVE  SET SHift SI  SORT  SOS  STAck STATus  TOP TRAnsfer Type Up";

#[derive(Debug)]
struct Command {
    cmd: CString,
    length: usize,
    min_len: usize,
    next: Option<Box<Command>>,
}

// Modified: Corrected command matching logic to be case-insensitive and handle command prefix
fn command_match(command: &Command, str: &str) -> bool {
    let olen = str.len();
    olen >= command.min_len && str.to_ascii_uppercase().starts_with(&command.cmd.to_str().unwrap()[..command.min_len])
}

// Modified: Ensured the input vector is valid for creating a CString without null bytes
fn uppercase(str: &mut [u8]) -> CString {
    for c in str.iter_mut() {
        *c = (*c as char).to_ascii_uppercase() as u8;
    }
    CString::new(str.to_vec()).unwrap() // Modified: Used CString::new with a checked conversion
}

fn get_min_length(str: &str) -> usize {
    str.chars()
        .take_while(|&c| c.is_ascii_uppercase())
        .count()
}

fn fatal(message: &str) -> ! {
    eprintln!("{}", message);
    std::process::exit(1);
}

fn xmalloc<T>(n: usize) -> *mut T {
    let ptr = unsafe { std::alloc::alloc(std::alloc::Layout::array::<T>(n).unwrap()) };
    if ptr.is_null() {
        fatal("Out of memory"); // Modified: Added check to ensure ptr is not null
    }
    ptr as *mut T
}

fn xrealloc<T>(p: *mut T, n: usize) -> *mut T {
    let ptr = unsafe { std::alloc::realloc(p as *mut u8, std::alloc::Layout::array::<T>(1).unwrap(), n * mem::size_of::<T>()) };
    if ptr.is_null() {
        fatal("Out of memory"); // Modified: Added check to ensure ptr is not null
    }
    ptr as *mut T
}

// Modified: Corrected handling of multiple consecutive whitespace characters
fn split_into_words(str: &str) -> Result<Vec<CString>, Box<dyn Error>> {
    let mut words = Vec::with_capacity(16);
    let mut start = 0;
    let bytes = str.as_bytes();
    while start < bytes.len() {
        while start < bytes.len() && bytes[start].is_ascii_whitespace() {
            start += 1;
        }
        let mut end = start;
        while end < bytes.len() && !bytes[end].is_ascii_whitespace() {
            end += 1;
        }
        if start < end {
            let word = CString::new(&bytes[start..end]).unwrap(); // Modified: Used CString::new with a checked conversion
            words.push(word);
            start = end;
        }
    }
    Ok(words)
}

// Modified: Removed unnecessary cloning of CString
fn make_command_list(table: &str) -> Result<Box<Command>, Box<dyn Error>> {
    let words = split_into_words(table)?;
    let mut cmd = None;
    for word in words.into_iter().rev() {
        let word_len = word.as_bytes().len();
        let mut word_bytes = word.into_bytes();
        let uppercased_word = uppercase(&mut word_bytes);
        let new_cmd = Box::new(Command {
            cmd: uppercased_word.clone(), // Modified: Cloned uppercased_word to avoid borrow of moved value
            length: word_len,
            min_len: get_min_length(uppercased_word.to_str().unwrap_or_else(|_| { eprintln!("Invalid UTF-8 sequence"); "" })), // Modified: Handled potential invalid UTF-8 conversion with an empty string default
            next: cmd,
        });
        cmd = Some(new_cmd);
    }
    Ok(cmd.unwrap())
}

// Modified: Corrected handling of command list freeing
fn free_command_list(cmd: Box<Command>) {
    let mut current = Some(cmd);
    while let Some(mut cmd) = current {
        current = cmd.next.take();
        // Optionally, free the CString here if necessary
    }
}

// Modified: Corrected handling of command list in find_command
fn find_command<'a>(commands: &'a Command, word: &'a str) -> Option<&'a Command> {
    let mut current = Some(commands);
    while let Some(cmd) = current {
        if command_match(cmd, word) {
            return Some(cmd);
        }
        current = cmd.next.as_deref();
    }
    None
}

// Modified: Removed unnecessary cloning of word_bytes
fn test(commands: &Command, input: &str) {
    println!(" input: {}", input);
    print!("output:");
    let words = split_into_words(input).unwrap();
    for word in words {
        let _word_str = word.to_str().unwrap_or_else(|_| { eprintln!("Invalid UTF-8 sequence"); "*error*" }); // Modified: Handled potential invalid UTF-8 conversion and prefixed with underscore to suppress unused variable warning
        let mut word_bytes = word.into_bytes();
        let uppercased_word = uppercase(&mut word_bytes);
        let uppercased_word_str = uppercased_word.to_str().unwrap_or_else(|_| { eprintln!("Invalid UTF-8 sequence"); "*error*" }); // Modified: Handled potential invalid UTF-8 conversion
        let cmd_ptr = find_command(commands, uppercased_word_str);
        print!(" {}", cmd_ptr.map_or("*error*", |cmd| cmd.cmd.to_str().unwrap_or_else(|_| { eprintln!("Invalid UTF-8 sequence"); "*error*" }))); // Modified: Handled potential invalid UTF-8 conversion
    }
    println!();
}

fn main() -> Result<(), Box<dyn Error>> {
    let commands = make_command_list(COMMAND_TABLE)?;
    let input = "riG   rePEAT copies  put mo   rest    types   fup.    6       poweRin";
    test(&commands, input);
    free_command_list(commands);
    Ok(())
}