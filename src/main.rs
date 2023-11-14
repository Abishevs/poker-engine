extern crate enum_iterator;
use enum_iterator::{all, Sequence};

#[derive(Debug, Sequence, Clone, Copy)]
enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

impl Suit {
    fn to_string(&self) -> &str {
        match self {
            Suit::Club => "Clubs",
            Suit::Diamond => "Diamonds",
            Suit::Heart => "Hearts",
            Suit::Spade => "Spades",
        }
    }
}

#[derive(Debug, Sequence, Clone, Copy)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    fn to_string(&self) -> &str{
        match self {
            Rank::Two => "Two",
            Rank::Three => "Three",
            Rank::Four => "Four",
            Rank::Five => "Five",
            Rank::Six => "Six",
            Rank::Seven => "Seven",
            Rank::Eight => "Eight",
            Rank::Nine => "Nine",
            Rank::Ten => "Ten",
            Rank::Jack => "Jack",
            Rank::Queen => "Queen",
            Rank::King => "King",
            Rank::Ace => "Ace",
        }
    }
}

#[derive(Debug)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn to_string(&self) -> String{
        format!("{} of {}", self.rank.to_string(), self.suit.to_string())
    }
}

#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Deck {
        let mut cards = Vec::new();
        for suit in all::<Suit>() {
            for rank in all::<Rank>() {
                cards.push(Card { rank, suit });
            }
        }

        Deck { cards }
    }
}

#[derive(Debug)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

fn main() {
    println!("Hello, world!");
}
