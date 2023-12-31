use enum_iterator::Sequence;
use itertools::Itertools;
use rand::Rng;

#[derive(Debug, Sequence, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Hash, Sequence, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone)]
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
        let capacity:usize  = 52;
        let mut cards = Vec::with_capacity(capacity);

        for suit in enum_iterator::all::<Suit>() {
            for rank in enum_iterator::all::<Rank>() {
                cards.push(Card { rank, suit });
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {
        // fisher yates shuffle
        let mut rng = rand::thread_rng();
        for i in (1..52).rev() {
            let r: usize = rng.gen();
            self.cards.swap(i, r % (i + 1));
        }
    }
}

#[derive(Debug,Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
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

impl HandType {
    fn to_string(&self) -> &str {
        match self {
            HandType::HighCard => "High card",
            HandType::Pair => "A pair",
            HandType::TwoPair => "Two pair",
            HandType::ThreeOfAKind => "Three of a kind",
            HandType::Straight => "Straight",
            HandType::Flush => "Flush",
            HandType::FullHouse => "Full house",
            HandType::FourOfAKind => "Four of a kind",
            HandType::StraightFlush => "Straight Flush",
            HandType::RoyalFlush => "Royal flush",
        }
    }


}

#[derive(Debug, Clone)]
struct Hand {
    hand_type: Option<HandType>,
    combo: Option<Vec<Card>>,
    
}

impl Hand {
    fn new() -> Self {
        Hand { 
            hand_type: None,
            combo: None,
        }
    }

    fn evaluate(&self, player_cards: &[Card], community_cards: &[Card]) -> HandType {

        let mut all_cards = [player_cards, community_cards].concat();
        all_cards.sort_by_key(|card| card.rank);
        let mut best_hand = HandType::HighCard; // lowest handtype
        let mut best_combo = vec![];
                                                
                                                
        for combo in all_cards.iter().combinations(5) {
            let hand_type = evaluate_hand(&combo);
            if hand_type > best_hand {
                best_hand = hand_type;
                best_combo = combo.iter().map(|&&card| card).collect();
                
            }
        }
        self.combo = Some(best_combo);
        self.hand_type = Some(best_hand);
        best_hand
    }

}

fn is_straight(ranks: &Vec<&Rank>) -> bool {
    // ranks should be already sorted in acceeding order
    // uses rank enum defintion order to look if hand is a straight 
    for window in ranks.windows(2) {
        if let [a, b] = window {
            if **b as u8 != **a as u8 + 1 {
                return false;
            }
        }
    }

    // when ace acts as low card
    if ranks == &[&Rank::Two, &Rank::Three, &Rank::Four, &Rank::Five, &Rank::Ace] {
        return true;
    }
    true
}

fn evaluate_hand(cards: &[&Card]) -> HandType {

    // cards by rank should be sorted in acceding order 
    let mut ranks = cards.iter().map(|card| &card.rank).collect::<Vec<_>>();
    let counts = ranks.clone().into_iter().counts();
    let has_three_of_a_kind = counts.values().any(|&count| count == 3);
    let has_four_of_a_kind = counts.values().any(|&count| count == 4);

    let pairs_count = counts.values().filter(|&&count| count == 2).count();
    
    ranks.dedup(); // remove dublicates

    let mut straight = false;
    if ranks.len() == 5 {
        straight = is_straight(&ranks)
    }

    let mut suites = cards.iter().map(|card| &card.suit).collect::<Vec<_>>();
    suites.dedup(); // remove dublicates
                    
    let mut flush = false;
    if suites.len() == 1 {
        flush = true;
    }

    if straight && flush && ranks[0] == &Rank::Ten {
        HandType::RoyalFlush

    } else if straight && flush {
        HandType::StraightFlush
        
    } else if has_four_of_a_kind {
        HandType::FourOfAKind

    } else if has_three_of_a_kind && pairs_count > 0 {
        HandType::FullHouse
        
    } else if flush {
        HandType::Flush
        
    } else if straight {
        HandType::Straight

    } else if has_three_of_a_kind {
        HandType::ThreeOfAKind 

    } else if pairs_count == 2 {
        HandType::TwoPair 

    } else if pairs_count == 1  {
        HandType::Pair

    } else { 
        HandType::HighCard 
    }

}

#[derive(Debug)]
struct Player {
    nickname: String,
    position: Position,
    cards: [Card; 2],
    hand: Option<Hand>,
    // chips: Option<>,

}

#[derive(Debug)]
enum Position {
    BTN,
    SB,
    BB,
    UTG,
    UTG1,
    UTG2,
    MP,
    HJ,
    CO,

}

#[derive(Debug)]
struct Board {
    current_player: Player,
    players: Vec<Player>,
}

fn main() {
    let mut hand_types_count = std::collections::HashMap::new();
    let runs = 6_000_000;
    for _ in 0..runs {
        let mut deck = Deck::new();
        deck.shuffle();
        let mut dealt = Vec::new();
        for _ in 0..2 {
            dealt.push(deck.cards.remove(0))
        }

        let hand = Hand::new();
        let mut community_cards = Vec::new();
        for _ in 0..5 {
            community_cards.push(deck.cards.remove(0))
        }

        let handtype = hand.evaluate(&dealt, &community_cards);

        *hand_types_count.entry(handtype).or_insert(0) += 1;

        // if handtype == HandType::StraightFlush{
            // println!("Handtype is: {} hit on index: {i} ", handtype.to_string());
            // println!("Hand: {} {}", hand.cards[0].to_string(), hand.cards[1].to_string());
            // break;
        // }
        // println!("Handtype is: {}", handtype.to_string());
        // println!("Hand: {} {}", hand.cards[0].to_string(), hand.cards[1].to_string());

        // for (i,card) in deck.cards.iter().enumerate() {
        //     println!("card index: {i}: {}", card.to_string())
        // }
    } 

    for (hand_type, count) in hand_types_count {
        println!("{} occurred {} %", hand_type.to_string(), count as f64 / runs as f64);
    }

}
