use std::marker::PhantomData;

const COMMAND_TABLE: &str = "add 1  alter 3  backup 2  bottom 1  Cappend 2  change 1  Schange  Cinsert 2  Clast 3 \
                             compress 4 copy 2 count 3 Coverlay 3 cursor 3  delete 3 Cdelete 2  down 1  duplicate \
                             3 xEdit 1 expand 3 extract 3  find 1 Nfind 2 Nfindup 6 NfUP 3 Cfind 2 findUP 3 fUP 2 \
                             forward 2  get  help 1 hexType 4  input 1 powerInput 3  join 1 split 2 spltJOIN load \
                             locate 1 Clocate 2 lowerCase 3 upperCase 3 Lprefix 2  macro  merge 2 modify 3 move 2 \
                             msg  next 1 overlay 1 parse preserve 4 purge 3 put putD query 1 quit  read recover 3 \
                             refresh renum 3 repeat 3 replace 1 Creplace 2 reset 3 restore 4 rgtLEFT right 2 left \
                             2  save  set  shift 2  si  sort  sos  stack 3 status 4 top  transfer 3  type 1  up 1";

// 修改: 移除生命周期参数 'a
struct Command {
    cmd: String,
    length: usize,
    min_len: usize,
    // 修改: 使用 PhantomData 来处理生命周期问题
    next: Option<Box<Command>>,
    _marker: PhantomData<()>,
}

// 修改: 移除生命周期参数 'a
fn command_match(command: &Command, str: &str) -> bool {
    let olen = str.len();
    olen >= command.min_len && olen <= command.length && str.chars().take(olen).collect::<String>() == command.cmd.chars().take(olen).collect::<String>()
}

fn uppercase(str: &mut [char]) {
    for c in str {
        *c = c.to_uppercase().next().unwrap();
    }
}

fn fatal(message: &str) -> ! {
    eprint!("{}\n", message);
    std::process::exit(1);
}

fn xmalloc<T: Default + Clone>(n: usize) -> Box<[T]> {
    vec![T::default(); n].into_boxed_slice()
}

fn xrealloc<T: Default + Clone>(mut vec: Vec<T>, n: usize) -> Vec<T> {
    vec.resize(n, T::default());
    vec
}

fn split_into_words(str: &str) -> Vec<String> {
    str.split_whitespace().map(String::from).collect()
}

fn make_command_list(table: &str) -> Option<Box<Command>> {
    let words = split_into_words(table);
    let mut cmd = None;

    for (i, word) in words.iter().enumerate() {
        let mut new_cmd = Box::new(Command {
            cmd: word.to_string(),
            length: word.len(),
            min_len: word.len(),
            next: None,
            _marker: PhantomData,
        });

        if i + 1 < words.len() {
            match words[i + 1].parse::<usize>() {
                Ok(min_len) => new_cmd.min_len = min_len,
                Err(_) => (),
            }
        }

        new_cmd.next = cmd;
        cmd = Some(new_cmd);
    }
    cmd
}

fn free_command_list(cmd: Option<Box<Command>>) {
    let mut cmd = cmd;
    while let Some(mut c) = cmd {
        cmd = c.next.take();
    }
}

// 修改: 移除生命周期参数 'a
fn find_command(commands: &Option<Box<Command>>, word: &str) -> Option<&Command> {
    let mut cmd = commands.as_ref();
    while let Some(c) = cmd {
        if command_match(c, word) {
            return Some(c);
        }
        cmd = c.next.as_ref();
    }
    None
}

// 修改: 移除生命周期参数 'a
fn test(commands: &Option<Box<Command>>, input: &str) {
    print!(" input: {}\noutput:", input);
    let words = split_into_words(input);
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
    print!("\n");
}

// 修改: 移除生命周期参数 'a
fn main() {
    let commands = make_command_list(COMMAND_TABLE);
    // 修改: 移除生命周期参数 'a
    test(&commands, "riG   rePEAT copies  put mo   rest    types   fup.    6       poweRin");
    free_command_list(commands);
}