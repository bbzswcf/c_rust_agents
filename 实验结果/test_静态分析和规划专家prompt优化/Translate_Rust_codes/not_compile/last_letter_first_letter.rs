use std::alloc::{alloc, Layout};
use std::mem;
use std::process;
use std::ptr;
use std::slice;
#[derive(Clone, Copy)]
struct Ref {
    index: u16,
    last_char: char,
    first_char: char,
}
static mut LONGEST_PATH_REFS: *mut Ref = ptr::null_mut();
static mut LONGEST_PATH_REFS_LEN: usize = 0;
static mut REFS: *mut Ref = ptr::null_mut();
static mut REFS_LEN: usize = 0;
static mut N_SOLUTIONS: usize = 0;
static mut LONGEST_PATH: *mut *const str = ptr::null_mut();
static mut LONGEST_PATH_LEN: usize = 0;
unsafe fn search(curr_len: usize) {
    if curr_len == LONGEST_PATH_REFS_LEN {
        N_SOLUTIONS += 1;
    } else if curr_len > LONGEST_PATH_REFS_LEN {
        N_SOLUTIONS = 1;
        LONGEST_PATH_REFS_LEN = curr_len;
        ptr::copy_nonoverlapping(REFS, LONGEST_PATH_REFS, curr_len);
    }
    let last_char = (*REFS.add(curr_len - 1)).last_char as isize;
    for i in curr_len..REFS_LEN {
        if (*REFS.add(i)).first_char as isize == last_char {
            mem::swap(&mut *REFS.add(curr_len), &mut *REFS.add(i));
            search(curr_len + 1);
            mem::swap(&mut *REFS.add(curr_len), &mut *REFS.add(i));
        }
    }
}
unsafe fn find_longest_chain(items: &[&str]) {
    REFS_LEN = items.len();
    let layout = Layout::array::<Ref>(REFS_LEN).unwrap();
    REFS = alloc(layout) as *mut Ref;
    LONGEST_PATH_REFS_LEN = 0;
    LONGEST_PATH_REFS = alloc(layout) as *mut Ref;
    for i in 0..items.len() {
        let itemsi_len = items[i].len();
        if itemsi_len <= 1 {
            process::exit(1);
        }
        (*REFS.add(i)).index = i as u16;
        (*REFS.add(i)).last_char = items[i].chars().last().unwrap();
        (*REFS.add(i)).first_char = items[i].chars().next().unwrap();
    }
    for i in 0..items.len() {
        mem::swap(&mut *REFS.add(0), &mut *REFS.add(i));
        search(1);
        mem::swap(&mut *REFS.add(0), &mut *REFS.add(i));
    }
    LONGEST_PATH_LEN = LONGEST_PATH_REFS_LEN;
    let layout = Layout::array::<*const str>(LONGEST_PATH_LEN).unwrap();
    LONGEST_PATH = alloc(layout) as *mut *const str;
    for i in 0..LONGEST_PATH_LEN {
        *LONGEST_PATH.add(i) = items[(*LONGEST_PATH_REFS.add(i)).index as usize];
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
        println!("Maximum path length: {}", LONGEST_PATH_LEN);
        println!("Paths of that length: {}", N_SOLUTIONS);
        println!("Example path of that length:");
        for i in (0..LONGEST_PATH_LEN).step_by(7) {
            print!("  ");
            for j in i..(i + 7).min(LONGEST_PATH_LEN) {
                print!("{:?} ", *LONGEST_PATH.add(j));
            }
            println!();
        }
    }
}