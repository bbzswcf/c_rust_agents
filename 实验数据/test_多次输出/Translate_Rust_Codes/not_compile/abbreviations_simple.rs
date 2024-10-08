use std::collections::LinkedList;
use std::error::Error;
use std::fmt;
use std::mem;

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
    cmd: String,
    length: usize,
    min_len: usize,
}

impl Command {
    fn new(cmd: String, length: usize, min_len: usize) -> Self {
        Command { cmd, length, min_len }
    }
}

fn command_match(command: &Command, str: &str) -> bool {
    let olen = str.len();
    olen >= command.min_len && olen <= command.length && str.starts_with(&command.cmd)
}

fn uppercase(str: &mut [char]) {
    for c in str {
        *c = c.to_ascii_uppercase();
    }
}

fn fatal(message: &str) -> ! {
    eprintln!("{}", message);
    std::process::exit(1);
}

fn split_into_words(str: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut words = Vec::new();
    let mut word = String::new();
    let mut in_word = false;

    for c in str.chars() {
        if c.is_whitespace() {
            if in_word {
                words.push(mem::take(&mut word));
                in_word = false;
            }
        } else {
            word.push(c);
            in_word = true;
        }
    }

    if in_word {
        words.push(word);
    }

    Ok(words)
}

fn make_command_list(table: &str) -> Result<LinkedList<Command>, Box<dyn Error>> {
    let mut commands = LinkedList::new();
    let words = split_into_words(table)?;

    let mut iter = words.iter();
    while let Some(word) = iter.next() {
        let word_len = word.len();
        let mut min_len = word_len;

        if let Some(next_word) = iter.next() {
            if let Ok(num) = next_word.parse::<usize>() {
                min_len = num;
            } else {
                iter.next();
            }
        }

        let cmd = Command::new(word.to_ascii_uppercase(), word_len, min_len);
        commands.push_front(cmd);
    }

    Ok(commands)
}

fn find_command(commands: &LinkedList<Command>, word: &str) -> Option<&Command> {
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
        let mut chars: Vec<char> = word.chars().collect();
        uppercase(&mut chars);
        let word: String = chars.into_iter().collect();

        if let Some(cmd) = find_command(commands, &word) {
            print!(" {}", cmd.cmd);
        } else {
            print!(" *error*");
        }
    }

    println!();
}

fn main() -> Result<(), Box<dyn Error>> {
    let commands = make_command_list(COMMAND_TABLE)?;
    let input = "riG   rePEAT copies  put mo   rest    types   fup.    6       poweRin";
    test(&commands, input);
    Ok(())
}