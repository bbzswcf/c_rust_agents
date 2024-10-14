use std::mem;

const N_DECKS: usize = 7;
const K_DECKS: [usize; N_DECKS] = [8, 24, 52, 100, 1020, 1024, 10000];

fn create_deck(deck: &mut Vec<i32>, n_cards: usize) -> bool {
    *deck = vec![0; n_cards];
    !deck.is_empty()
}

fn init_deck(deck: &mut [i32]) {
    for (i, card) in deck.iter_mut().enumerate() {
        *card = i as i32;
    }
}

fn duplicate_deck(dest: &mut Vec<i32>, orig: &[i32]) -> bool {
    if orig.is_empty() {
        return false;
    }
    *dest = orig.to_vec();
    !dest.is_empty()
}

fn inited_deck(deck: &[i32]) -> bool {
    for (i, &card) in deck.iter().enumerate() {
        if card != i as i32 {
            return false;
        }
    }
    true
}

fn shuffle_deck(deck: &mut [i32]) -> bool {
    let mut copy = Vec::new();
    if !duplicate_deck(&mut copy, deck) {
        return false;
    }

    let n_cards = deck.len();
    for i in 0..n_cards / 2 {
        deck[2 * i] = copy[i];
        deck[2 * i + 1] = copy[i + n_cards / 2];
    }

    true
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
            if !shuffle_deck(&mut deck) {
                eprintln!("Error: shuffle failed!");
                return;
            }
            n_shuffles += 1;
            if inited_deck(&deck) {
                break;
            }
        }

        println!("Cards count: {}, shuffles required: {}.", n_cards, n_shuffles);
    }
}