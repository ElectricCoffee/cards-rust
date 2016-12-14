extern crate rand;
use ::std::str;
use self::rand::Rng;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum Suit { Clubs, Diamonds, Hearts, Spades, Stars }

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub struct Card {
    suit: Suit,
    value: &'static str,
}

impl Card {
    pub fn new(suit: Suit, value: &'static str) -> Card {
        Card { suit: suit, value: value }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

pub fn make_standard_deck() -> Vec<Card> {
    let mut result = Vec::new();
    let suits  = &[Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
    let values = &["2", "3", "4", "5", "6", "7", "8", "9", "10",
                   "Jack", "Queen", "King", "Ace"];

    for suit in suits {
        for value in values {
            result.push(Card::new(*suit, value));
        }
    }
    return result;
}

pub fn make_fifth_dimension_deck() -> Vec<Card> {
    let mut result = Vec::new();
    let suits  = &[Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades, Suit::Stars];
    let values = &["1", "2", "3", "4", "5", "6", "7", "8", "9", "10",
                   "Jack", "Princess", "Queen", "King", "Joker"];

    for suit in suits {
        for value in values {
            result.push(Card::new(*suit, value));
        }
    }
    return result;
}

pub fn shuffle(deck: &mut Vec<Card>) {
    rand::thread_rng().shuffle(deck);
}
