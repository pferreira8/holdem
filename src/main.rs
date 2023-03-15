use rand::{Rng};
use std::time::Instant;

//TODO IMPLEMENTATION
// #[derive(Debug, PartialEq)]
// enum HandRank {
//     HighCard,
//     Pair,
//     TwoPair,
//     ThreeOfAKind,
//     Straight,
//     Flush,
//     FullHouse,
//     FourOfAKind,
//     StraightFlush,
//     RoyalFlush,
// }



fn main() {
    // need to know when to add a new deck
    let start_time = Instant::now();
    simulation_builder(1000000);

    track_runtime(start_time);
}
fn track_runtime(pass_start: Instant) {
    let end_time = Instant::now();
    let duration = end_time.duration_since(pass_start);

    println!("Time elapsed: {:?}", duration);
}
//STRUCT AND IMPL LOGIC
#[derive(Debug, PartialEq)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, PartialEq)]
enum Rank {
    Ace,
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
}

#[derive(Debug)]
struct Card {
    rank: &'static Rank,
    suit: &'static Suit,
}

impl Card {
    fn new(rank:&'static Rank, suit:&'static Suit) -> Card {
        Card { rank, suit}
    }
}


#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
    // ranks: &'a [Rank],
    // suits: &'a [Suit],
}

impl Deck {
    //ranks: &'a [Rank], suits: &'a [Suit]
    fn new() -> Deck {
        let ranks = &[Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King];
        let suits = &[Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
        let mut cards = vec![];
        for suit in suits {
            for rank in ranks {
                cards.push(Card::new(rank, suit));
            }
        }
        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        for i in (1..self.cards.len()).rev() {
            let j = rng.gen_range(0..i+1);
            self.cards.swap(i, j);
        }
    }

    fn deal(&mut self, n: usize) -> Option<Vec<Card>> {
        if n > self.cards.len() {
            None
        } else {
            let result = self.cards.split_off(self.cards.len() - n);
            Some(result)
        }
    }
}
#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
}

#[derive(Debug)]
enum HandError {
    InvalidSize(usize),
}

impl Hand {
    fn new(cards: Vec<Card>) -> Result<Self, HandError> {
        if cards.len() != 2 {
            return Err(HandError::InvalidSize(cards.len()));
        }
        Ok(Self { cards })
    }
    fn paired_hand(&self) -> bool {
        self.cards[0].rank == self.cards[1].rank
    }

    fn suited_hand(&self) -> bool {
        self.cards[0].suit == self.cards[1].suit
    }
    
}

fn simulation_builder(n_sims: usize)  {
    let mut deck = Box::new(Deck::new());
    let mut pair_tracker = 0;
    let mut suit_tracker = 0;
    for _ in 0..n_sims {
        deck.shuffle();
        let rng_hand = deck.deal(2).map(|cards| Hand::new(cards));
        // error handler to use a new deck
        if deck.cards.is_empty() {
            deck = Box::new(Deck::new());

        }
        match rng_hand {
            Some(Ok(hand)) => {
                // println!("{:?}", hand.cards);
                if hand.paired_hand() {
                    // println!("Paired hand");
                    pair_tracker+=1;
                } 
                if hand.suited_hand() {
                    suit_tracker+=1;
                    // println!("Suited hand");
                }
            },
            _ => println!("No hand dealt")
        }
        // TODO ADD EVALUATION LOGIC 
        // after running x sims, show stats at that point, pause 5 seconds
        
    // END 
    } // FOR LOOP
    println!("Out of {} iterations: \n{} had a matching pair,\n{} were suited preflop \n", 
        n_sims, 
        pair_tracker, 
        suit_tracker);
    println!("sample probability of pair: {:?}%", pair_tracker as f32 / n_sims as f32);
    println!("sample probability of suited hand: {:?}%", suit_tracker as f32 / n_sims as f32);
}
