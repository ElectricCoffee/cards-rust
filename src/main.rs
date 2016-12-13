use std::str;
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
    Stars,
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct Card {
    suit: Suit,
    value: &'static str,
}

impl Card {
    fn new(suit: Suit, value: &'static str) -> Card {
        Card { suit: suit, value: value }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}
fn make_standard_deck() -> Vec<Card> {
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

fn make_fifth_dimension_deck() -> Vec<Card> {
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

fn main() {
}
