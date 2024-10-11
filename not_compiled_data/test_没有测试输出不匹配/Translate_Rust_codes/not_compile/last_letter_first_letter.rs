use std::alloc::{alloc, Layout};
use std::mem;
use std::ptr;
use std::process;

#[derive(Clone, Copy)]
struct Ref {
    index: u16,
    last_char: char,
    first_char: char,
}

fn search(curr_len: usize, refs: &mut [Ref], longest_path_refs: &mut [Ref], n_solutions: &mut usize, longest_path_refs_len: &mut usize) {
    if curr_len == *longest_path_refs_len {
        *n_solutions += 1;
    } else if curr_len > *longest_path_refs_len {
        *n_solutions = 1;
        *longest_path_refs_len = curr_len;
        unsafe {
            ptr::copy_nonoverlapping(refs.as_ptr(), longest_path_refs.as_mut_ptr(), curr_len);
        }
    }

    let last_char = refs[curr_len - 1].last_char as i32;
    for i in curr_len..refs.len() {
        if refs[i].first_char as i32 == last_char {
            refs.swap(curr_len, i);
            search(curr_len + 1, refs, longest_path_refs, n_solutions, longest_path_refs_len);
            refs.swap(curr_len, i);
        }
    }
}

fn find_longest_chain(items: &[&str]) {
    let refs_len = items.len();
    let mut refs = Vec::with_capacity(refs_len);
    let mut longest_path_refs = Vec::with_capacity(refs_len);
    let mut longest_path_refs_len = 0;
    let mut n_solutions = 0;

    for i in 0..refs_len {
        let itemsi_len = items[i].len();
        if itemsi_len <= 1 {
            process::exit(1);
        }
        refs.push(Ref {
            index: i as u16,
            last_char: items[i].chars().last().unwrap(),
            first_char: items[i].chars().next().unwrap(),
        });
    }

    for i in 0..refs_len {
        refs.swap(0, i);
        search(1, &mut refs, &mut longest_path_refs, &mut n_solutions, &mut longest_path_refs_len);
        refs.swap(0, i);
    }

    let longest_path_len = longest_path_refs_len;
    let mut longest_path = Vec::with_capacity(longest_path_len);
    for i in 0..longest_path_len {
        longest_path.push(items[longest_path_refs[i].index as usize]);
    }

    println!("Maximum path length: {}", longest_path_len);
    println!("Paths of that length: {}", n_solutions);
    println!("Example path of that length:");
    for i in (0..longest_path_len).step_by(7) {
        print!("  ");
        for j in i..(i + 7).min(longest_path_len) {
            print!("{} ", longest_path[j]);
        }
        println!();
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

    find_longest_chain(&pokemon);
}