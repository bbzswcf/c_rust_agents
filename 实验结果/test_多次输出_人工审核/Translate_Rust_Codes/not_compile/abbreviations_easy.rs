use std::collections::LinkedList;
use std::error::Error;
use std::fmt;
use std::mem;

const COMMAND_TABLE: &str = "Add ALTer  BAckup Bottom  CAppend Change SCHANGE  CInsert CLAst COMPress COpy \
                             COUnt COVerlay CURsor DELete CDelete Down DUPlicate Xedit EXPand EXTract Find \
                             NFind NFINDUp NFUp CFind FINdup FUp FOrward GET Help HEXType Input POWerinput \
                             Join SPlit SPLTJOIN  LOAD  Locate CLocate  LOWercase UPPercase  LPrefix MACRO \
                             MErge MODify MOve MSG Next Overlay PARSE PREServe PURge PUT PUTD  Query  QUIT \
                             READ  RECover REFRESH RENum REPeat  Replace CReplace  RESet  RESTore  RGTLEFT \
                             RIght LEft  SAVE  SET SHift SI  SORT  SOS  STAck STATus  TOP TRAnsfer Type Up";

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
    olen >= command.min_len && olen <= command.length && str.starts_with(&command.cmd[..olen])
}

fn uppercase(str: &mut str) {
    str.make_ascii_uppercase();
}

fn get_min_length(str: &str) -> usize {
    str.chars().take_while(|c| c.is_ascii_uppercase()).count()
}

fn fatal(message: &str) -> ! {
    eprintln!("{}", message);
    std::process::exit(1);
}

fn split_into_words(str: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut words = Vec::new();
    let mut word = String::new();
    let mut in_word = false;

    for ch in str.chars() {
        if ch.is_whitespace() {
            if in_word {
                words.push(mem::take(&mut word));
                in_word = false;
            }
        } else {
            word.push(ch);
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

    for word in words {
        let word_len = word.len();
        let min_len = get_min_length(&word);
        let mut cmd = word.clone();
        uppercase(&mut cmd);
        commands.push_front(Command::new(cmd, word_len, min_len));
    }

    Ok(commands)
}

fn find_command(commands: &LinkedList<Command>, word: &str) -> Option<&Command> {
    commands.iter().find(|cmd| command_match(cmd, word))
}

fn test(commands: &LinkedList<Command>, input: &str) {
    println!(" input: {}", input);
    print!("output:");

    let words = split_into_words(input).unwrap();
    for mut word in words {
        uppercase(&mut word);
        let cmd_ptr = find_command(commands, &word);
        print!(" {}", cmd_ptr.map_or("*error*", |cmd| &cmd.cmd));
    }

    println!();
}

fn main() -> Result<(), Box<dyn Error>> {
    let commands = make_command_list(COMMAND_TABLE)?;
    let input = "riG   rePEAT copies  put mo   rest    types   fup.    6       poweRin";
    test(&commands, input);
    Ok(())
}