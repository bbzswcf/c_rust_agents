// Added: Ensure the `libc` crate is added to your `Cargo.toml` file.
extern crate libc;

use std::mem;
use std::ptr;

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

    // Modified: Ensure curr_len is greater than 0 before subtraction
    if curr_len == 0 {
        return;
    }
    let last_char = (*REFS.add(curr_len - 1)).last_char as i32;
    for i in curr_len..REFS_LEN {
        if (*REFS.add(i)).first_char as i32 == last_char {
            let aux = *REFS.add(curr_len);
            *REFS.add(curr_len) = *REFS.add(i);
            *REFS.add(i) = aux;
            search(curr_len + 1);
            *REFS.add(i) = *REFS.add(curr_len);
            *REFS.add(curr_len) = aux;
        }
    }
}

unsafe fn find_longest_chain(items: &[&str]) {
    REFS_LEN = items.len();
    REFS = libc::calloc(REFS_LEN, mem::size_of::<Ref>()).cast::<Ref>();
    // Modified: Check for null pointer before dereferencing
    if REFS.is_null() {
        libc::exit(1);
    }

    LONGEST_PATH_REFS_LEN = 0;
    LONGEST_PATH_REFS = libc::calloc(REFS_LEN, mem::size_of::<Ref>()).cast::<Ref>();
    // Modified: Check for null pointer before dereferencing
    if LONGEST_PATH_REFS.is_null() {
        libc::exit(1);
    }

    for i in 0..items.len() {
        let itemsi_len = items[i].len();
        if itemsi_len <= 1 {
            libc::exit(1);
        }
        (*REFS.add(i)).index = i as u16;
        (*REFS.add(i)).last_char = items[i].chars().last().unwrap();
        (*REFS.add(i)).first_char = items[i].chars().next().unwrap();
    }

    for i in 0..items.len() {
        let aux = *REFS.add(0);
        *REFS.add(0) = *REFS.add(i);
        *REFS.add(i) = aux;
        search(1);
        *REFS.add(i) = *REFS.add(0);
        *REFS.add(0) = aux;
    }

    LONGEST_PATH_LEN = LONGEST_PATH_REFS_LEN;
    LONGEST_PATH = libc::calloc(LONGEST_PATH_LEN, mem::size_of::<*const str>()).cast::<*const str>();
    // Modified: Check for null pointer before dereferencing
    if LONGEST_PATH.is_null() {
        libc::exit(1);
    }
    for i in 0..LONGEST_PATH_LEN {
        *LONGEST_PATH.add(i) = items[(*LONGEST_PATH_REFS.add(i)).index as usize];
    }

    libc::free(LONGEST_PATH_REFS.cast::<libc::c_void>());
    libc::free(REFS.cast::<libc::c_void>());
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
                // Modified: Use a reference to the string slice to avoid dereferencing unsized type
                print!("{} ", unsafe { &**LONGEST_PATH.add(j) });
            }
            println!();
        }

        libc::free(LONGEST_PATH.cast::<libc::c_void>());
    }
}