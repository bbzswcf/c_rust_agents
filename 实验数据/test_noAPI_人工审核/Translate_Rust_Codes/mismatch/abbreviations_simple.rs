use std::collections::LinkedList;
use std::error::Error;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::slice;
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

// Modified: Ensure the input string's length is at least the minimum length specified for the command before performing the prefix match
fn command_match(command: &Command, str: &str) -> bool {
    let olen = str.len();
    olen >= command.min_len && command.cmd.to_str().expect("Failed to convert command to string").to_ascii_uppercase().starts_with(&str.to_ascii_uppercase())
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

fn split_into_words(str: &str) -> Result<Vec<CString>, Box<dyn Error>> {
    let mut words = Vec::with_capacity(16);
    let bytes = str.as_bytes();
    let mut begin = 0;
    while begin < bytes.len() {
        let mut i = begin;
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        begin = i;
        while i < bytes.len() && !bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        let word_len = i - begin;
        if word_len == 0 {
            break;
        }
        // Modified: Return an error immediately if creating a CString fails
        let word = CString::new(&bytes[begin..begin + word_len]).map_err(|_| "Failed to create CString from byte slice")?;
        words.push(word);
        begin += word_len;
    }
    Ok(words)
}

// Modified: Correctly parse the command table and build the command list in the correct order
fn make_command_list(table: &str) -> Result<LinkedList<Command>, Box<dyn Error>> {
    let mut commands = LinkedList::new();
    let words = split_into_words(table)?;
    let mut iter = words.into_iter();
    while let Some(word) = iter.next() {
        let word_len = word.to_bytes().len();
        let mut new_cmd = Command {
            cmd: word,
            length: word_len,
            min_len: word_len,
            next: None,
        };
        if let Some(next_word) = iter.next() {
            // Modified: Correctly handle the parsing of the next word as a number
            if let Ok(min_len_str) = str::from_utf8(next_word.to_bytes()) {
                if let Ok(min_len) = min_len_str.parse::<usize>() {
                    new_cmd.min_len = min_len;
                }
            }
        }
        commands.push_back(new_cmd); // Use push_back to maintain the order
    }
    Ok(commands)
}

// Modified: Ensure the command list is correctly iterated and the correct command is returned if found
fn find_command<'a>(commands: &'a LinkedList<Command>, word: &'a str) -> Option<&'a Command> {
    for cmd in commands {
        if command_match(cmd, word) {
            return Some(cmd);
        }
    }
    None
}

// Modified: Ensure the input words are correctly converted to strings and handled
fn test(commands: &LinkedList<Command>, input: &str) {
    println!(" input: {}", input);
    print!("output:");
    let words = match split_into_words(input) {
        Ok(words) => words,
        Err(e) => {
            eprintln!("Failed to split input into words: {}", e);
            return;
        }
    };
    for word in words {
        let word_str = match word.to_str() {
            Ok(s) => s,
            Err(_) => {
                print!(" *error*");
                continue;
            }
        };
        let cmd_ptr = find_command(commands, word_str);
        print!(" {}", cmd_ptr.map_or("*error*", |cmd| cmd.cmd.to_str().unwrap()));
    }
    println!();
}

fn main() -> Result<(), Box<dyn Error>> {
    let commands = make_command_list(COMMAND_TABLE)?;
    let input = "riG   rePEAT copies  put mo   rest    types   fup.    6       poweRin";
    test(&commands, input);
    Ok(())
}