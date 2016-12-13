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

fn main() {
    println!("Hello, world!");
}
