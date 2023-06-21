use std::error::Error;

use rand::Rng;
use clap::Parser;

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
    pub fn high_card_eval(self) -> u8 {
        // Assign points for high cards
        return match self.rank {
            Rank::Two => 1,
            Rank::Three => 2,
            Rank::Four => 3,
            Rank::Five => 4,
            Rank::Six => 5,
            Rank::Seven => 6,
            Rank::Eight => 7,
            Rank::Nine => 8,
            Rank::Ten => 9,
            Rank::Jack => 10,
            Rank::Queen => 11,
            Rank::King => 12,
            Rank::Ace => 13,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
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
        let mut cards = Vec::new();
        for suit in suits {
            for rank in ranks {
                cards.push(Card::new(rank, suit));
            }
        }
        Deck { cards }
    }
    #[allow(dead_code)]
    fn get_suits() -> Vec<Suit> {
        vec![Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades]
    }
    #[allow(dead_code)]
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
}


#[derive(Debug, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
}
impl Hand {
    pub fn new(cards: Vec<Card>) -> Result<Self, HandError> {
        if cards.len() > 3 {
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


struct EvaluateHands {}

impl EvaluateHands {
    fn evaluate(hand: Option<Rc<RefCell<Hand>>>) -> Option<HandRank> {
        let mut suited_bonus = 0;
        if let Some(h) = hand {
            let x1 = h.borrow().cards[0];
            let x2 = h.borrow().cards[1];

            let pair_score = match (x1.rank, x2.rank) {
                (Rank::Two, Rank::Two) => Some(2),
                (Rank::Three, Rank::Three) => Some(4),
                (Rank::Four, Rank::Four) => Some(6),
                (Rank::Five, Rank::Five) => Some(8),
                (Rank::Six, Rank::Six) => Some(10),
                (Rank::Seven, Rank::Seven) => Some(12),
                (Rank::Eight, Rank::Eight) => Some(14),
                (Rank::Nine, Rank::Nine) => Some(16),
                (Rank::Ten, Rank::Ten) => Some(18),
                (Rank::Jack, Rank::Jack) => Some(20),
                (Rank::Queen, Rank::Queen) => Some(22),
                (Rank::King, Rank::King) => Some(24),
                (Rank::Ace, Rank::Ace) => Some(26),
                _ => None, // Skip any non-pair hand combos
            };

            if let Some(score) = pair_score {
                // this will only be called if a pair exists
                return Some(HandRank::Pair(score*100));
            } else {
                // at this point we know we don't have a piar
                let s1 = x1.high_card_eval();
                let s2 = x2.high_card_eval();
                if x1.suit == x2.suit {
                    suited_bonus = 10;
                }
                Some(HandRank::HighCard(s1 + s2 + suited_bonus))
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

use std::rc::{Rc};
use core::cell::RefCell;
#[derive(Debug, Clone)]
pub struct Player {
    pub name: Option<String>,
    pub hand: Option<Rc<RefCell<Hand>>>,
    pub chips: Option<u16>,
    pub hand_value: Option<HandRank>,
}
impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name: Some(name),
            hand: None,
            chips: None,
            hand_value: None,
        }

    }
    pub fn eval_hand(&mut self) {
        self.hand_value = EvaluateHands::evaluate(self.hand.clone()) ;
    }
    pub fn add_hand(&mut self, h: Hand) {
        self.hand.replace(Rc::new(RefCell::new(h)));
    }
}
#[derive(Debug, Clone)]
pub struct Game {
    pub deck: Box<Deck>,
    pub player_ct: Option<usize>,
    pub players: Option<Vec<Player>>,
    pub table_cards: Option<Vec<Card>>,
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
        }
    }

    pub fn show_players(self) {
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




