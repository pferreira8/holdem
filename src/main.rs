use rand::Rng;
use std::fmt;
use std::time::Instant;
use clap::Parser;
/// arg for number of simulations to run
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short = 'n', default_value_t = 1000000)]
    num_simulations: u32,
    #[arg(short = 'r', default_value_t = 0, required = false)]
    repeat_n_sims: u32,
    // hand_lookup: String,
    // target_hand: String,
}

//DEPRecTaING PROBABLY
// impl ParseTargetHandOptions {
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
    // optional sims comes from the -r flag = "repeat"
    let optional_sims: u32 = args.repeat_n_sims;
    // run one simulation
    if optional_sims.clone() == 0 {
        simulation_builder(args.num_simulations);
    // optional arg -r was passed to repeat a custom simulation 'n' times
    } else {
        //repeat a custom simulation with n-hands optional and checking for any type of hand
        for sample_num in 0..optional_sims {
            println!("RUNNING SIMULATION #{:?}\n", sample_num + 1);
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
// simulator eval logic takes MonteCarloPairDistribution::default() as input
// and increments the sim pair results array through a match block
fn track_simulation_pairs(
    mut pair_distribution: MonteCarloPairDistribution,
    h: Hand,
) -> MonteCarloPairDistribution {
    match (h.cards[0].rank, h.cards[1].rank) {
        (Rank::Two, Rank::Two) => pair_distribution.deuces += 1,
        (Rank::Three, Rank::Three) => pair_distribution.threes += 1,
        (Rank::Four, Rank::Four) => pair_distribution.fours += 1,
        (Rank::Five, Rank::Five) => pair_distribution.fives += 1,
        (Rank::Six, Rank::Six) => pair_distribution.sixes += 1,
        (Rank::Seven, Rank::Seven) => pair_distribution.sevens += 1,
        (Rank::Eight, Rank::Eight) => pair_distribution.eights += 1,
        (Rank::Nine, Rank::Nine) => pair_distribution.nines += 1,
        (Rank::Ten, Rank::Ten) => pair_distribution.tens += 1,
        (Rank::Jack, Rank::Jack) => pair_distribution.jacks += 1,
        (Rank::Queen, Rank::Queen) => pair_distribution.queens += 1,
        (Rank::King, Rank::King) => pair_distribution.kings += 1,
        (Rank::Ace, Rank::Ace) => pair_distribution.aces += 1,
        _ => {} //skip any non-pair hand combos
    }
    //update pair distribution tracker and return it
    return pair_distribution
}
//STRUCT AND IMPL LOGIC
#[derive(Default)]
//Default value of usize is 0
struct MonteCarloPairDistribution {
    total_hands: u32,
    deuces: usize,
    threes: usize,
    fours: usize,
    fives: usize,
    sixes: usize,
    sevens: usize,
    eights: usize,
    nines: usize,
    tens: usize,
    jacks: usize,
    queens: usize,
    kings: usize,
    aces: usize,
}
impl fmt::Display for MonteCarloPairDistribution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "2️⃣: {}/{}\n3️⃣: {}/{}\n4️⃣: {}/{}\n5️⃣: {}/{}\n6️⃣: {}/{}\n7️⃣: {}/{}\n8️⃣: {}/{}\n9️⃣: {}/{}\n🔟: {}/{}\n👑: {}/{}\n👸: {}/{}\n🃏: {}/{}\n🅰️: {}/{}",
            self.deuces, self.total_hands, self.threes, self.total_hands, self.fours, self.total_hands, self.fives, self.total_hands, self.sixes, self.total_hands, self.sevens, self.total_hands, self.eights, self.total_hands, self.nines, self.total_hands, self.tens, self.total_hands, self.kings, self.total_hands, self.queens, self.total_hands, self.jacks, self.total_hands, self.aces, self.total_hands
        )
    }
}

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
    fn new(rank: &'static Rank, suit: &'static Suit) -> Card {
        Card { rank, suit }
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
        let ranks = &[
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ];
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
        vec![Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades]
    }

    fn get_rank_list() -> Vec<Rank> {
        vec![
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ]
    }

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        for i in (1..self.cards.len()).rev() {
            let j = rng.gen_range(0..i + 1);
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
    fn int_val(&self) -> f32{
        1.5
        //-> Result<HandRank, HandError>
    }
    // fn suited_hand(&self) -> bool {
    //     self.cards[0].suit == self.cards[1].suit
    // }
    // fn got_aces(&self) -> bool {
    //     self.cards[0].rank.eq(&Rank::Ace) && self.cards[1].rank.eq(&Rank::Ace)
    // }
}

//TODO IMPLEMENTATION
#[derive(Debug, PartialEq, PartialOrd)]
#[allow(dead_code)]
enum HandRank {
    HighCard(u8), 
    Pair(u8),
    TwoPair(u8),
    ThreeOfAKind(u8),
    Straight(u8),
    Flush(u8),
    FullHouse(u8),
    FourOfAKind(u8),
    StraightFlush(u8),
    RoyalFlush(u8),
}
impl HandRank {
    fn new(hand_rank: u8) -> Self {
        match hand_rank {
            1 => HandRank::HighCard(1),
            2 => HandRank::Pair(2),
            3 => HandRank::TwoPair(3),
            4 => HandRank::ThreeOfAKind(4),
            5 => HandRank::Straight(5),
            6 => HandRank::Flush(6),
            7 => HandRank::FullHouse(7),
            8 => HandRank::FourOfAKind(8),
            9 => HandRank::StraightFlush(9),
            10 => HandRank::RoyalFlush(10),
            _ => panic!("Invalid HandRank"),
        }
    }
}
// this needs to be outside the scope of hands ==>
struct EvaluateHands {}

impl EvaluateHands {
    fn evaluate(&self, hands: &Vec<Hand>) -> Vec<(Hand, f32)> {
        let mut scores: Vec<(&Hand, f32)> = Vec::new();

        for hand in hands {
            let mut score = 0.0;

            if let Some(pair_value) = hand.pair_value() {
                score += (pair_value as f32) * 2.0;
            } else {
                // Add half of the highest card value as a high card bonus
                score += (hand.highest_card_value() as f32) / 2.0;
            }

            scores.push((hand, score));
        }

        // Sort by score in descending order
        scores.sort_by(|(_, score1), (_, score2)| score2.partial_cmp(score1).unwrap());

        scores
    }
}

#[derive(Debug)]
enum HandError {
    InvalidSize(usize),
}
#[derive(Debug)]
#[allow(dead_code)]
struct Player {
    hand: Hand,
    chips: u16,
}
impl Player {
    fn new(&mut self, dealt_hand: Hand, chip_count:u16) {
        self.hand = dealt_hand;
        self.chips = chip_count;
    }
}
#[derive(Debug)]
#[allow(dead_code)]
struct Game {
    deck: Vec<Card>,
    players: Vec<Player>,
    table_cards: Vec<Card>,
}
impl Game {
    fn winning_hand(self) {
    }
}

fn simulation_builder(n_sims: u32) {
    let mut pair_tracker = MonteCarloPairDistribution::default();
    pair_tracker.total_hands = n_sims;
    let mut deck = Box::new(Deck::new());
    deck.shuffle();
    // let mut pair_tracker = 0;
    // let mut suit_tracker = 0;
    // let mut rockets = 0;
    for _ in 0..n_sims {
        let rng_hand = deck.deal(2).map(|cards| Hand::new(cards));
        // error handler to use a new deck
        if deck.cards.is_empty() {
            deck = Box::new(Deck::new());
            deck.shuffle();
        }
        match rng_hand {
            Some(Ok(sim_hand)) => {
                pair_tracker = track_simulation_pairs(pair_tracker, sim_hand);
            }
            _ => println!("No hand dealt"),
        }
    } // END FOR LOOP

    //RETURN PAIR TRACKER STRUCT
    println!("randomized hands dealt: \n {}", n_sims);
    println!("pair distribution \n {}", pair_tracker);

    // NEW PLAN
    // SHOW OUTLIER RESULTS
    
    
    //STATISTICS OUTPUT
    // println!("Out of {} iterations: \n{} had a matching pair,\n{} were suited preflop \n",
    //     n_sims,
    //     pair_tracker,
    //     suit_tracker);

    // println!("POCKET ROCKET COUNTER: {:?}", rockets);
    // println!("sample probability of aces: {:?}%", rockets as f32 / n_sims as f32);
    // println!("sample probability of any pair: {:?}%", pair_tracker as f32 / n_sims as f32);
    // println!("sample probability of any suited hand: {:?}%\n\n", suit_tracker as f32 / n_sims as f32);
}
