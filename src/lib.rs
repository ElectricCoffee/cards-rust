extern crate rand;
use std::str;
use std::fmt;
use rand::Rng;

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone)]
pub struct Card {
    pub suit:  String,
    pub value: String,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone)]
pub struct Deck {
    pub cards: Vec<Card>
}

impl Card {
    pub fn new(suit: String, value: String) -> Card {
        Card { suit: suit, value: value.to_string() }
    }

    pub fn new_from_str(suit: &str, value: &str) -> Card {
        Card::new(suit.to_string(), value.to_string())
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Suit: {}, Value: {}", self.suit, self.value)
    }
}

impl Deck {
    pub fn new_from_vec(input: Vec<Card>) -> Deck {
        Deck { cards: input }
    }

    pub fn make_standard() -> Deck {
        let mut result = Vec::new();
        let suits  = &["♣", "♦", "♥", "♠"];
        let values = &["2", "3", "4", "5", "6", "7", "8", "9", "10",
                       "Jack", "Queen", "King", "Ace"];

        for suit in suits {
            for value in values {
                result.push(Card::new_from_str(*suit, value));
            }
        }
        return Deck::new_from_vec(result);
    }

    pub fn make_fifth_dimension() -> Deck {
        let mut result = Vec::new();
        let suits  = &["♣", "♦", "♥", "♠", "★"];
        let values = &["1", "2", "3", "4", "5", "6", "7", "8", "9", "10",
                       "Jack", "Princess", "Queen", "King", "Joker"];

        for suit in suits {
            for value in values {
                result.push(Card::new_from_str(*suit, value));
            }
        }
        return Deck::new_from_vec(result);
    }

    // destructively shuffles the internal vector
    pub fn shuffle(&mut self) {
        rand::thread_rng().shuffle(&mut self.cards);
    }

    // returns a new shuffled deck
    pub fn shuffled(&self) -> Self {
        let mut res = self.clone();
        res.shuffle();
        res
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }
}

impl Ord for Deck {
    fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
        self.cards.cmp(&other.cards)
    }
}
