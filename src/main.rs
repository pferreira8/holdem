use rand::{Rng};
use std::thread;

#[derive(Debug)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug)]
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
impl PartialEq for Rank {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Debug)]
struct Card<'a> {
    rank: &'a Rank,
    suit: &'a Suit,
}

impl Card<'static> {
    fn new(rank: &'static Rank, suit: &'static Suit) -> Card<'static> {
        Card { rank, suit}
    }
}


#[derive(Debug)]
struct Deck<'a> {
    cards: Vec<Card<'a>>,
    // ranks: &'a [Rank],
    // suits: &'a [Suit],
}

impl<'a> Deck<'a> {
    //ranks: &'a [Rank], suits: &'a [Suit]
    fn new() -> Deck<'a> {
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

    fn deal(&mut self, n: usize) -> Option<Vec<Card<'a>>> {
        if n > self.cards.len() {
            None
        } else {
            let result = self.cards.split_off(self.cards.len() - n);
            Some(result)
        }
    }
}
#[derive(Debug)]
struct Hand<'a> {
    cards: Vec<Card<'a>>,
}

impl<'a> Hand<'a> {
    // fn new(cards: [Card<'static>; 5]) -> Hand {
    //     Hand { cards }
    // }
    fn paired_hand(&self) -> bool {
        if self.cards[0].rank == self.cards[1].rank {
            true
        } else {
            false
        }
    }
    
}
// this is like a new function
impl<'a> From<Vec<Card<'a>>> for Hand<'a> {
    fn from(cards: Vec<Card<'a>>) -> Self {
        Self { cards }
    }
}
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


fn check_hand(c1: &Rank, c2: &Rank) -> bool {
    if c1==c2 {
        true
    } else {
        false
    }
}

fn simulate_draw() -> bool {
    let mut deck = Box::new(Deck::new());

    // println!("Deck before shuffling: {:?}", deck);
    deck.shuffle();
    // println!("Deck after shuffling: {:?}", deck);

    let hand = deck.deal(2).map(|cards| Hand::from(cards));
    // let mut xpair = Vec::<Rank>::new();
    if let Some(dealt) = hand {
        assert_eq!(dealt.cards.len(), 2);
        println!("Dealt hand: {:?}", dealt.cards);

        // stack overflow when workin with the static lifetime classes
        // if let Some(crd1) = dealt.cards.get(0) {
        //     if let Some(crd2) = dealt.cards.get(1) {
        //         if check_hand(crd1.rank.to_owned(), crd2.rank.to_owned()) {
        //             println!("Pair found: {:?}", crd1.rank);
        //             return true;
        //         }
        //     }
        // }
    }
    //stop condition for testing not really gonna remain here
    false
}
#[tokio::main]
async fn main() {
    // let builder = thread::Builder::new().stack_size(12 * 1024 * 1024);
    // let _ = builder.spawn(|| {
    let mut iterations = 0;
    let max_iterations = 10;

    while !simulate_draw() && iterations < max_iterations {
        iterations += 1;
    }

    if iterations == max_iterations {
        println!("\n Maximum number of iterations reached.");
    }
        // thread code here
    // }).unwrap();

}

    // EVALUATE PROGRESS PRINTOUT
    //     if i % 100 == 0 {
    //         println!("Simulations completed: {}", i);
    //     }
    //
    // after running 100 sims, show stats at that point, pause 5 seconds
    // println!("Out of {} iterations, {} had a matching pair", num_iterations, matching_pairs);

    // SAVE THIS 
    // let hand = deck.deal(2).map(|cards| Hand::from(cards))