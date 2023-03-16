
use std::time::Instant;
use rand::Rng;
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

use clap::Parser;
/// arg for number of simulations to run
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short = 'n', default_value_t = 1000000)]
    num_simulations: u32,
    #[arg(short = 'r', default_value_t = 0, required=false)]
    repeat_n_sims: u32,
    // hand_lookup: String,
    // target_hand: String,
}
// struct TargetHand {
//     aces: String,
//     kings: String, 
    
// }

// impl TargetHand {
//     fn new(target: &str) {
//         match target {
//             "aa" | "AA" => {
//                 // let c1 = Card::new(&Rank::Ace, &Suit::generate_random());
//                 // let c2 = Card::new(&Rank::Ace, &Suit::generate_random());
//                 // Hand::new(vec![c1, c2]).unwrap()
//             }
//             _ => {

//             }
//         }

//     }
// }

fn main() {
    let args = Args::parse();
    let start_time = Instant::now();
    let optional_sims: u32 = args.repeat_n_sims;
    // run one simulation
    if optional_sims.clone() == 0 {
        simulation_builder(args.num_simulations);
    // optional arg -r was passed to repeat a custom simulation 'n' times
    } else {
        //repeat a custom simulation with n-hands optional and checking for any type of hand
        for sample_num in 0..optional_sims {
            println!("RUNNING SIMULATION #{:?}\n", sample_num+1);
           simulation_builder(args.num_simulations);
        }
    }
        
    track_runtime(start_time);
}
fn track_runtime(pass_start: Instant) {
    let end_time = Instant::now();
    let duration = end_time.duration_since(pass_start);

    println!("Time elapsed: {:?}", duration);
}

//STRUCT AND IMPL LOGIC
#[derive(Debug, PartialEq, Clone, Parser)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}
impl Suit {
    fn randomize_suit() -> Suit {
        let mut suit_rng = rand::thread_rng();
        let suits = Deck::get_suits();
        let rand_suit = suit_rng.gen_range(0..suits.len());
        suits.get(rand_suit).unwrap().to_owned()
}
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
    
    fn get_suits() -> Vec<Suit> {
        vec![Suit::Clubs, Suit::Diamonds, 
            Suit::Hearts, Suit::Spades]
    }
    
    fn get_rank_list() -> Vec<Rank> {
        vec![Rank::Ace, Rank::Two, Rank::Three, 
            Rank::Four, Rank::Five, Rank::Six, 
            Rank::Seven, Rank::Eight, Rank::Nine, 
            Rank::Ten, Rank::Jack, Rank::Queen, Rank::King]
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
    fn got_aces(&self) -> bool {
        self.cards[0].rank.eq(&Rank::Ace) && self.cards[1].rank.eq(&Rank::Ace)
    }
}

fn simulation_builder(n_sims: u32)  {
    let mut deck = Box::new(Deck::new());
    deck.shuffle();
    let mut pair_tracker = 0;
    let mut suit_tracker = 0;
    let mut rockets = 0;
    for _ in 0..n_sims {    
        let rng_hand = deck.deal(2).map(|cards| Hand::new(cards));
        // error handler to use a new deck
        if deck.cards.is_empty() {
            deck = Box::new(Deck::new());
            deck.shuffle();

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
                if hand.got_aces() {
                    rockets+=1;
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
        
    println!("POCKET ROCKET COUNTER: {:?}", rockets);
    println!("sample probability of aces: {:?}%", rockets as f32 / n_sims as f32);
    println!("sample probability of any pair: {:?}%", pair_tracker as f32 / n_sims as f32);
    println!("sample probability of any suited hand: {:?}%\n\n", suit_tracker as f32 / n_sims as f32);
}
