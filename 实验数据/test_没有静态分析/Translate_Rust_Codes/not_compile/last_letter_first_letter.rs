use std::alloc::{alloc, Layout};
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

static mut LONGEST_PATH: *mut *mut &str = std::ptr::null_mut(); // Modified: Changed type to *mut *mut &str
static mut LONGEST_PATH_LEN: usize = 0;

unsafe fn search(curr_len: usize) {
    if curr_len == LONGEST_PATH_REFS_LEN {
        N_SOLUTIONS += 1;
    } else if curr_len > LONGEST_PATH_REFS_LEN {
        N_SOLUTIONS = 1;
        LONGEST_PATH_REFS_LEN = curr_len;
        ptr::copy_nonoverlapping(REFS, LONGEST_PATH_REFS, curr_len); // Modified: Used ptr::copy_nonoverlapping instead of copy_from
    }

    let last_char = (*REFS.add(curr_len - 1)).last_char as i32; // Modified: Ensured correct usage of add
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

unsafe fn find_longest_chain(items: &[&str], items_len: usize) {
    REFS_LEN = items_len;
    let layout = Layout::array::<Ref>(REFS_LEN).unwrap();
    REFS = alloc(layout) as *mut Ref; // Modified: Ensured correct usage of alloc

    LONGEST_PATH_REFS_LEN = 0;
    LONGEST_PATH_REFS = alloc(layout) as *mut Ref; // Modified: Ensured correct usage of alloc

    for i in 0..items_len {
        let itemsi_len = items[i].len();
        if itemsi_len <= 1 {
            process::exit(1);
        }
        (*REFS.add(i)).index = i as u16; // Modified: Ensured correct usage of add
        (*REFS.add(i)).last_char = items[i].chars().last().unwrap();
        (*REFS.add(i)).first_char = items[i].chars().next().unwrap();
    }

    for i in 0..items_len {
        let aux = *REFS.add(0); // Modified: Ensured correct usage of add
        *REFS.add(0) = *REFS.add(i);
        *REFS.add(i) = aux;
        search(1);
        *REFS.add(i) = *REFS.add(0);
        *REFS.add(0) = aux;
    }

    LONGEST_PATH_LEN = LONGEST_PATH_REFS_LEN;
    let layout = Layout::array::<*mut &str>(LONGEST_PATH_LEN).unwrap(); // Modified: Changed type to *mut &str
    LONGEST_PATH = alloc(layout) as *mut *mut &str; // Modified: Ensured correct usage of alloc
    for i in 0..LONGEST_PATH_LEN {
        *LONGEST_PATH.add(i) = items[(*LONGEST_PATH_REFS.add(i)).index as usize]; // Modified: Changed to mutable assignment
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
                print!("{:?} ", *LONGEST_PATH.add(j)); // Modified: Ensured correct usage of add
            }
            println!();
        }
    }
}