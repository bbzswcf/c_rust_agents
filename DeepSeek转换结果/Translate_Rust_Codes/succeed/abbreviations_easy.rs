use std::process::exit;
use std::ptr::copy_nonoverlapping;
use std::str::SplitWhitespace;

const COMMAND_TABLE: &str =
    "Add ALTer  BAckup Bottom  CAppend Change SCHANGE  CInsert CLAst COMPress COpy \
     COunt COVerlay CURsor DELete CDelete Down DUPlicate Xedit EXPand EXTract Find \
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
    olen >= command.min_len && olen <= command.length && str.chars().take(olen).collect::<String>() == command.cmd.chars().take(olen).collect::<String>()
}

fn uppercase(str: &mut [char]) {
    for c in str {
        *c = c.to_uppercase().next().unwrap();
    }
}

fn get_min_length(str: &str) -> usize {
    str.chars().take_while(|c| c.is_uppercase()).count()
}

fn fatal(message: &str) {
    eprint!("{}\n", message);
    exit(1);
}

fn xmalloc<T>(n: usize) -> Vec<T> {
    Vec::with_capacity(n)
}

fn xrealloc<T>(mut vec: Vec<T>, n: usize) -> Vec<T> {
    vec.reserve(n);
    vec
}

fn split_into_words(str: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut capacity = 16;
    words.reserve(capacity);
    let mut begin = 0;
    for (i, c) in str.chars().enumerate() {
        if c.is_whitespace() {
            if i > begin {
                let word = str[begin..i].to_string();
                if words.len() == capacity {
                    capacity *= 2;
                    words.reserve(capacity);
                }
                words.push(word);
            }
            begin = i + 1;
        }
    }
    if begin < str.len() {
        words.push(str[begin..].to_string());
    }
    words
}

fn make_command_list(table: &str) -> Option<Box<Command>> {
    let words = split_into_words(table);
    let mut cmd = None;
    for word in words {
        let word_len = word.len();
        let mut new_cmd = Box::new(Command {
            cmd: word.chars().map(|c| c.to_uppercase().next().unwrap()).collect(),
            length: word_len,
            min_len: get_min_length(&word),
            next: None,
        });
        new_cmd.next = cmd;
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
    print!(" input: {}\noutput:", input);
    let words = split_into_words(input);
    for word in words {
        let mut chars: Vec<char> = word.chars().collect();
        uppercase(&mut chars);
        let word: String = chars.into_iter().collect();
        if let Some(cmd_ptr) = find_command(commands, &word) {
            print!(" {}", cmd_ptr.cmd);
        } else {
            print!(" *error*");
        }
    }
    print!("\n");
}

fn main() {
    let commands = make_command_list(COMMAND_TABLE);
    let input = "riG   rePEAT copies  put mo   rest    types   fup.    6       poweRin";
    test(&commands, input);
    free_command_list(commands);
}