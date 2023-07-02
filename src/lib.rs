use std::collections::HashMap;
use std::error::Error;
use std::rc::{Rc};
use core::cell::RefCell;

use rand::Rng;
use clap::Parser;
#[derive(Debug, Clone)]

// there is a difference between the players field in this struct and the internal Game.players
// by using a Hashmap to keep track of player turn and SB/BB chip updating through a key index
pub struct GameMaster {
    pub gamestate: Game,
    pub players: Vec<Player>,
    pub small_blind: Option<i32>,
    pub big_blind: Option<i32>,
    pub bb_position: usize,
    pub player_log: Vec<PlayerDecisions>,
    pub turn_index: usize,
    pub active_player_count: usize,
}
impl GameMaster {
    pub fn new(g: Game, vp: Vec<Player>) -> Self {

        let mut table_layout: HashMap<usize, Player> = HashMap::new();
        for (index, p) in vp.clone().iter().enumerate() {
            table_layout.insert(index, p.clone());
        }

        let player_log: Vec<PlayerDecisions> = Vec::new();
        let size = vp.len();
        Self {
            gamestate: g,
            players: vp,
            small_blind: Some(10),
            big_blind: Some(20),
            bb_position: 1,
            player_log: player_log,
            turn_index: 2,
            active_player_count: size

        }
    }
    pub fn init(&mut self) -> Result<(), Box<dyn Error>> {
        self.initial_blinds()?;
        
        self.initial_deal()?;

        self.start_turn()?;


        while self.player_log.len() < self.active_player_count {
            // appends to player log, which will eventually equal the amount of active players to 
            self.decision_event();
        }
        // self.decision_event();
        self.deal_flop()?;

 
        // passing internal state to the GameMaster.players field
        // adds flop hands to players cards in a Vec for determining who has the best hand
        self.update_game_status();


        // begin inner loop of player decisions

        // HAND OVER 
        Ok(())
    }

    pub fn update_game_status(&mut self) {
        println!("TABLE CARDS: {:?} \n ", self.gamestate.table_cards);
        if let Some(players_state) = self.gamestate.players.clone() {
            // copying the players from an internal state to a global state
            self.players = players_state;
        }

        for p in &self.players {
            println!("PLAYER HAND: {:?} \n", p);
        }
    }
    // INTERNAL FUNCTIONS

    fn start_turn(&mut self) -> Result<(), Box<dyn Error>> {
        assert_eq!(self.turn_index, self.bb_position+1);

        let first_to_act = self.players.get_mut(self.turn_index);
        if let Some(first) = first_to_act {
            first.is_turn = true;
        }
        Ok(())
    }

    fn pass_turn(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn initial_deal(&mut self) -> Result<(), Box<dyn Error>> {
        // DEAL OUT PLAYER HANDS
        for mut p in self.players.clone() {
            let rng_hand = self.gamestate.deck.deal(2).map(|cards| Hand::new(cards));

            if let Some(h) = rng_hand {
                match h {
                    Ok(h) => {
                        p.add_hand(h);
                        // eval hand assigns the initial score attribute to each Player
                        p.eval_hand();

                        self.gamestate.add_player(p)?;
                    }
                    Err(err) => {
                        eprintln!("{:?}", err);
                    }
                }
            }
        }
        Ok(())
    }
    fn initial_blinds(&mut self) -> Result<(), Box<dyn Error>> {
        let bb = self.players.get_mut(self.bb_position);
        if let Some(first_big) = bb {
            first_big.chips -= self.big_blind.expect("BIG BLIND NOT FOUND");
        }
        let sb = self.players.get_mut(self.bb_position-1);
        if let Some(first_small) = sb {
            first_small.chips -= self.small_blind.expect("SMALL BLIND NOT FOUND");
        }
        Ok(())

    }
    fn deal_flop(&mut self) -> Result<(), Box<dyn Error>> {
        // burn a card
        let _ = &self.gamestate.deck.deal(1); // does not need to be assigned to a hand like below

        let flop = self.gamestate.deck.deal(3).map(|cards| Hand::new(cards));
        
        // ADD FLOP TO GAME OBJECT
        if let Some(h) = flop {
            match h {
                Ok(h) => {
                    self.gamestate.table_cards = Some(h.clone().cards);

                    if let Some(updated_players) = self.gamestate.players.as_mut() {
                        for p in updated_players {
                            p.update_hand(h.clone());
                            // eval is broken at this stage
                            // need an update_eval type function
                            p.eval_hand();
                        }
                    }
                    // EXPERIMENTAL CODE ABOVE
                    // COPYING FLOP CARDS INTO EACH PLAYERS HAND
                    // THIS WILL MAKE IT EASIER TO LOOP
                    // AND DO A MAXIMIZING FUNCTION 
                    // THIS SHIT BROKE RN
                    // REPLACES HAND WITH FLOP CARDS
                }
                Err(err) => {
                    eprintln!("{:?}", err);
                }
            }
        }
        Ok(())
    }

    fn decision_event(&mut self) {

    }
}
#[derive(Debug, Clone)]
pub struct DecisionHandler {

}

#[derive(Debug, Clone)]
pub enum PlayerDecisions {
    Call,
    Bet,
    Raise,
    Shove,
    Check,
    Fold
}

impl PlayerDecisions {
    fn get_all_options() -> Vec<PlayerDecisions> {
        vec![
            PlayerDecisions::Call,
            PlayerDecisions::Bet,
            PlayerDecisions::Check,
            PlayerDecisions::Raise,
            PlayerDecisions::Shove,
            PlayerDecisions::Fold
        ]
    }
}
/*

To calculate the probability of 
making a pair before the flop, 
you divide the number of ways to make a pair -> 78 
by the total number of possible combinations -> 1,326:
1 in 17, or 5.88%.
 */
#[derive(Debug, PartialEq, Clone, Parser)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}
impl Suit {
    #[allow(dead_code)]
    fn randomize_suit() -> Suit {
        let mut suit_rng = rand::thread_rng();
        let suits = Deck::get_suits();
        let rand_suit = suit_rng.gen_range(0..suits.len());
        suits.get(rand_suit).unwrap().to_owned()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub rank: &'static Rank,
    pub suit: &'static Suit,
}

impl Card {
    pub fn new(rank: &'static Rank, suit: &'static Suit) -> Card {
        Card { rank, suit }
    }
    pub fn high_card_eval(self) -> f32 {
        // Assign points for high cards
        return match self.rank {
            Rank::Two => 1.0,
            Rank::Three => 2.0,
            Rank::Four => 3.0,
            Rank::Five => 4.0,
            Rank::Six => 5.5,
            Rank::Seven => 6.5,
            Rank::Eight => 7.5,
            Rank::Nine => 8.5,
            Rank::Ten => 10.0,
            Rank::Jack => 11.0,
            Rank::Queen => 14.0,
            Rank::King => 16.0,
            Rank::Ace => 18.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
    pub deck_count: usize,
    // ranks: &'a [Rank],
    // suits: &'a [Suit],
}

impl Deck {
    //ranks: &'a [Rank], suits: &'a [Suit]
    pub fn new() -> Deck {
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
        let mut cards_vec = Vec::new();
        for suit in suits {
            for rank in ranks {
                cards_vec.push(Card::new(rank, suit));
            }
        }
        let count = cards_vec.len();
        assert_eq!(count, 52);
        Deck { 
            cards: cards_vec,
            deck_count: count, //this will mutate as the cards are dealt
        }
    }

    // shows the state of current deck
    // after hands are dealt,
    // before flop,
    // after flop, 
    // post turn / pre-river
    pub fn check_current_deck(self) -> Vec<Card> {
        println!("CARDS IN DECK: {:?} \n", self.cards.len());
        self.cards
    }
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        for i in (1..self.cards.len()).rev() {
            let j = rng.gen_range(0..i + 1);
            self.cards.swap(i, j);
        }
    }

    pub fn deal(&mut self, n: usize) -> Option<Vec<Card>> {
        if n > self.cards.len() {
            None
        } else {
            let result = self.cards.split_off(self.cards.len() - n);
            Some(result)
        }
    }

    fn get_suits() -> Vec<Suit> {
        vec![Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades]
    }
    // #[allow(dead_code)]
    // fn get_rank_list() -> Vec<Rank> {
    //     vec![
    //         Rank::Ace,
    //         Rank::Two,
    //         Rank::Three,
    //         Rank::Four,
    //         Rank::Five,
    //         Rank::Six,
    //         Rank::Seven,
    //         Rank::Eight,
    //         Rank::Nine,
    //         Rank::Ten,
    //         Rank::Jack,
    //         Rank::Queen,
    //         Rank::King,
    //     ]
    // }

}


#[derive(Debug, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
}
impl Hand {
    pub fn new(cards: Vec<Card>) -> Result<Self, HandError> {
        if cards.len() > 7 {
            return Err(HandError::InvalidSize(cards.len()));
        }
        Ok(Self { cards })
    }
    fn paired_hand(&self) -> bool {
        self.cards[0].rank == self.cards[1].rank
    }
    // fn int_val(&self) -> f64 {
    //     1.5
    // }
    // fn suited_hand(&self) -> bool {
    //     self.cards[0].suit == self.cards[1].suit
    // }
    // fn got_aces(&self) -> bool {
    //     self.cards[0].rank.eq(&Rank::Ace) && self.cards[1].rank.eq(&Rank::Ace)
    // }
}
#[derive(Debug, Copy, Clone)]
pub enum HandRank {
    HighCard(f32),
    Pair(f32),
    TwoPair(f32),
    ThreeOfAKind(f32),
    Straight(f32),
    Flush(f32),
    FullHouse(f32),
    FourOfAKind(f32),
    StraightFlush(f32),
    RoyalFlush(f32),
}


struct EvaluateHands {

}

impl EvaluateHands {
    fn evaluate(hand: Option<Rc<RefCell<Hand>>>) -> Option<HandRank> {
        let mut suited_bonus: f32 = 0.0;
        let mut score = 0.0;
        if let Some(h) = hand {
            let x1 = h.borrow().cards[0];
            let x2 = h.borrow().cards[1];

            let pair_score = match (x1.rank, x2.rank) {
                (Rank::Two, Rank::Two) => Some(2.0),
                (Rank::Three, Rank::Three) => Some(4.0),
                (Rank::Four, Rank::Four) => Some(6.0),
                (Rank::Five, Rank::Five) => Some(8.0),
                (Rank::Six, Rank::Six) => Some(10.0),
                (Rank::Seven, Rank::Seven) => Some(12.0),
                (Rank::Eight, Rank::Eight) => Some(14.0),
                (Rank::Nine, Rank::Nine) => Some(16.0),
                (Rank::Ten, Rank::Ten) => Some(18.0),
                (Rank::Jack, Rank::Jack) => Some(20.0),
                (Rank::Queen, Rank::Queen) => Some(22.0),
                (Rank::King, Rank::King) => Some(24.0),
                (Rank::Ace, Rank::Ace) => Some(26.0),
                _ => None, // Skip any non-pair hand combos
            };

            if let Some(score) = pair_score {
                // this will only be called if a pair exists
                return Some(HandRank::Pair(score*100.0));
            } else {
                // at this point we know we don't have a piar
                let s1 = x1.high_card_eval();
                let s2 = x2.high_card_eval();
                if x1.suit == x2.suit {
                    suited_bonus = 10.0;
                }
                score = s1+s2+suited_bonus;
                Some(HandRank::HighCard(score))
            }
        } 
        else {
            // should not reach this 
            // if so, option was not accessed properly.
            None
        }


    }


}



#[derive(Debug)]
pub enum HandError {
    InvalidSize(usize),
}



// GAME STRUCT



#[derive(Debug, Clone)]
pub struct Player {
    pub name: Option<String>,
    pub hand: Option<Rc<RefCell<Hand>>>,
    pub chips: i32,
    pub hand_value: Option<HandRank>,
    pub is_turn: bool,
}
impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name: Some(name),
            hand: None,
            chips: 0,
            hand_value: None,
            is_turn: false
        }

    }
    pub fn eval_hand(&mut self) {
        self.hand_value = EvaluateHands::evaluate(self.hand.clone()) ;
    }
    pub fn add_hand(&mut self, h: Hand) {
        self.hand.replace(Rc::new(RefCell::new(h)));
    }
    // experimental
    pub fn update_hand(&mut self, h: Hand) {
        let tmp = self.hand.clone();
        if let Some(ref_hand) = tmp {
            let mut mt = ref_hand.borrow().cards.clone();
            for flop_card in h.cards {
                mt.push(flop_card);
            }
            // essentially rebuild hand with flop added to it 
            self.hand = Some(Rc::new(RefCell::new(Hand::new(mt).unwrap())));
            
        }
    }
}
#[derive(Debug, Clone)]
pub struct Game {
    pub deck: Box<Deck>,
    pub player_ct: Option<usize>,
    pub players: Option<Vec<Player>>,
    pub table_cards: Option<Vec<Card>>,
    pub hand_list: Option<Vec<Hand>>
}
impl Game {
    pub fn new() -> Self {
        let player_list = Vec::<Player>::new();
        let mut d = Box::new(Deck::new());
        d.shuffle();
        let n_players = 8;
        Self {
            deck: d,
            player_ct: Some(n_players),
            players: Some(player_list),
            table_cards: None,
            hand_list: None,
        }
    }

    pub fn show_players(self) {
        println!("PLAYER COUNT: {:?} \n", self.player_ct);
        if let Some(ls) = self.players {
            for p in ls {
                println!("\n");
                println!("PLAYER NAME:  {:?}", p.name.expect("issue accessing player data"));
                println!("CARDS:        {:?}", p.hand.expect("issue accessing player data").borrow());
                println!("HAND VALUE:   {:?}", p.hand_value.expect("issue accessing player data"));
                println!("CHIP VALUE:   {:?}", p.chips);
                println!("\n");
            }
        }
   
        
    }
    // LATER
    // pub fn winning_hand(self) {}

    pub fn get_player(self, name: &str) -> Option<Player> {
        for p in self.players.unwrap() {
            if p.name == Some(name.to_string()) {
                return Some(p)
            } 
        }
        None
    }
    pub fn add_player(&mut self, p: Player) -> Result<(), Box<dyn Error>> {
        if let Some(ref mut players) = self.players {
            players.push(p);
            Ok(())
        } else {
            Err("Players vector not initialized".into())
        }
    }
    //pseudo code 
    // evaluate(player.get.hand)
    // return ordered ranking of player hands
}

// pub struct HandEvaluator {
//     hand: Hand
// }
#[derive(Debug, PartialEq)]
pub enum Rank {
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
pub struct HandScore {
    value_scoring: HandType,

}

impl HandScore {
    pub fn new() -> Self {
        let ht = HandType::new();
        Self {
            value_scoring: ht
        }
    }

    pub fn display_point_values(&self) {
        println!("{:?}", self.value_scoring);
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct HandType {
    high_card: u8,
    pair: u8,
    two_pair: u8,
    trips: u8,
    straight: u8,
    flush: u8,
    full_house: u8, 
    quads: u8,
    straight_flush: u8,
    royal_flush: u8,
}
impl HandType {
    pub fn new() -> Self {
        Self {
            high_card: 1,
            pair: 2,
            two_pair: 3,
            trips: 4,
            straight: 5,
            flush: 6,
            full_house: 7,
            quads: 8,
            straight_flush: 9,
            royal_flush: 10,
        }
    }
}
/*
IMPORTANT 
two-tiered scoring system

we can evaluate the point totals for situations where straight vs straight occurs,
the max sum of all evaluated hands will be best
at a specific level this can be pruned
if we know for example best hand is in a higher rank indicated by 
multiplying the base score, 

EXAMPLE

Pair of 4s
4pts each
8*2 = 16 point hand value

TwoPair
4s and 3s
(2 is the pair rank multiple) 
4*2 + 3*2 = 14 
        -> 14 * 3 (two pair rank multiple of 3) = 42

10s and Jacks
10 + 11 -> ans * 3 = 63
42 * 3 = 126

Jacks and Queens
(11 + 12) * 3 = 69

Aces and Kings (top range of two pairs)
14 + 13 
* 3
= 81

at minimum sets must be > 81
Set of 2s
n*rank_multiple squared
(2*5)^2
10 * 10 = 100

but this means for sets, (14*5)^2 now is the min value
of straight hands

//Straights

4-5-6-7-8

4+5+6+7+8 = 30*5 ^2

2+3+4+5+6 = 20*5 ^2
need to recognize A2345 as worse point value


 */




