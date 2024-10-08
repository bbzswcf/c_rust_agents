use std::error::Error;
use std::ffi::CString;
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

fn command_match(command: &Command, str: &str) -> bool {
    // Modified: Perform case-insensitive comparison and ensure the input string matches the command string from the start
    let olen = str.len();
    olen >= command.min_len && olen <= command.length && str.eq_ignore_ascii_case(&command.cmd.to_str().unwrap_or("")[..olen])
}

fn uppercase(str: &[u8]) -> CString {
    let mut upper_str = str.to_vec();
    for c in &mut upper_str {
        *c = c.to_ascii_uppercase();
    }
    CString::new(upper_str).unwrap()
}

fn get_min_length(str: &str) -> usize {
    // Modified: Calculate minimum length based on actual command string length
    str.len()
}

fn fatal(message: &str) -> ! {
    eprintln!("{}", message);
    std::process::exit(1);
}

fn xmalloc<T>(n: usize) -> *mut T {
    let layout = std::alloc::Layout::array::<T>(n).unwrap();
    let ptr = unsafe { std::alloc::alloc(layout) };
    if ptr.is_null() {
        fatal("Out of memory");
    }
    ptr as *mut T
}

fn xrealloc<T>(p: *mut T, n: usize) -> *mut T {
    let layout = std::alloc::Layout::array::<T>(n).unwrap();
    let ptr = unsafe { std::alloc::realloc(p as *mut u8, layout, layout.size()) };
    if ptr.is_null() {
        fatal("Out of memory");
    }
    ptr as *mut T
}

fn split_into_words(str: &str) -> Result<Vec<CString>, Box<dyn Error>> {
    let mut words = Vec::new();
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
            let word = CString::new(bytes[start..end].to_vec())?;
            words.push(word);
            start = end;
        }
    }
    Ok(words)
}

fn make_command_list(table: &str) -> Result<Box<Command>, Box<dyn Error>> {
    let words = split_into_words(table)?;
    let mut cmd = None;
    for word in words { // Modified: Removed .into_iter().rev() to maintain original order
        let word_len = word.as_bytes().len();
        let word_str = word.to_str().map_err(|_| "Invalid UTF-8 sequence")?;
        let new_cmd = Box::new(Command {
            cmd: word.clone(),
            length: word_len,
            min_len: get_min_length(word_str),
            next: cmd,
        });
        cmd = Some(new_cmd);
    }
    cmd.ok_or_else(|| "Failed to create command list".into())
}

fn free_command_list(cmd: Box<Command>) {
    let mut current = Some(cmd);
    while let Some(mut cmd) = current {
        current = cmd.next.take();
    }
}

fn find_command<'a>(commands: &'a Command, word: &str) -> Option<&'a Command> {
    let mut current = Some(commands);
    while let Some(cmd) = current {
        if command_match(cmd, word) {
            return Some(cmd);
        }
        current = cmd.next.as_deref();
    }
    None
}

fn test(commands: &Command, input: &str) {
    println!(" input: {}", input);
    print!("output:");
    let words = match split_into_words(input) {
        Ok(words) => words,
        Err(e) => {
            eprintln!("Error splitting input: {}", e);
            return;
        }
    };
    for word in words {
        let word_str = word.to_str().map_err(|_| "Invalid UTF-8 sequence").ok();
        let upper_word = uppercase(word.as_bytes());
        let cmd_ptr = find_command(commands, upper_word.to_str().unwrap());
        print!(" {}", cmd_ptr.map_or("*error*", |cmd| cmd.cmd.to_str().unwrap_or("*invalid*")));
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