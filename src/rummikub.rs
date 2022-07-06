use core::fmt;
use rand::prelude::*;
use iter_tools::Itertools;

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Color {
    Red,
    Blue,
    Black,
    Yellow
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Eq, PartialEq, PartialOrd, Ord)]
pub enum Card {
    Numbered {
        number: i8,
        color: Color
    },
    Wildcard
}

impl Card {
    fn number(&self) -> Option<&i8> {
        match self {
            Card::Numbered { number, color: _ } => Some(number),
            Card::Wildcard => None
        }
    }

    fn color(&self) -> Option<&Color> {
        match self {
            Card::Numbered { number: _, color } => Some(color),
            Card::Wildcard => None
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Card::Numbered {number, color} => write!(f, "{} {}", number, color),
            Card::Wildcard => write!(f, "Wildcard")
        }
        
    }
}


pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::with_capacity(106);
        for _ in 0..2 {
            // Two sets of cards
            for c in [Color::Red, Color::Blue, Color::Black, Color::Yellow] {
                for num in 1..=13 {
                    cards.push(Card::Numbered{ number: num, color: c })
                }
            }
            cards.push(Card::Wildcard);
        }
        Deck { cards }
    }

    pub fn new_shuffled(rounds: i32) -> Deck {
        let mut deck = Deck::new();
        deck.shuffle(rounds);
        deck
    }

    pub fn shuffle(&mut self, rounds: i32) {
        let mut rng = rand::thread_rng();
        for _ in 0..rounds {
            let idx_1 = rng.gen_range(0..self.cards.len());
            let idx_2 = rng.gen_range(0..self.cards.len());
            self.cards.swap(idx_1, idx_2);
        }
    }

    pub fn pick_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn pick_tray(&mut self, num_cards: usize) -> Vec<Card> {
        let mut tray = Vec::with_capacity(num_cards);
        for _ in 0..num_cards {
            if let Some(card) = self.pick_card() {
                tray.push(card)
            }
        }
        tray
    }
}


fn find_runs(cards: &[Card]) -> Vec<Vec<&Card>> {
    Vec::new()
}

fn find_same_numbers(cards: &[Card]) -> Vec<Vec<&Card>> {
    // Sort the cards by number and color (with Wildcards at the end)
    let mut sorted_cards: Vec<&Card> = cards.iter().collect();
    sorted_cards.sort();

    let mut sets: Vec<Vec<&Card>> = vec![];
    // Group cards by their number
    for (_, cards) in &sorted_cards.into_iter().group_by(|c| c.number()) {
        // Exclude cards with duplicated color
        let cards: Vec<&Card> = cards.unique_by(|c| c.color()).collect();
        // Take only groups with 3 cards or more
        if cards.len() >= 3 {
            sets.push(cards)
        }

        // TODO: manage Wildcards
        // TODO: try every permutation
    }
    sets
}

pub fn valid_sets(cards: &[Card]) -> Vec<Vec<&Card>> {
    let mut runs = find_runs(cards);
    let mut same_numbers = find_same_numbers(cards);
    same_numbers.append(&mut runs);
    same_numbers
}