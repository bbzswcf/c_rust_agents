use std::mem;

#[derive(Clone, Copy)]
struct Ref {
    index: u16,
    last_char: char,
    first_char: char,
}

fn search(refs: &mut [Ref], longest_path_refs: &mut Vec<Ref>, n_solutions: &mut usize, curr_len: usize) {
    if curr_len == longest_path_refs.len() {
        *n_solutions += 1;
    } else if curr_len > longest_path_refs.len() {
        *n_solutions = 1;
        longest_path_refs.clear();
        longest_path_refs.extend_from_slice(&refs[..curr_len]);
    }

    let last_char = refs[curr_len - 1].last_char;
    for i in curr_len..refs.len() {
        if refs[i].first_char == last_char {
            refs.swap(curr_len, i);
            search(refs, longest_path_refs, n_solutions, curr_len + 1);
            refs.swap(curr_len, i);
        }
    }
}

fn find_longest_chain(items: &[&str]) {
    let refs_len = items.len();
    let mut refs: Vec<Ref> = vec![Ref { index: 0, last_char: '\0', first_char: '\0' }; refs_len];
    let mut longest_path_refs: Vec<Ref> = Vec::new();
    let mut n_solutions = 0;

    for (i, &item) in items.iter().enumerate() {
        let item_len = item.len();
        if item_len <= 1 {
            panic!("Items must be longer than 1 character");
        }
        refs[i] = Ref {
            index: i as u16,
            last_char: item.chars().last().unwrap(),
            first_char: item.chars().next().unwrap(),
        };
    }

    for i in 0..refs_len {
        refs.swap(0, i);
        search(&mut refs, &mut longest_path_refs, &mut n_solutions, 1);
        refs.swap(0, i);
    }

    let longest_path_len = longest_path_refs.len();
    let mut longest_path: Vec<&str> = Vec::with_capacity(longest_path_len);
    for &ref_item in &longest_path_refs {
        longest_path.push(items[ref_item.index as usize]);
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
    let pokemon = vec![
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