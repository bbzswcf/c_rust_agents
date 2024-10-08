use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;
use std::slice;
use lazy_static::lazy_static; // 修改: 使用lazy_static crate来初始化静态变量

// :  `String`  `*const str` 
#[derive(Clone, Copy, Debug)]
struct Ref {
    index: u16,
    last_char: char,
    first_char: char,
}

lazy_static! {
    static ref LONGEST_PATH_REFS: *mut Ref = ptr::null_mut(); // 修改: 使用lazy_static来初始化静态变量
    static ref LONGEST_PATH_REFS_LEN: usize = 0; // 修改: 使用lazy_static来初始化静态变量
}

lazy_static! {
    static ref REFS: *mut Ref = ptr::null_mut(); // 修改: 使用lazy_static来初始化静态变量
    static ref REFS_LEN: usize = 0; // 修改: 使用lazy_static来初始化静态变量
}

lazy_static! {
    static ref N_SOLUTIONS: usize = 0; // 修改: 使用lazy_static来初始化静态变量
}

// :  `Vec<String>`  `*mut *const str`
lazy_static! {
    static ref LONGEST_PATH: Vec<String> = Vec::new(); // 修改: 使用lazy_static来初始化静态变量
    static ref LONGEST_PATH_LEN: usize = 0; // 修改: 使用lazy_static来初始化静态变量
}

unsafe fn search(curr_len: usize) {
    if curr_len == *LONGEST_PATH_REFS_LEN {
        *N_SOLUTIONS += 1;
    } else if curr_len > *LONGEST_PATH_REFS_LEN {
        *N_SOLUTIONS = 1;
        *LONGEST_PATH_REFS_LEN = curr_len;
        // :  `ptr::copy_nonoverlapping` 
        ptr::copy_nonoverlapping(*REFS, *LONGEST_PATH_REFS, (*LONGEST_PATH_REFS_LEN).min(curr_len));
    }

    let last_char = (*REFS.add(curr_len - 1)).last_char as isize;
    for i in curr_len..*REFS_LEN {
        if (*REFS.add(i)).first_char as isize == last_char {
            // :  `clone` 
            let aux = (*REFS.add(curr_len)).clone();
            *REFS.add(curr_len) = (*REFS.add(i)).clone();
            *REFS.add(i) = aux;
            search(curr_len + 1);
            *REFS.add(i) = (*REFS.add(curr_len)).clone();
            *REFS.add(curr_len) = aux;
        }
    }
}

unsafe fn find_longest_chain(items: &[&str]) {
    *REFS_LEN = items.len();
    let layout = Layout::array::<Ref>(*REFS_LEN).unwrap();
    *REFS = alloc(layout) as *mut Ref;

    *LONGEST_PATH_REFS_LEN = 0;
    *LONGEST_PATH_REFS = alloc(layout) as *mut Ref;

    for i in 0..items.len() {
        let itemsi_len = items[i].len();
        if itemsi_len <= 1 {
            // : 1
            continue;
        }
        (*REFS.add(i)).index = i as u16;
        // :  `chars().last()`  `chars().next()` 
        (*REFS.add(i)).last_char = items[i].chars().last().unwrap_or('\0');
        (*REFS.add(i)).first_char = items[i].chars().next().unwrap_or('\0');
    }

    for i in 0..items.len() {
        let aux = (*REFS.add(0)).clone();
        *REFS.add(0) = (*REFS.add(i)).clone();
        *REFS.add(i) = aux;
        search(1);
        *REFS.add(i) = (*REFS.add(0)).clone();
        *REFS.add(0) = aux;
    }

    *LONGEST_PATH_LEN = *LONGEST_PATH_REFS_LEN;
    *LONGEST_PATH = Vec::with_capacity(*LONGEST_PATH_LEN);
    for i in 0..*LONGEST_PATH_LEN {
        // :  `items[index].to_string()` 
        LONGEST_PATH.push(items[(*LONGEST_PATH_REFS.add(i)).index as usize].to_string());
    }
}

fn main() {
    let pokemon: Vec<&str> = vec![
        "audino", "bagon", "baltoy", "banette", "bidoof", "braviary", "bronzor", "carracosta",
        "charmeleon", "cresselia", "croagunk", "darmanitan", "deino", "emboar", "emolga",
        "exeggcute", "gabite", "girafarig", "gulpin", "haxorus", "heatmor", "heatran", "ivysaur",
        "jellicent", "jumpluff", "kangaskhan", "kricketune", "landorus", "ledyba", "loudred",
        "lumineon", "lunatone", "machamp", "magnezone", "mamoswine", "nosepass", "petilil",
        "pidgeotto", "pikachu", "pinsir", "poliwrath", "poochyena", "porygon2", "porygonz",
        "registeel", "relicanth", "remoraid", "rufflet", "sableye", "scolipede", "scrafty",
        "seaking", "sealeo", "silcoon", "simisear", "snivy", "snorlax", "spoink", "starly",
        "tirtouga", "trapinch", "treecko", "tyrogue", "vigoroth", "vulpix", "wailord", "wartortle",
        "whismur", "wingull", "yamask",
    ];

    unsafe {
        find_longest_chain(&pokemon);
        println!("Maximum path length: {}", *LONGEST_PATH_LEN);
        println!("Paths of that length: {}", *N_SOLUTIONS);
        println!("Example path of that length:");
        for i in (0..*LONGEST_PATH_LEN).step_by(7) {
            print!("  ");
            for j in i..(i + 7).min(*LONGEST_PATH_LEN) {
                // :  `LONGEST_PATH[j]`
                print!("{:?} ", LONGEST_PATH[j]);
            }
            println!();
        }
    }
}