use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
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

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
pub enum Card {
    Numbered {
        number: i8,
        color: Color
    },
    Wildcard
}

impl Card {
    fn new(number: i8, color: Color) -> Card {
        Card::Numbered { number, color }
    }

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

    fn is_wildcard(&self) -> bool {
        *self == Card::Wildcard
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
                    cards.push(Card::new(num, c));
                }
            }
            cards.push(Card::Wildcard);
        }
        Deck { cards }
    }

    pub fn new_shuffled() -> Deck {
        let mut deck = Deck::new();
        deck.shuffle();
        deck
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
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

fn get_wildcards(cards: &[Card]) -> Vec<&Card> {
    cards.iter()
        .filter(|c| c.is_wildcard())
        .collect()
}

fn create_permutations<'a>(set: &[&'a Card]) -> BTreeSet<Vec<&'a Card>> {
    let mut permutations = BTreeSet::new();
    if set.len() <= 4 {
        permutations.insert(set.to_owned());
    }
    if set.len() > 3 {
        for remove_idx in 0..set.len() {
            let mut subset = set.to_owned();
            subset.remove(remove_idx);
            let mut subpermutations = create_permutations(&subset);
            permutations.append(&mut subpermutations);
        }
    }
    permutations
}

fn create_run_windows<'a>(set: &[Option<&'a Card>], wildcards: &[&'a Card]) -> Vec<Vec<&'a Card>> {
    let mut found_windows = vec![];
    for window_length in (3..=set.len()).rev() {
        // println!("Trying window of length {}", window_length);
        for start_idx in 0..=set.len()-window_length {
            //println!("Checking subset from {} to {}", start_idx, window_length+start_idx);
            let subwindow = &set[start_idx..window_length+start_idx];
            let missing_cards = subwindow.iter()
                .filter(|c| c.is_none())
                .count();
            if missing_cards <= wildcards.len() {
                let mut tmp_wildcards = wildcards.to_owned();
                let run: Vec<&Card> = subwindow.iter()
                    .map(|c| match c {
                        Some(card) => *card,
                        None => tmp_wildcards.pop().unwrap()
                    })
                    .collect();
                //println!("Found run: {:?}", run);
                found_windows.push(run);
            }
        }
        
    }
    found_windows
}

fn find_runs(cards: &[Card]) -> Vec<Vec<&Card>> {
    let wildcards = get_wildcards(cards);

    // Sort the cards by number and color (with Wildcards at the end)
    //let mut sorted_cards: Vec<&Card> = cards.iter().collect();
    //sorted_cards.sort();

    let mut sets: Vec<Vec<&Card>> = vec![];
    // Group cards (excluding wildcards) by their color
    let mut grouped_cards: BTreeMap<Color, BTreeSet<&Card>> = BTreeMap::new();
    for c in cards {
        if let Card::Numbered { number: _, color } = c {
            grouped_cards.entry(*color).or_default().insert(c);
        }
        
    }

    for cards in grouped_cards.values() {
        // Take only groups with 3 cards or more (even with the help of wildcards)
        if cards.len() + wildcards.len() >= 3 {
            let mut all_nums_set: Vec<Option<&Card>> = Vec::with_capacity(13);
            for num in 1..=13 {
                let available_card = cards.iter().find(|c| c.number() == Some(&num)).cloned();
                all_nums_set.push(available_card);
            }
            //println!("{:?}", all_nums_set);

            let mut run_windows = create_run_windows(&all_nums_set, &wildcards);
            sets.append(&mut run_windows);
        }

    }
    sets
}

fn find_same_numbers(cards: &[Card]) -> Vec<Vec<&Card>> {
    let wildcards = get_wildcards(cards);

    // Sort the cards by number and color (with Wildcards at the end)
    let mut sorted_cards: Vec<&Card> = cards.iter().collect();
    sorted_cards.sort();

    let mut sets: BTreeSet<Vec<&Card>> = BTreeSet::new();
    // Group cards (excluding wildcards) by their number
    let grouped_cards = sorted_cards.into_iter()
        .filter(|c| !c.is_wildcard())
        .group_by(|c| c.number());

    for (_, cards) in &grouped_cards {
        // Exclude cards with duplicated color
        let mut cards: Vec<&Card> = cards.unique_by(|c| c.color()).collect();
        // Take only groups with 3 cards or more (even with the help of wildcards)
        if cards.len() + wildcards.len() >= 3 {
            // Add wildcards to the set
            for wildcard in &wildcards {
                cards.push(wildcard)
            }

            // Compute permutation
            // The group of cards + wildcards can be > 4, since create_permutations excludes blocks bigger than 4
            let mut permutations = create_permutations(&cards);
            sets.append(&mut permutations)
        }

    }
    sets.into_iter().collect()
}

pub fn valid_sets(cards: &[Card]) -> Vec<Vec<&Card>> {
    let mut runs = find_runs(cards);
    let mut same_numbers = find_same_numbers(cards);
    same_numbers.append(&mut runs);
    same_numbers
}

#[cfg(test)]
mod tests {

    use super::*;

    fn equals_vec<T>(ref_slice: &[&T], struct_slice: &[T]) -> bool
        where T : std::cmp::PartialEq {
            ref_slice.len() == struct_slice.len() &&
                ref_slice.iter().all(|&c| struct_slice.contains(c))
    }

    #[test]
    fn find_same_numbers_test() {
        let test_tray = vec![
            Card::new(2, Color::Blue),
            Card::new(2, Color::Red),
            Card::new(2, Color::Yellow),
            Card::new(3, Color::Blue),
            Card::new(3, Color::Blue),
            Card::new(3, Color::Red),
            Card::new(4, Color::Yellow),
            Card::new(5, Color::Blue)
        ];

        let sets = find_same_numbers(&test_tray);
        
        let match_sets = vec![
            vec![
                Card::new(2, Color::Blue),
                Card::new(2, Color::Red),
                Card::new(2, Color::Yellow)
            ]
        ];
        
        assert!(sets.len() == match_sets.len());
        for cur_match_set in match_sets {
            assert!(sets.iter().any(|s| equals_vec(s, &cur_match_set)));
        }
    }

    #[test]
    fn find_same_numbers_with_wildcards_test() {
        let test_tray = vec![
            Card::new(2, Color::Blue),
            Card::new(2, Color::Red),
            Card::new(2, Color::Yellow),
            Card::new(3, Color::Blue),
            Card::new(3, Color::Blue),
            Card::new(3, Color::Red),
            Card::new(4, Color::Yellow),
            Card::new(5, Color::Blue),
            Card::Wildcard
        ];

        let sets = find_same_numbers(&test_tray);

        let match_sets = vec![
            vec![
                Card::new(2, Color::Blue),
                Card::new(2, Color::Red),
                Card::new(2, Color::Yellow),
                Card::Wildcard
            ],
            vec![
                Card::new(2, Color::Blue),
                Card::new(2, Color::Red),
                Card::new(2, Color::Yellow)
            ],
            vec![
                Card::new(2, Color::Blue),
                Card::new(2, Color::Red),
                Card::Wildcard
            ],
            vec![
                Card::new(2, Color::Blue),
                Card::new(2, Color::Yellow),
                Card::Wildcard
            ],
            vec![
                Card::new(2, Color::Red),
                Card::new(2, Color::Yellow),
                Card::Wildcard
            ],
            vec![
                Card::new(3, Color::Blue),
                Card::new(3, Color::Red),
                Card::Wildcard
            ]
        ];

        assert!(sets.len() == match_sets.len());
        for cur_match_set in match_sets {
            assert!(sets.iter().any(|s| equals_vec(s, &cur_match_set)));
        }
        
    }

    #[test]
    fn find_same_numbers_with_2_wildcards_test() {
        let test_tray = vec![
            Card::new(2, Color::Blue),
            Card::new(2, Color::Red),
            Card::new(2, Color::Yellow),
            Card::Wildcard,
            Card::Wildcard
        ];

        let sets = find_same_numbers(&test_tray);
        
        let match_sets = vec![
            vec![
                Card::new(2, Color::Blue),
                Card::new(2, Color::Red),
                Card::new(2, Color::Yellow)
            ],
            vec![
                Card::new(2, Color::Blue),
                Card::new(2, Color::Red),
                Card::new(2, Color::Yellow),
                Card::Wildcard
            ],
            
            vec![
                Card::new(2, Color::Blue),
                Card::new(2, Color::Red),
                Card::Wildcard
            ],
                        
            vec![
                Card::new(2, Color::Blue),
                Card::new(2, Color::Yellow),
                Card::Wildcard
            ],
                        
            vec![
                Card::new(2, Color::Red),
                Card::new(2, Color::Yellow),
                Card::Wildcard
            ],
                        
            vec![
                Card::new(2, Color::Blue),
                Card::new(2, Color::Red),
                Card::Wildcard,
                Card::Wildcard
            ],
                        
            vec![
                Card::new(2, Color::Blue),
                Card::new(2, Color::Yellow),
                Card::Wildcard,
                Card::Wildcard
            ],
                        
            vec![
                Card::new(2, Color::Red),
                Card::new(2, Color::Yellow),
                Card::Wildcard,
                Card::Wildcard
            ],
            
            vec![
                Card::new(2, Color::Blue),
                Card::Wildcard,
                Card::Wildcard
            ],
            
            vec![
                Card::new(2, Color::Red),
                Card::Wildcard,
                Card::Wildcard
            ],
            vec![
                Card::new(2, Color::Yellow),
                Card::Wildcard,
                Card::Wildcard
            ]
        ];
        
        println!("{:?}", sets);
        assert_eq!(sets.len(), match_sets.len());
        for cur_match_set in match_sets {
            assert!(sets.iter().any(|s| equals_vec(s, &cur_match_set)));
        }
    }

    #[test]
    fn find_same_numbers_with_2_wildcards_2_items_test() {
        let test_tray = vec![
            Card::new(3, Color::Blue),
            Card::new(3, Color::Red),
            Card::Wildcard,
            Card::Wildcard
        ];

        let sets = find_same_numbers(&test_tray);
        
        let match_sets = vec![
            vec![
                Card::new(3, Color::Blue),
                Card::new(3, Color::Red),
                Card::Wildcard
            ],
            vec![
                Card::new(3, Color::Blue),
                Card::new(3, Color::Red),
                Card::Wildcard,
                Card::Wildcard
            ],
            vec![
                Card::new(3, Color::Blue),
                Card::Wildcard,
                Card::Wildcard
            ],
            vec![
                Card::new(3, Color::Red),
                Card::Wildcard,
                Card::Wildcard
            ]
        ];
        
        println!("{:?}", sets);
        assert_eq!(sets.len(), match_sets.len());
        for cur_match_set in match_sets {
            assert!(sets.iter().any(|s| equals_vec(s, &cur_match_set)));
        }
    }

    #[test]
    fn find_same_numbers_with_2_wildcards_1_item_test() {
        let test_tray = vec![
            Card::new(3, Color::Blue),
            Card::Wildcard,
            Card::Wildcard
        ];

        let sets = find_same_numbers(&test_tray);
        
        let match_sets = vec![
            vec![
                Card::new(3, Color::Blue),
                Card::Wildcard,
                Card::Wildcard
            ]
        ];
        
        println!("{:?}", sets);
        assert_eq!(sets.len(), match_sets.len());
        for cur_match_set in match_sets {
            assert!(sets.iter().any(|s| equals_vec(s, &cur_match_set)));
        }
    }

    #[test]
    fn find_runs_test() {
        let test_tray = vec![
            Card::new(2, Color::Blue),
            Card::new(3, Color::Blue),
            Card::new(4, Color::Blue),
            Card::new(5, Color::Red),
            Card::new(6, Color::Blue),
            Card::new(7, Color::Blue)
        ];

        let sets = find_runs(&test_tray);
        
        let match_sets = vec![
            vec![
                Card::new(2, Color::Blue),
                Card::new(3, Color::Blue),
                Card::new(4, Color::Blue),
            ]
        ];
        
        println!("{:?}", sets);
        assert_eq!(sets.len(), match_sets.len());
        for cur_match_set in match_sets {
            assert!(sets.iter().any(|s| equals_vec(s, &cur_match_set)));
        }
    }

    #[test]
    fn find_runs_with_wildcards_test() {
        let test_tray = vec![
            Card::new(2, Color::Blue),
            Card::new(3, Color::Blue),
            Card::new(4, Color::Blue),
            Card::new(5, Color::Red),
            Card::new(6, Color::Blue),
            Card::new(7, Color::Blue),
            Card::Wildcard
        ];

        let sets = find_runs(&test_tray);
        
        let match_sets = vec![
            vec![
                Card::new(2, Color::Blue),
                Card::new(3, Color::Blue),
                Card::new(4, Color::Blue),
                Card::Wildcard,
                Card::new(6, Color::Blue),
                Card::new(7, Color::Blue)
            ],
            vec![
                Card::new(2, Color::Blue),
                Card::new(3, Color::Blue),
                Card::new(4, Color::Blue),
                Card::Wildcard,
                Card::new(6, Color::Blue)
            ],
            vec![
                Card::new(3, Color::Blue),
                Card::new(4, Color::Blue),
                Card::Wildcard,
                Card::new(6, Color::Blue),
                Card::new(7, Color::Blue)
            ],
            vec![
                Card::new(2, Color::Blue),
                Card::new(3, Color::Blue),
                Card::new(4, Color::Blue),
                Card::Wildcard
            ],
            vec![
                Card::new(3, Color::Blue),
                Card::new(4, Color::Blue),
                Card::Wildcard,
                Card::new(6, Color::Blue)
            ],
            vec![
                Card::new(4, Color::Blue),
                Card::Wildcard,
                Card::new(6, Color::Blue),
                Card::new(7, Color::Blue)
            ],
            vec![
                Card::new(2, Color::Blue),
                Card::new(3, Color::Blue),
                Card::new(4, Color::Blue)
            ],
            vec![
                Card::new(3, Color::Blue),
                Card::new(4, Color::Blue),
                Card::Wildcard
            ],
            vec![
                Card::new(4, Color::Blue),
                Card::Wildcard,
                Card::new(6, Color::Blue)
            ],
            vec![
                Card::Wildcard,
                Card::new(6, Color::Blue),
                Card::new(7, Color::Blue)
            ],
            vec![
                Card::Wildcard,
                Card::new(2, Color::Blue),
                Card::new(3, Color::Blue),
                Card::new(4, Color::Blue)
            ],
            vec![
                Card::new(2, Color::Blue),
                Card::new(3, Color::Blue),
                Card::Wildcard
            ],
            vec![
                Card::Wildcard,
                Card::new(3, Color::Blue),
                Card::new(4, Color::Blue)
            ]

        ];
        
        println!("{:?}", sets);
        assert_eq!(sets.len(), match_sets.len());
        for cur_match_set in match_sets {
            assert!(sets.iter().any(|s| equals_vec(s, &cur_match_set)));
        }
    }

    #[test]
    fn find_runs_with_2_wildcards_test() {
        let test_tray = vec![
            Card::new(2, Color::Blue),
            Card::new(3, Color::Blue),
            Card::new(6, Color::Blue),
            Card::new(7, Color::Blue),
            Card::Wildcard,
            Card::Wildcard
        ];

        let sets = find_runs(&test_tray);
        
        let match_sets = vec![
            vec![
                Card::new(2, Color::Blue),
                Card::new(3, Color::Blue),
                Card::Wildcard,
                Card::Wildcard,
                Card::new(6, Color::Blue),
                Card::new(7, Color::Blue)
            ],
            vec![
                Card::new(3, Color::Blue),
                Card::Wildcard,
                Card::Wildcard,
                Card::new(6, Color::Blue),
                Card::new(7, Color::Blue)
            ],
            vec![
                Card::new(2, Color::Blue),
                Card::new(3, Color::Blue),
                Card::Wildcard,
                Card::Wildcard,
                Card::new(6, Color::Blue)
            ],
            vec![
                Card::new(2, Color::Blue),
                Card::new(3, Color::Blue),
                Card::Wildcard,
                Card::Wildcard
            ],
            vec![
                Card::new(3, Color::Blue),
                Card::Wildcard,
                Card::Wildcard,
                Card::new(6, Color::Blue)
            ],
            vec![
                Card::Wildcard,
                Card::Wildcard,
                Card::new(6, Color::Blue),
                Card::new(7, Color::Blue)
            ],
            vec![
                Card::new(2, Color::Blue),
                Card::new(3, Color::Blue),
                Card::Wildcard
            ],
            vec![
                Card::new(3, Color::Blue),
                Card::Wildcard,
                Card::Wildcard
            ],
            vec![
                Card::Wildcard,
                Card::Wildcard,
                Card::new(6, Color::Blue)
            ],
            vec![
                Card::Wildcard,
                Card::new(6, Color::Blue),
                Card::new(7, Color::Blue)
            ],
            vec![
                Card::Wildcard,
                Card::new(2, Color::Blue),
                Card::new(3, Color::Blue),
                Card::Wildcard
            ],
            vec![
                Card::Wildcard,
                Card::new(6, Color::Blue),
                Card::new(7, Color::Blue),
                Card::Wildcard
            ],
            vec![
                Card::new(6, Color::Blue),
                Card::new(7, Color::Blue),
                Card::Wildcard,
                Card::Wildcard
            ],
            vec![
                Card::Wildcard,
                Card::new(2, Color::Blue),
                Card::new(3, Color::Blue)
            ],
            vec![
                Card::new(7, Color::Blue),
                Card::Wildcard,
                Card::Wildcard
            ],
            vec![
                Card::new(6, Color::Blue),
                Card::new(7, Color::Blue),
                Card::Wildcard
            ]
        ];
        
        println!("{:?}", sets);
        assert_eq!(sets.len(), match_sets.len());
        for cur_match_set in match_sets {
            assert!(sets.iter().any(|s| equals_vec(s, &cur_match_set)));
        }
    }

}