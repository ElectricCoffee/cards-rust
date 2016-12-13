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

fn main() {
}
