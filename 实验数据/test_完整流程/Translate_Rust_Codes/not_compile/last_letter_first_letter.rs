#[derive(Copy)] // Modified: Removed `Clone` trait since `Ref` is already `Copy`
struct Ref {
    index: u16,
    last_char: char,
    first_char: char,
}

// Encapsulate mutable state in safe structures
struct LongestPathRefs {
    refs: Vec<Ref>,
    len: usize,
}

impl LongestPathRefs {
    fn new(capacity: usize) -> Self {
        LongestPathRefs {
            refs: Vec::with_capacity(capacity), // Modified: Use `Vec::with_capacity` for efficient initialization
            len: 0,
        }
    }

    fn copy_from(&mut self, other: &[Ref], len: usize) {
        self.refs.resize(len, Ref { index: 0, last_char: 'a', first_char: 'a' }); // Ensure capacity is sufficient
        self.refs[..len].copy_from_slice(&other[..len]);
        self.len = len;
    }
}

struct Refs {
    refs: Vec<Ref>,
    len: usize,
}

impl Refs {
    fn new(capacity: usize) -> Self {
        Refs {
            refs: Vec::with_capacity(capacity), // Modified: Use `Vec::with_capacity` for efficient initialization
            len: 0,
        }
    }
}

// Removed: Unused `LongestPath` struct

fn search(refs: &mut Refs, longest_path_refs: &mut LongestPathRefs, n_solutions: &mut usize, curr_len: usize, items: &[&'static str]) {
    if curr_len > longest_path_refs.len {
        *n_solutions = 1;
        longest_path_refs.len = curr_len;
        longest_path_refs.copy_from(&refs.refs, curr_len);
    } else if curr_len == longest_path_refs.len {
        *n_solutions += 1;
    }

    if curr_len > 0 { // Modified: Ensure `curr_len` is greater than 0 before accessing `refs.refs[curr_len - 1]`
        let last_char = refs.refs[curr_len - 1].last_char;
        for i in curr_len..refs.len {
            if refs.refs[i].first_char == last_char {
                refs.refs.swap(curr_len, i);
                search(refs, longest_path_refs, n_solutions, curr_len + 1, items);
                refs.refs.swap(curr_len, i);
            }
        }
    }
}

fn find_longest_chain(items: &[&'static str], items_len: usize) -> (LongestPathRefs, Vec<&'static str>, usize) { // Modified: Return `Vec<&'static str>` instead of `LongestPath`
    let mut refs = Refs::new(items_len);
    let mut longest_path_refs = LongestPathRefs::new(items_len);
    let mut n_solutions = 0;

    for i in 0..items_len {
        if let Some(item) = items.get(i) { // Ensure `items` is not empty and `i` is within bounds
            let itemsi_len = item.len();
            if itemsi_len > 1 { // Ensure single-character names are skipped
                refs.refs.push(Ref { index: i as u16, last_char: item.chars().last().unwrap(), first_char: item.chars().next().unwrap() }); // Modified: Push elements to avoid unnecessary memory allocation
                refs.len += 1;
            }
        }
    }

    for i in 0..refs.len {
        refs.refs.swap(0, i);
        search(&mut refs, &mut longest_path_refs, &mut n_solutions, 1, items);
        refs.refs.swap(0, i);
    }

    let mut longest_path = Vec::with_capacity(longest_path_refs.len); // Modified: Use `Vec::with_capacity` for efficient initialization
    for i in 0..longest_path_refs.len {
        let ref_item = longest_path_refs.refs[i]; // Modified: Directly access `longest_path_refs.refs[i]`
        let index = ref_item.index as usize;
        if index < items.len() { // Modified: Ensure `index` is within bounds
            longest_path.push(items[index]); // Correctly populate the path with references
        }
    }

    (longest_path_refs, longest_path, n_solutions)
}

fn main() {
    let pokemon: Vec<&'static str> = vec![
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

    let (longest_path_refs, longest_path, n_solutions) = find_longest_chain(&pokemon, pokemon_len);
    println!("Maximum path length: {}", longest_path_refs.len);
    println!("Paths of that length: {}", n_solutions);
    println!("Example path of that length:");
    for i in (0..longest_path_refs.len).step_by(7) {
        print!("  ");
        for j in i..i + 7 { // Modified: Directly use `i..i + 7` instead of `min` function
            if j < longest_path_refs.len { // Ensure `j` is within bounds
                print!("{} ", longest_path[j]);
            }
        }
        println!();
    }
}