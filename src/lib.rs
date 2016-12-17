extern crate rand;
use ::std::str;
use self::rand::Rng;

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub struct Card {
    suit:  String,
    value: String,
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

pub fn make_standard_deck() -> Vec<Card> {
    let mut result = Vec::new();
    let suits  = &["♣", "♦", "♥", "♠"];
    let values = &["2", "3", "4", "5", "6", "7", "8", "9", "10",
                   "Jack", "Queen", "King", "Ace"];

    for suit in suits {
        for value in values {
            result.push(Card::new_from_str(*suit, value));
        }
    }
    return result;
}

pub fn make_fifth_dimension_deck() -> Vec<Card> {
    let mut result = Vec::new();
    let suits  = &["♣", "♦", "♥", "♠", "★"];
    let values = &["1", "2", "3", "4", "5", "6", "7", "8", "9", "10",
                   "Jack", "Princess", "Queen", "King", "Joker"];

    for suit in suits {
        for value in values {
            result.push(Card::new_from_str(*suit, value));
        }
    }
    return result;
}

pub fn shuffle(deck: &mut Vec<Card>) {
    rand::thread_rng().shuffle(deck);
}
