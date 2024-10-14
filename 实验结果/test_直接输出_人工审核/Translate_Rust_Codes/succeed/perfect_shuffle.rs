use std::ptr;

const N_DECKS: usize = 7;
const K_DECKS: [usize; N_DECKS] = [8, 24, 52, 100, 1020, 1024, 10000];

fn create_deck(deck: &mut Vec<i32>, n_cards: usize) -> bool {
    *deck = vec![0; n_cards];
    !deck.is_empty()
}

fn init_deck(deck: &mut [i32]) {
    for (i, item) in deck.iter_mut().enumerate() {
        *item = i as i32;
    }
}

fn duplicate_deck(dest: &mut Vec<i32>, orig: &[i32], n_cards: usize) -> bool {
    if create_deck(dest, n_cards) {
        unsafe {
            ptr::copy_nonoverlapping(orig.as_ptr(), dest.as_mut_ptr(), n_cards);
        }
        true
    } else {
        false
    }
}

fn inited_deck(deck: &[i32], n_cards: usize) -> bool {
    for i in 0..n_cards {
        if deck[i] != i as i32 {
            return false;
        }
    }
    true
}

fn shuffle_deck(deck: &mut [i32], n_cards: usize) -> bool {
    let mut copy = Vec::new();
    if duplicate_deck(&mut copy, deck, n_cards) {
        for i in 0..n_cards / 2 {
            deck[i * 2] = copy[i];
            deck[i * 2 + 1] = copy[i + n_cards / 2];
        }
        true
    } else {
        false
    }
}

fn main() {
    for &n_cards in K_DECKS.iter() {
        let mut deck = Vec::new();

        if !create_deck(&mut deck, n_cards) {
            eprintln!("Error: malloc() failed!");
            return;
        }

        init_deck(&mut deck);
        let mut n_shuffles = 0;

        loop {
            if !shuffle_deck(&mut deck, n_cards) {
                eprintln!("Error: shuffle failed!");
                return;
            }
            n_shuffles += 1;
            if inited_deck(&deck, n_cards) {
                break;
            }
        }

        println!("Cards count: {}, shuffles required: {}.", n_cards, n_shuffles);
    }
}