use clap::Parser;
use std::fmt;
use std::time::Instant;
use holdem::{Rank, Deck, Hand};
/// arg for number of simulations to run
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of hands dealt in a sim
    #[arg(short = 'n', default_value_t = 1000000)]
    num_simulations: u32,
    ///Amount of sims to run
    #[arg(short = 'r', default_value_t = 1, required = false)]
    repeat_n_sims: u32,
}

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

// IMPLEMENT CODE FROM bin/game.rs
// fn run_game_sim() {
//     let g = Game::new();
//     let mut deck = Box::new(Deck::new());
//     // let mut pair_tracker = 0;
//     // let mut suit_tracker = 0;
//     // let mut rockets = 0;
//     for _ in 0..g.player_ct.unwrap() {
//         let rng_hand = deck.deal(2).map(|cards| Hand::new(cards));
//         // error handler to use a new deck
//         if deck.cards.is_empty() {
//             deck = Box::new(Deck::new());
//             deck.shuffle();
//         }
//         match rng_hand {
//             Some(Ok(sim_hand)) => {

//             }
//             _ => println!("No hand dealt"),
//         }
//     } // END FOR LOOP
// }


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
    return pair_distribution;
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
            "TWOS: {}/{}\n THREES: {}/{}\n FOURS: {}/{}\n FIVES: {}/{}\n SIXES: {}/{}\n SEVENS: {}/{}\n EIGHTS: {}/{}\n NINES: {}/{}\n TENS: {}/{}\n JACKS: {}/{}\n QUEENS: {}/{}\n KINGS: {}/{}\n ACES: {}/{}",
            self.deuces, self.total_hands, self.threes, self.total_hands, self.fours, self.total_hands, self.fives, self.total_hands, self.sixes, self.total_hands, self.sevens, self.total_hands, self.eights, self.total_hands, self.nines, self.total_hands, self.tens, self.total_hands, self.kings, self.total_hands, self.queens, self.total_hands, self.jacks, self.total_hands, self.aces, self.total_hands
        )
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
}
