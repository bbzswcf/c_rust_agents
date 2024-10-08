use std::process::exit;

const COMMAND_TABLE: &str =
    "Add ALTer  BAckup Bottom  CAppend Change SCHANGE  CInsert CLAst COMPress COpy \
     COUnt COVerlay CURsor DELete CDelete Down DUPlicate Xedit EXPand EXTract Find \
     NFind NFINDUp NFUp CFind FINdup FUp FOrward GET Help HEXType Input POWerinput \
     Join SPlit SPLTJOIN  LOAD  Locate CLocate  LOWercase UPPercase  LPrefix MACRO \
     MErge MODify MOve MSG Next Overlay PARSE PREServe PURge PUT PUTD  Query  QUIT \
     READ  RECover REFRESH RENum REPeat  Replace CReplace  RESet  RESTore  RGTLEFT \
     RIght LEft  SAVE  SET SHift SI  SORT  SOS  STAck STATus  TOP TRAnsfer Type Up";

// Modified: Introduced a named lifetime parameter 'a to specify the relationship between the input references and the output reference.
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

fn uppercase(str: &mut String) {
    *str = str.to_uppercase();
}

fn get_min_length(str: &str) -> usize {
    str.chars().take_while(|c| c.is_uppercase()).count()
}

fn fatal(message: &str) {
    eprintln!("{}", message);
    exit(1);
}

fn split_into_words(str: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut start = 0;
    let len = str.len();
    while start < len {
        while start < len && str[start..start+1].chars().next().unwrap().is_whitespace() {
            start += 1;
        }
        let mut end = start;
        while end < len && !str[end..end+1].chars().next().unwrap().is_whitespace() {
            end += 1;
        }
        if start < end {
            words.push(str[start..end].to_string());
        }
        start = end;
    }
    words
}

fn make_command_list(table: &str) -> Option<Box<Command>> {
    let words = split_into_words(table);
    let mut cmd = None;
    for word in words {
        let word_len = word.len();
        let mut new_cmd = Box::new(Command {
            length: word_len,
            min_len: get_min_length(&word),
            cmd: word.to_uppercase(),
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

// Modified: Introduced a named lifetime parameter 'a to specify the relationship between the input references and the output reference.
fn find_command<'a>(commands: &'a Option<Box<Command>>, word: &'a str) -> Option<&'a Command> {
    let mut current = commands.as_ref();
    while let Some(node) = current {
        if command_match(node, word) {
            return Some(node);
        }
        current = node.next.as_ref();
    }
    None
}

fn test(commands: &Option<Box<Command>>, input: &str) {
    println!(" input: {}", input);
    print!("output:");
    let words = split_into_words(input);
    for mut word in words {
        uppercase(&mut word);
        if let Some(cmd_ptr) = find_command(commands, &word) {
            print!(" {}", cmd_ptr.cmd);
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