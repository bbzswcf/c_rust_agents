use std::process::exit;
// Removed: Unused import
// use std::ptr::copy_nonoverlapping;
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

fn uppercase(str: &mut [u8]) -> &str {
    for c in str.iter_mut() {
        *c = (*c as char).to_ascii_uppercase() as u8;
    }
    unsafe { from_utf8_unchecked(str) }
}

fn get_min_length(str: &str) -> usize {
    str.chars()
        .take_while(|c| c.is_ascii_uppercase())
        .count()
}

fn fatal(message: &str) -> ! {
    eprintln!("{}", message);
    exit(1);
}

fn split_into_words(str: &str) -> Vec<String> {
    let mut words = Vec::with_capacity(16);
    let mut start = 0;
    let bytes = str.as_bytes();
    let len = bytes.len();

    while start < len {
        while start < len && bytes[start].is_ascii_whitespace() {
            start += 1;
        }
        let mut end = start;
        while end < len && !bytes[end].is_ascii_whitespace() {
            end += 1;
        }
        if start < end {
            let word = String::from_utf8_lossy(&bytes[start..end]).to_string();
            words.push(word);
        }
        start = end;
    }
    words
}

fn make_command_list(table: &str) -> Option<Box<Command>> {
    let words = split_into_words(table);
    let mut cmd = None;

    for word in words.into_iter().rev() {
        let word_len = word.len();
        // Modified: Removed `mut` keyword since `new_cmd` does not need to be mutable
        let new_cmd = Box::new(Command {
            // Modified: Cloned `word` before moving it to avoid borrowing after move
            cmd: uppercase(&mut word.clone().into_bytes()).to_string(),
            length: word_len,
            // Modified: Cloned `word` before moving it to avoid borrowing after move
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

// Modified: Introduced a named lifetime parameter to specify the relationship between the input references and the output reference
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
        let word = uppercase(&mut word.into_bytes()).to_string();
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