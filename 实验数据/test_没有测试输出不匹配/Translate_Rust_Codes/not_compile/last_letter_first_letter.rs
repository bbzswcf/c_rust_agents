use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Copy, Clone)] // Modified: Added Clone trait to satisfy trait bounds required by Copy
struct Ref {
    index: u16,
    last_char: char,
    first_char: char,
}

static N_SOLUTIONS: AtomicUsize = AtomicUsize::new(0);

fn search(curr_len: usize, refs: &mut Vec<Ref>, longest_path_refs: &mut Vec<Ref>) {
    if curr_len > longest_path_refs.len() {
        N_SOLUTIONS.store(1, Ordering::SeqCst);
        longest_path_refs.clear();
        longest_path_refs.extend_from_slice(&refs[0..curr_len]); // Modified: Now Ref implements Clone
    } else if curr_len == longest_path_refs.len() {
        N_SOLUTIONS.fetch_add(1, Ordering::SeqCst);
    }

    if curr_len > 0 {
        let last_char = refs[curr_len - 1].last_char;
        for i in curr_len..refs.len() {
            if refs[i].first_char == last_char {
                refs.swap(curr_len, i);
                search(curr_len + 1, refs, longest_path_refs);
                refs.swap(curr_len, i);
            }
        }
    }
}

fn find_longest_chain(items: &[&str]) {
    let items_len = items.len();
    let mut refs = Vec::new(); // Modified: Removed unnecessary capacity allocation
    let mut longest_path_refs = Vec::new();

    for i in 0..items_len {
        let item = items[i];
        if item.len() <= 1 {
            // Modified: Handle short items gracefully instead of exiting
            eprintln!("Warning: Skipping item '{}' with length <= 1", item);
            continue;
        }
        refs.push(Ref {
            index: i as u16,
            last_char: item.chars().nth(item.len() - 1).unwrap_or_default(), // Modified: Direct access to last character
            first_char: item.chars().nth(0).unwrap_or_default(), // Modified: Direct access to first character
        });
    }

    for i in 0..items_len {
        refs.swap(0, i);
        search(1, &mut refs, &mut longest_path_refs);
        refs.swap(0, i);
    }

    let longest_path_len = longest_path_refs.len();
    println!("Maximum path length: {}", longest_path_len);
    println!("Paths of that length: {}", N_SOLUTIONS.load(Ordering::SeqCst));
    println!("Example path of that length:");
    for i in (0..longest_path_len).step_by(7) {
        print!("  ");
        for j in i..(i + 7).min(longest_path_len) {
            print!("{} ", items[longest_path_refs[j].index as usize]); // Modified: Direct access without creating a new vector
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