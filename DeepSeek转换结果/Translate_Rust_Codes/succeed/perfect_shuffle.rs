const N_DECKS: usize = 7;
const K_DECKS: [i32; N_DECKS] = [8, 24, 52, 100, 1020, 1024, 10000];

fn create_deck(deck: &mut Vec<i32>, n_cards: i32) -> bool {
    *deck = Vec::with_capacity(n_cards as usize);
    deck.resize(n_cards as usize, 0);
    !deck.is_empty()
}

fn init_deck(deck: &mut [i32]) {
    for (i, item) in deck.iter_mut().enumerate() {
        *item = i as i32;
    }
}

fn duplicate_deck(dest: &mut Vec<i32>, orig: &[i32], n_cards: i32) -> bool {
    if orig.is_empty() {
        return false;
    }
    if !create_deck(dest, n_cards) {
        return false;
    }
    unsafe {
        std::ptr::copy_nonoverlapping(orig.as_ptr(), dest.as_mut_ptr(), n_cards as usize);
    }
    true
}

fn inited_deck(deck: &[i32], n_cards: i32) -> bool {
    for i in 0..n_cards {
        if deck[i as usize] != i {
            return false;
        }
    }
    true
}

fn shuffle_deck(deck: &mut [i32], n_cards: i32) -> bool {
    let mut copy = Vec::new();
    if !duplicate_deck(&mut copy, deck, n_cards) {
        return false;
    }
    for i in 0..(n_cards / 2) {
        deck[2 * i as usize] = copy[i as usize];
        deck[2 * i as usize + 1] = copy[(i + n_cards / 2) as usize];
    }
    true
}

fn main() {
    for &n_cards in K_DECKS.iter() {
        let mut deck = Vec::new();
        if !create_deck(&mut deck, n_cards) {
            eprint!("Error: malloc() failed!\n");
            return;
        }

        init_deck(&mut deck);
        let mut n_shuffles = 0;

        loop {
            if !shuffle_deck(&mut deck, n_cards) {
                eprint!("Error: shuffle failed!\n");
                return;
            }
            n_shuffles += 1;
            if inited_deck(&deck, n_cards) {
                break;
            }
        }

        print!("Cards count: {}, shuffles required: {}.\n", n_cards, n_shuffles);
    }
}