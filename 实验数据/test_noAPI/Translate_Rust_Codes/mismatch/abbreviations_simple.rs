use std::collections::LinkedList;
use std::error::Error;
use std::ffi::CString;
use std::mem;
use std::str;

const COMMAND_TABLE: &str = "add 1  alter 3  backup 2  bottom 1  Cappend 2  change 1  Schange  Cinsert 2  Clast 3 \
                             compress 4 copy 2 count 3 Coverlay 3 cursor 3  delete 3 Cdelete 2  down 1  duplicate \
                             3 xEdit 1 expand 3 extract 3  find 1 Nfind 2 Nfindup 6 NfUP 3 Cfind 2 findUP 3 fUP 2 \
                             forward 2  get  help 1 hexType 4  input 1 powerInput 3  join 1 split 2 spltJOIN load \
                             locate 1 Clocate 2 lowerCase 3 upperCase 3 Lprefix 2  macro  merge 2 modify 3 move 2 \
                             msg  next 1 overlay 1 parse preserve 4 purge 3 put putD query 1 quit  read recover 3 \
                             refresh renum 3 repeat 3 replace 1 Creplace 2 reset 3 restore 4 rgtLEFT right 2 left \
                             2  save  set  shift 2  si  sort  sos  stack 3 status 4 top  transfer 3  type 1  up 1";

#[derive(Debug)]
struct Command {
    cmd: CString,
    length: usize,
    min_len: usize,
    next: Option<Box<Command>>,
}

fn command_match(command: &Command, str: &str) -> bool {
    let olen = str.len();
    // Modified: Check if the input string matches the command string up to its actual length
    olen >= command.min_len && olen <= command.length && str.to_lowercase().starts_with(&command.cmd.to_str().unwrap().to_lowercase()[..olen.min(command.length)])
}

fn uppercase(str: &mut [u8]) {
    for c in str {
        *c = c.to_ascii_uppercase();
    }
}

fn fatal(message: &str) -> ! {
    eprintln!("{}", message);
    std::process::exit(1);
}

fn xmalloc<T>(n: usize) -> *mut T {
    let ptr = unsafe { std::alloc::alloc(std::alloc::Layout::array::<T>(n).unwrap()) };
    if ptr.is_null() {
        fatal("Out of memory");
    }
    ptr as *mut T
}

fn xrealloc<T>(p: *mut T, n: usize) -> *mut T {
    let ptr = unsafe { std::alloc::realloc(p as *mut u8, std::alloc::Layout::array::<T>(1).unwrap(), n * mem::size_of::<T>()) };
    if ptr.is_null() {
        fatal("Out of memory");
    }
    ptr as *mut T
}

fn split_into_words(str: &str) -> Result<Vec<CString>, Box<dyn Error>> {
    let mut words = Vec::with_capacity(16);
    let bytes = str.as_bytes();
    let len = bytes.len();
    let mut begin = 0;

    while begin < len {
        let mut i = begin;
        while i < len && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        begin = i;
        while i < len && !bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        let word_len = i - begin;
        if word_len == 0 {
            break;
        }
        let word = unsafe { CString::from_vec_unchecked(bytes[begin..begin + word_len].to_vec()) };
        begin += word_len;
        words.push(word);
    }
    Ok(words)
}

fn make_command_list(table: &str) -> Result<LinkedList<Command>, Box<dyn Error>> {
    let mut commands = LinkedList::new();
    let words = split_into_words(table)?;
    let mut iter = words.into_iter();

    while let Some(word) = iter.next() {
        let word_len = word.as_bytes().len();
        let mut min_len = word_len;
        if let Some(next) = iter.next() {
            if let Ok(num) = next.to_str().unwrap().parse::<usize>() {
                min_len = num;
            } else {
                iter.next(); // Skip the invalid number and continue with the next word
            }
        }
        let cmd = Command {
            cmd: word,
            length: word_len,
            min_len,
            next: None,
        };
        commands.push_front(cmd);
    }

    Ok(commands)
}

fn free_command_list(commands: LinkedList<Command>) {
    for cmd in commands {
        drop(cmd.cmd);
    }
}

fn find_command<'a>(commands: &'a LinkedList<Command>, word: &'a str) -> Option<&'a Command> { // Modified: Introduced named lifetime parameter
    for cmd in commands {
        if command_match(cmd, word) {
            return Some(cmd);
        }
    }
    None
}

fn test(commands: &LinkedList<Command>, input: &str) {
    println!(" input: {}", input);
    print!("output:");
    let words = split_into_words(input).unwrap();
    for word in words {
        let mut bytes = word.into_bytes();
        uppercase(&mut bytes);
        let word_str = unsafe { str::from_utf8_unchecked(&bytes) };
        let cmd_ptr = find_command(commands, word_str);
        print!(" {}", cmd_ptr.map_or("*error*", |cmd| cmd.cmd.to_str().unwrap()));
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