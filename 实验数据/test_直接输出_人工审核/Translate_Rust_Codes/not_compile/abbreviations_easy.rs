use std::process::exit;
use std::ptr::copy;
use std::str::from_utf8_unchecked;

const COMMAND_TABLE: &str =
    "Add ALTer  BAckup Bottom  CAppend Change SCHANGE  CInsert CLAst COMPress COpy \
     COUnt COVerlay CURsor DELete CDelete Down DUPlicate Xedit EXPand EXTract Find \
     NFind NFINDUp NFUp CFind FINdup FUp FOrward GET Help HEXType Input POWerinput \
     Join SPlit SPLTJOIN  LOAD  Locate CLocate  LOWercase UPPercase  LPrefix MACRO \
     MErge MODify MOve MSG Next Overlay PARSE PREServe PURge PUT PUTD  Query  QUIT \
     READ  RECover REFRESH RENum REPeat  Replace CReplace  RESet  RESTore  RGTLEFT \
     RIght LEft  SAVE  SET SHift SI  SORT  SOS  STAck STATus  TOP TRAnsfer Type Up";

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

fn get_min_length(str: &str) -> usize {
    str.chars().take_while(|c| c.is_ascii_uppercase()).count()
}

fn fatal(message: &str) {
    eprintln!("{}", message);
    exit(1);
}

fn split_into_words(str: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut start = 0;
    let chars: Vec<char> = str.chars().collect();
    let len = chars.len();

    while start < len {
        while start < len && chars[start].is_whitespace() {
            start += 1;
        }
        let mut end = start;
        while end < len && !chars[end].is_whitespace() {
            end += 1;
        }
        if start < end {
            let word: String = chars[start..end].iter().collect();
            words.push(word);
        }
        start = end;
    }
    words
}

fn make_command_list(table: &str) -> Option<Box<Command>> {
    let words = split_into_words(table);
    let mut cmd: Option<Box<Command>> = None;

    for word in words {
        let word_len = word.len();
        let mut chars: Vec<char> = word.chars().collect();
        uppercase(&mut chars);
        let upper_word: String = chars.iter().collect();

        let new_cmd = Box::new(Command {
            cmd: upper_word,
            length: word_len,
            min_len: get_min_length(&word),
            next: cmd,
        });
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
        let mut chars: Vec<char> = word.chars().collect();
        uppercase(&mut chars);
        let upper_word: String = chars.iter().collect();
        let cmd_ptr = find_command(commands, &upper_word);
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