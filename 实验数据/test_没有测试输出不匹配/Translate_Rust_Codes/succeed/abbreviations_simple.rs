// Modified: Removed unused import
// use std::process::exit;

const COMMAND_TABLE: &str =
    "add 1  alter 3  backup 2  bottom 1  Cappend 2  change 1  Schange  Cinsert 2  Clast 3 \
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
    olen >= command.min_len && olen <= command.length && str.chars().take(olen).collect::<String>() == command.cmd.chars().take(olen).collect::<String>()
}

fn uppercase(str: &mut String) {
    *str = str.to_uppercase();
}

fn fatal(message: &str) {
    eprintln!("{}", message);
    std::process::exit(1);
}

fn split_into_words(str: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut start = 0;
    let len = str.len();
    while start < len {
        while start < len && str.get(start..start+1).and_then(|s| s.chars().next()).unwrap_or(' ').is_whitespace() {
            start += 1;
        }
        let mut end = start;
        while end < len && !str.get(end..end+1).and_then(|s| s.chars().next()).unwrap_or(' ').is_whitespace() {
            end += 1;
        }
        if start < end {
            words.push(str.get(start..end).map(|s| s.to_string()).unwrap_or_default());
        }
        start = end;
    }
    words
}

fn make_command_list(table: &str) -> Option<Box<Command>> {
    let words = split_into_words(table);
    let mut cmd = None;
    let mut i = 0;
    while i < words.len() {
        let word = &words[i];
        // Modified: Declare `new_cmd` as mutable to allow mutable borrowing and assignment
        let mut new_cmd = Box::new(Command {
            cmd: String::from(word),
            length: word.len(),
            min_len: word.len(),
            next: cmd,
        });
        uppercase(&mut new_cmd.cmd);
        if i + 1 < words.len() {
            if let Ok(min_len) = words[i + 1].parse::<usize>() {
                new_cmd.min_len = min_len;
                i += 1;
            }
        }
        cmd = Some(new_cmd);
        i += 1;
    }
    cmd
}

fn free_command_list(cmd: Option<Box<Command>>) {
    let mut current = cmd;
    while let Some(mut node) = current {
        current = node.next.take();
    }
}

fn find_command<'a>(commands: &'a Option<Box<Command>>, word: &'a str) -> Option<&'a Command> {
    let mut current = commands.as_ref();
    while let Some(cmd) = current {
        if command_match(cmd, word) {
            return Some(cmd);
        }
        current = cmd.next.as_ref();
    }
    None
}

fn test(commands: &Option<Box<Command>>, input: &str) {
    println!(" input: {}", input);
    print!("output:");
    let words = split_into_words(input);
    for word in words {
        let mut word = String::from(word);
        uppercase(&mut word);
        let cmd_ptr = find_command(commands, &word);
        print!(" {}", cmd_ptr.map_or("*error*", |cmd| &cmd.cmd));
    }
    println!();
}

fn main() {
    let commands = make_command_list(COMMAND_TABLE);
    let input = "riG   rePEAT copies  put mo   rest    types   fup.    6       poweRin";
    test(&commands, input);
    free_command_list(commands);
}