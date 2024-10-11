use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;
use std::process;

#[derive(Clone, Copy)]
struct Ref {
    index: u16,
    last_char: char,
    first_char: char,
}

static mut LONGEST_PATH_REFS: *mut Ref = std::ptr::null_mut();
static mut LONGEST_PATH_REFS_LEN: usize = 0;

static mut REFS: *mut Ref = std::ptr::null_mut();
static mut REFS_LEN: usize = 0;

static mut N_SOLUTIONS: usize = 0;

static mut LONGEST_PATH: *mut *const str = std::ptr::null_mut();
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
            let aux = *REFS.add(curr_len);
            *REFS.add(curr_len) = *REFS.add(i);
            *REFS.add(i) = aux;
            search(curr_len + 1);
            *REFS.add(i) = *REFS.add(curr_len);
            *REFS.add(curr_len) = aux;
        }
    }
}

unsafe fn find_longest_chain(items: &[&str], items_len: usize) {
    REFS_LEN = items_len;
    let layout = Layout::array::<Ref>(REFS_LEN).unwrap();
    REFS = alloc(layout) as *mut Ref;

    // Modified: Check for null pointer after allocation
    if REFS.is_null() {
        eprintln!("Memory allocation failed for REFS");
        process::exit(1);
    }

    LONGEST_PATH_REFS_LEN = 0;
    LONGEST_PATH_REFS = alloc(layout) as *mut Ref;

    // Modified: Check for null pointer after allocation
    if LONGEST_PATH_REFS.is_null() {
        eprintln!("Memory allocation failed for LONGEST_PATH_REFS");
        process::exit(1);
    }

    for i in 0..items_len {
        let itemsi_len = items[i].len();
        if itemsi_len <= 1 {
            process::exit(1);
        }
        (*REFS.add(i)).index = i as u16;
        (*REFS.add(i)).last_char = items[i].chars().last().unwrap();
        (*REFS.add(i)).first_char = items[i].chars().next().unwrap();
    }

    for i in 0..items_len {
        let aux = *REFS.add(0);
        *REFS.add(0) = *REFS.add(i);
        *REFS.add(i) = aux;
        search(1);
        *REFS.add(i) = *REFS.add(0);
        *REFS.add(0) = aux;
    }

    LONGEST_PATH_LEN = LONGEST_PATH_REFS_LEN;
    let layout = Layout::array::<*const str>(LONGEST_PATH_LEN).unwrap();
    LONGEST_PATH = alloc(layout) as *mut *const str;

    // Modified: Check for null pointer after allocation
    if LONGEST_PATH.is_null() {
        eprintln!("Memory allocation failed for LONGEST_PATH");
        process::exit(1);
    }

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
    let pokemon_len = pokemon.len();

    unsafe {
        find_longest_chain(&pokemon, pokemon_len);
        println!("Maximum path length: {}", LONGEST_PATH_LEN);
        println!("Paths of that length: {}", N_SOLUTIONS);
        println!("Example path of that length:");
        for i in (0..LONGEST_PATH_LEN).step_by(7) {
            print!("  ");
            for j in i..(i + 7).min(LONGEST_PATH_LEN) {
                // Modified: Dereference the pointer to get the string slice
                if !LONGEST_PATH.is_null() {
                    print!("{} ", **LONGEST_PATH.add(j)); // Changed to dereference the pointer to &str
                }
            }
            println!();
        }

        // Modified: Ensure all allocated memory is properly deallocated
        let layout = Layout::array::<Ref>(REFS_LEN).unwrap();
        dealloc(REFS as *mut u8, layout);
        dealloc(LONGEST_PATH_REFS as *mut u8, layout);

        let layout = Layout::array::<*const str>(LONGEST_PATH_LEN).unwrap();
        dealloc(LONGEST_PATH as *mut u8, layout);
    }
}