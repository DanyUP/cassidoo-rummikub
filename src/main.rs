mod rummikub;

use rummikub::{Deck, valid_sets};

fn main() {
    let mut deck = Deck::new_shuffled(400);
    let tray = deck.pick_tray(14);

    println!("Your tray:");
    for c in &tray {
        println!(" - {}", c);
    }

    println!("Valid sets:");
    let valid_sets = valid_sets(&tray);
    for set in valid_sets {
        print!(" -> ");
        for card in set {
            print!("{} ", card)
        }
        println!();
    }
    
}
