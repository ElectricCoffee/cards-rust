#[derive(Debug)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug)]
struct Card {
    suit: Suit,
    value: char,
}

impl Card {
    fn new(suit: Suit, value: char) -> Card {
        Card { suit: suit, value: value }
    }
}
fn main() {
    println!("Hello, world!");
}
