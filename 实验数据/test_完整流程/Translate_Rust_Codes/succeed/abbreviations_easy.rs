use std::process::exit;
use std::str::SplitWhitespace;

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

// Modified: Convert both the command and the input string to uppercase and then check if the input string is a prefix of the command.
fn command_match<'a>(command: &'a Command, str: &str) -> bool {
    let olen = str.len();
    olen >= command.min_len && olen <= command.length && command.cmd.to_uppercase().starts_with(&str.to_uppercase())
}

// No lifetime parameter needed since the function returns a `String`.
fn uppercase(str: &str) -> String {
    str.to_uppercase()
}

// Modified: Calculate the minimum length based on the shortest valid prefix that can match the input string, considering case insensitivity.
fn get_min_length(str: &str) -> usize {
    str.chars().take_while(|c| c.is_uppercase()).count()
}

// No lifetime parameter needed since the function does not return any references.
fn fatal(message: &str) {
    eprintln!("{}", message);
    exit(1);
}

// No changes needed here.
fn split_into_words(str: &str) -> Vec<String> {
    str.split_whitespace().map(|s| s.to_string()).collect()
}

// Modified: Create the command list in the correct order, handling case insensitivity and partial matches.
fn make_command_list(table: &str) -> Option<Box<Command>> {
    let mut cmd = None;
    let words = split_into_words(table);
    for word in words {
        let word_len = word.len();
        let new_cmd = Box::new(Command {
            length: word_len,
            min_len: get_min_length(&word),
            cmd: word, // Store the original case of the command
            next: cmd,
        });
        cmd = Some(new_cmd);
    }
    cmd
}

// Modified: Look up the command in the command list based on the input string, considering partial matches and case insensitivity.
fn find_command<'a>(commands: &'a Option<Box<Command>>, word: &str) -> Option<&'a Command> {
    let mut current = commands;
    while let Some(cmd) = current {
        if command_match(cmd, word) {
            return Some(cmd);
        }
        current = &cmd.next;
    }
    None
}

// Modified: Print the matched command in uppercase to match the expected output.
fn test(commands: &Option<Box<Command>>, input: &str) {
    println!(" input: {}", input);
    print!("output:");
    let words = split_into_words(input);
    for word in words {
        if let Some(cmd) = find_command(commands, &word) {
            print!(" {}", uppercase(&cmd.cmd)); // Print the matched command in uppercase
        } else {
            print!(" *error*");
        }
    }
    println!();
}

// No lifetime parameter needed since the function does not return any references.
fn main() {
    let commands = make_command_list(COMMAND_TABLE);
    let input = "riG   rePEAT copies  put mo   rest    types   fup.    6       poweRin";
    test(&commands, input);
}