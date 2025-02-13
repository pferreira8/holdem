use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
use core::cell::RefCell;

use aya_poker::base::ParseError;
use rand::Rng;
use clap::Parser;
use aya_poker::base::Hand as AyaHand;
use aya_poker::base::Card as AyaCard;
pub mod aya;
use aya::equity_calculator;

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
    pub fn init(&mut self) -> Result<Self, Box<dyn Error>> {
        self.initial_blinds()?;
        
        self.initial_deal()?;

        self.start_turn()?;


        // while self.player_log.len() < self.active_player_count {
        //     // appends to player log, which will eventually equal the amount of active players to 
        //     self.decision_event();
        // }
        // self.decision_event();
        self.deal_flop()?;

        // self.deal_turn_or_river()?;
        // self.deal_turn_or_river()?;
        // passing internal state to the GameMaster.players field
        // adds flop hands to players cards in a Vec for determining who has the best hand
        self.update_game_status();


        // begin inner loop of player decisions

        // HAND OVER 
        Ok(self.to_owned())
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


    fn initial_deal(&mut self) -> Result<(), Box<dyn Error>> {
        // DEAL OUT PLAYER HANDS
        for p in &mut self.players {
            let rng_hand = self.gamestate.deck.deal(2).map(|cards| Hand::new(cards));

            if let Some(h) = rng_hand {
                match h {
                    Ok(h) => {
                        p.add_hand(h);

                        self.gamestate.add_player(p.clone())?;
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
                    let p = self.players[0].clone().hand.unwrap();
                    let o = self.players[1].clone().hand.unwrap();

                    let equity = equity_calculator(
                        &p.to_aya_sim().unwrap(), 
                        &o.to_aya_sim().unwrap(),
                        &h.to_aya_sim().unwrap()
                    );

                    println!(
                        "{} has {:.1}% equity on {:?} against {}.",
                        p.to_string(),
                        100.0 * equity,
                        h.cards,
                        o.to_string()
                    );
                }
                Err(err) => {
                    eprintln!("{:?}", err);
                }
            }
        }
        Ok(())
    }

    pub fn deal_turn_or_river(&mut self) -> Result<(), Box<dyn Error>> {
        //burn card
        let _ = &self.gamestate.deck.deal(1);
        let turn = self.gamestate.deck.deal(1).map(|cards| Hand::new(cards));
        
        // ADD FLOP TO GAME OBJECT
        if let Some(h) = turn {
            match h {
                Ok(h) => {
                    if let Some(card_vec) = self.gamestate.table_cards.borrow_mut() {
                        for c in h.cards {
                            card_vec.push(c.to_owned());
                        }
                    }
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
    fn _get_all_options() -> Vec<PlayerDecisions> {
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Card {
    pub rank: &'static Rank,
    pub suit: &'static Suit,
}

impl Card {
    pub fn new(rank: &'static Rank, suit: &'static Suit) -> Card {
        Card { rank, suit }
    }

    pub fn as_string(&self) -> String {
        format!("{}{}", self.rank.to_string(), self.suit.to_string())
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
    fn _paired_hand(&self) -> bool {
        self.cards[0].rank == self.cards[1].rank
    }

    pub fn to_string(&self) -> String {
        self.cards
            .iter()
            .map(|card| format!("{}{}", card.rank.to_string(), card.suit.to_string()))
            .collect::<Vec<String>>()
            .join(" ")
    }
    
    pub fn to_aya_sim(&self) -> Result<AyaHand, ParseError> {
        self.cards.iter()
            .map(|c| c.as_string().parse::<AyaCard>())
            .collect::<Result<AyaHand, ParseError>>()
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
    pub hand: Option<Hand>,
    pub chips: i32,
    pub hand_equity: f64,
    pub is_turn: bool,
}
impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name: Some(name),
            hand: None,
            chips: 0,
            hand_equity: 0.0,
            is_turn: false
        }

    }

    pub fn add_hand(&mut self, h: Hand) {
        // self.hand.replace(Rc::new(RefCell::new(h)));
        self.hand.replace(h);
    }
    // experimental
    pub fn update_hand(&mut self, h: Hand) {
        let tmp = self.hand.clone();
        if let Some(ref_hand) = tmp {
            let mut mt = ref_hand.cards.clone();
            for table_card in h.cards {
                mt.push(table_card);
            }
            // essentially rebuild hand with flop added to it 
            
            // self.hand = Some(Rc::new(RefCell::new(Hand::new(mt).unwrap())));
            self.hand = Some(Hand::new(mt).unwrap());
            
        }
    }

    pub fn get_cards_svg(self) -> Vec<String> {
        let mut card_file_paths: Vec<String> = Vec::new();
        if let Some(h) = self.hand.clone() {
            let cards = h.cards.clone();
            
            for c in cards {
                let rank_char = match c.rank {
                    Rank::Ace => "A",
                    Rank::King => "K", 
                    Rank::Queen => "Q",
                    Rank::Jack => "J",
                    Rank::Ten => "T",
                    Rank::Nine => "9",
                    Rank::Eight => "8",
                    Rank::Seven => "7",
                    Rank::Six => "6",
                    Rank::Five => "5",
                    Rank::Four => "4",
                    Rank::Three => "3",
                    Rank::Two => "2"
                };
                let suit_char = match c.suit {
                    Suit::Clubs => "C",
                    Suit::Diamonds => "D",
                    Suit::Hearts => "H",
                    Suit::Spades => "S"
                };
                card_file_paths.push(rank_char.to_owned()+suit_char+".svg");
            }
        }
        
        card_file_paths

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
                println!("CARDS:        {:?}", p.hand.expect("issue accessing player data"));
                println!("HAND EQUITY VALUE:   {:?}", p.hand_equity);
                println!("CHIP VALUE:   {:?}", p.chips);
                println!("\n");
            }
        }
   
        
    }
    // LATER
    // pub fn winning_hand(self) {}
    pub fn select_cards_svg(cards: Vec<Card>) -> Vec<String> {
        let mut card_file_paths: Vec<String> = Vec::new();

        for c in cards {
            let rank_char = match c.rank {
                Rank::Ace => "A",
                Rank::King => "K", 
                Rank::Queen => "Q",
                Rank::Jack => "J",
                Rank::Ten => "T",
                Rank::Nine => "9",
                Rank::Eight => "8",
                Rank::Seven => "7",
                Rank::Six => "6",
                Rank::Five => "5",
                Rank::Four => "4",
                Rank::Three => "3",
                Rank::Two => "2"
            };
            let suit_char = match c.suit {
                Suit::Clubs => "C",
                Suit::Diamonds => "D",
                Suit::Hearts => "H",
                Suit::Spades => "S"
            };
            card_file_paths.push(rank_char.to_owned()+suit_char+".svg");
        }
        
        
        card_file_paths

    }
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

}



#[derive(Debug, PartialEq, Clone, Parser)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}
impl Suit {
    pub fn to_string(&self) -> &'static str {
        match self {
            Suit::Hearts => "h",
            Suit::Diamonds => "d",
            Suit::Clubs => "c",
            Suit::Spades => "s",
        }
    }
}


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

impl Rank {
    pub fn to_string(&self) -> &'static str {
        match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "T",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        }
    }
}