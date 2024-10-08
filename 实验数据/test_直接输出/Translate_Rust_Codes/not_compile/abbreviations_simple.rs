use std::process::exit;
use std::ptr::copy_nonoverlapping;
use std::str::FromStr;

const COMMAND_TABLE: &str = "add 1  alter 3  backup 2  bottom 1  Cappend 2  change 1  Schange  Cinsert 2  Clast 3 \
                             compress 4 copy 2 count 3 Coverlay 3 cursor 3  delete 3 Cdelete 2  down 1  duplicate \
                             3 xEdit 1 expand 3 extract 3  find 1 Nfind 2 Nfindup 6 NfUP 3 Cfind 2 findUP 3 fUP 2 \
                             forward 2  get  help 1 hexType 4  input 1 powerInput 3  join 1 split 2 spltJOIN load \
                             locate 1 Clocate 2 lowerCase 3 upperCase 3 Lprefix 2  macro  merge 2 modify 3 move 2 \
                             msg  next 1 overlay 1 parse preserve 4 purge 3 put putD query 1 quit  read recover 3 \
                             refresh renum 3 repeat 3 replace 1 Creplace 2 reset 3 restore 4 rgtLEFT right 2 left \
                             2  save  set  shift 2  si  sort  sos  stack 3 status 4 top  transfer 3  type 1  up 1";

struct Command {
    cmd: String,
    length: usize,
    min_len: usize,
    next: Option<Box<Command>>,
}

fn command_match(command: &Command, str: &str) -> bool {
    let olen = str.len();
    olen >= command.min_len && olen <= command.length && str.starts_with(&command.cmd[..olen])
}

fn uppercase(str: &mut [char]) {
    for c in str {
        *c = c.to_ascii_uppercase();
    }
}

fn fatal(message: &str) -> ! {
    eprintln!("{}", message);
    exit(1);
}

fn split_into_words(str: &str) -> Vec<String> {
    let mut words = Vec::with_capacity(16);
    let chars: Vec<char> = str.chars().collect();
    let len = chars.len();
    let mut begin = 0;

    while begin < len {
        while begin < len && chars[begin].is_whitespace() {
            begin += 1;
        }
        let mut i = begin;
        while i < len && !chars[i].is_whitespace() {
            i += 1;
        }
        let word_len = i - begin;
        if word_len == 0 {
            break;
        }
        let mut word = vec![' '; word_len];
        unsafe {
            copy_nonoverlapping(chars.as_ptr().add(begin), word.as_mut_ptr(), word_len);
        }
        begin += word_len;
        words.push(word.iter().collect());
    }
    words
}

fn make_command_list(table: &str) -> Option<Box<Command>> {
    let words = split_into_words(table);
    let mut cmd = None;

    for (i, word) in words.iter().enumerate() {
        let mut new_cmd = Box::new(Command {
            cmd: word.clone(),
            length: word.len(),
            min_len: word.len(),
            next: cmd,
        });

        if i + 1 < words.len() {
            if let Ok(min_len) = u64::from_str(&words[i + 1]) {
                new_cmd.min_len = min_len as usize;
                new_cmd.cmd = words[i].chars().map(|c| c.to_ascii_uppercase()).collect();
            }
        }

        cmd = Some(new_cmd);
    }
    cmd
}

fn free_command_list(cmd: Option<Box<Command>>) {
    let mut current = cmd;
    while let Some(mut node) = current {
        current = node.next.take();
    }
}

fn find_command(commands: &Option<Box<Command>>, word: &str) -> Option<&Command> {
    let mut current = commands;
    while let Some(cmd) = current {
        if command_match(cmd, word) {
            return Some(cmd);
        }
        current = &cmd.next;
    }
    None
}

fn test(commands: &Option<Box<Command>>, input: &str) {
    println!(" input: {}", input);
    print!("output:");
    let words = split_into_words(input);
    for word in words {
        let mut chars: Vec<char> = word.chars().collect();
        uppercase(&mut chars);
        let word: String = chars.iter().collect();
        if let Some(cmd) = find_command(commands, &word) {
            print!(" {}", cmd.cmd);
        } else {
            print!(" *error*");
        }
    }
    println!();
}

fn main() {
    let commands = make_command_list(COMMAND_TABLE);
    let input = "riG   rePEAT copies  put mo   rest    types   fup.    6       poweRin";
    test(&commands, input);
    free_command_list(commands);
}