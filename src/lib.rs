extern crate rand;
use std::str;
use std::fmt;
use std::collections::VecDeque;
use rand::Rng;

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone)]
pub struct Card {
    pub suit:  String,
    pub value: String,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone)]
pub struct Deck {
    pub cards: VecDeque<Card>
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
    pub fn from_vec_deque(input: VecDeque<Card>) -> Deck {
        Deck { cards: input }
    }

    pub fn make_standard() -> Deck {
        let mut result = VecDeque::new();
        let suits  = &["♣", "♦", "♥", "♠"];
        let values = &["2", "3", "4", "5", "6", "7", "8", "9", "10",
                       "Jack", "Queen", "King", "Ace"];

        for suit in suits {
            for value in values {
                result.push_back(Card::new_from_str(*suit, value));
            }
        }
        return Deck::from_vec_deque(result);
    }

    pub fn make_fifth_dimension() -> Deck {
        let mut result = VecDeque::new();
        let suits  = &["♣", "♦", "♥", "♠", "★"];
        let values = &["1", "2", "3", "4", "5", "6", "7", "8", "9", "10",
                       "Jack", "Princess", "Queen", "King", "Joker"];

        for suit in suits {
            for value in values {
                result.push_back(Card::new_from_str(*suit, value));
            }
        }
        return Deck::from_vec_deque(result);
    }

    // destructively shuffles the internal vector
    pub fn shuffle(&mut self) {
        // shuffle isn't defined on VecDeques, so it's manually defined here.
        let mut i = self.len();
        while i > 2 {
            i -= 2;
            let j = rand::thread_rng().gen_range(0, i + 1);
            self.cards.swap(i, j)
        }
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

    pub fn push_front(&mut self, card: Card) {
        self.cards.push_front(card);
    }

    pub fn push_back(&mut self, card: Card) {
        self.cards.push_back(card);
    }

    pub fn pop_front(&mut self) -> Option<Card> {
        self.cards.pop_front()
    }

    pub fn pop_back(&mut self) -> Option<Card> {
        self.cards.pop_back()
    }
}

impl Ord for Deck {
    fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
        self.cards.cmp(&other.cards)
    }
}
