use std::collections::HashMap;
use iced::{ContentFit, Renderer};
use iced::widget::{checkbox, column, container, Svg, svg, row, Row, button, Checkbox, Button};
use iced::{Element, Length, Sandbox, Settings};
#[allow(unused_imports)]
use holdem::{Game, Player, Hand, GameMaster};
use iced_winit::core::svg::Handle;



fn setup_players(count: Option<u8>) -> Vec<Player> {
    let mut vp: Vec<Player> = Vec::new();
    let hero = Player::new("phil".to_string() );
    vp.push(hero);

    if let Some(ct) = count {
        // define a specific amount of players 
        for idx in 0..ct {
            vp.push(Player::new(format!("test_player{}", idx).to_string()));
        }
    } else {
        //default to 8 player table
        for idx in 0..7 {
            vp.push(Player::new(format!("test_player{}", idx).to_string()));
        }
       
    }
    vp
}

pub fn main() -> iced::Result {
    CardVisualizer::run(Settings::default())
}

#[derive(Debug)]
struct CardVisualizer {
    // users hand
    card_files: Vec<String>,
    player_hands: HashMap<Option<String>, Vec<String>>,
    table_card_files: Vec<String>,
    game_over: bool,
    game_state: GameMaster,
    deal_card: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    StartNewHand(bool),
    CardDealt(bool)
}

impl Sandbox for CardVisualizer {
    type Message = Message;

    fn new() -> Self {
        let player_group = setup_players(Some(3));

        // main logic encapsulated in Game struct
        let mut game_handler = GameMaster::new(Game::new(), player_group);
    
        // game top level function
        game_handler.init().unwrap();
        let g_copy = game_handler.clone();
        //need to clone above because we are getting player
        //TEST
        let mut player_cards: HashMap<Option<String>, Vec<String>> = HashMap::new();
        if let Some(players) =  game_handler.gamestate.players {
            for p in players {
                player_cards.insert(p.clone().name, p.get_cards_svg());
            }

        }
        //END TEST
        let hero = player_cards.get(&Some("phil".to_string())).unwrap();
        let card_files = hero;
        println!("hero cards: {:?}", card_files);
        CardVisualizer { 
            card_files: card_files.to_vec(), 
            player_hands: player_cards,
            table_card_files: Vec::with_capacity(5),
            game_over: false,
            game_state: g_copy,
            deal_card: false
         }

    }

    fn title(&self) -> String {
        String::from("Texas Holdem")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::StartNewHand(starting_next_game) => {
                self.game_over = starting_next_game;
                
                self.game_state = GameMaster::new(Game::new(), self.game_state.clone().players);
                self.game_state.init().unwrap();
                let hero = self.game_state.clone().gamestate.get_player("phil").unwrap();

                self.card_files = hero.get_cards_svg();
                println!("UPDATED: {:?}", self.card_files);
                // self.card_files = v;
            }
            Message::CardDealt(deal_new_card) => {
                self.deal_card = deal_new_card;
                if self.deal_card {
                    self.game_state.deal_turn_or_river().unwrap();
                    self.table_card_files = Game::select_cards_svg(self.game_state.gamestate.table_cards.clone().unwrap());

                }
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let mut table_svg: Vec<String> = Vec::new();
        if let Some(tbl_cards) = self.game_state.gamestate.table_cards.clone() {
            table_svg = Game::select_cards_svg(tbl_cards);
        } 
        let mut v: Vec<Vec<String>> = Vec::new();
        for h in self.player_hands.clone().values() {
            v.push(h.to_owned());
        }
        container_builder(self.card_files.clone(), v, table_svg)
        


    }
}

fn svg_path_setup(card_file: String) -> Svg {

    let generate_handle = svg::Handle::from_path(format!(
        "{}/img/custom/{}",
        env!("CARGO_MANIFEST_DIR"),
        card_file
    ));

    svg(generate_handle).content_fit(ContentFit::ScaleDown)
}
//Vec<(Svg, Svg)>
fn add_player_hands(hand_group: Vec<Vec<String>>) -> Vec<(Svg, Svg)> {
    let mut card_images: Vec<(Svg, Svg)> = Vec::new();
    for hand in hand_group {
        if let (Some(c1), Some(c2)) = (hand.get(0), hand.get(1)) {
            let cimg1: Svg = Svg::new(Handle::from_path(c1));
            let cimg2: Svg = Svg::new(Handle::from_path(c2));
            card_images.push((cimg1, cimg2));
        }
    }

    
    

    return card_images
}
fn container_builder(player_hand: Vec<String>, group_cards: Vec<Vec<String>>, table_cards: Vec<String>) -> Element<'static, Message> {
    // functions triggered through Messages
    let start_next_game: Checkbox<'static, Message, Renderer> = checkbox(
        "Click For New Hand",
        false,
        Message::StartNewHand,
    );

    let deal_turn_or_river: Button<'static, Message, Renderer> = button(
        "Deal Turn/River").on_press(Message::CardDealt(true));
    //

    //player cards
    let pc = player_hand.as_slice();
    //table cards
    let tc = table_cards.as_slice();

    // ugly but necessary
    let mut p1: Svg = Svg::new(Handle::from_path(""));
    let mut p2: Svg = Svg::new(Handle::from_path(""));
    let mut t1: Svg = Svg::new(Handle::from_path(""));
    let mut t2: Svg = Svg::new(Handle::from_path(""));
    let mut t3: Svg = Svg::new(Handle::from_path(""));
    let mut t4: Svg = Svg::new(Handle::from_path(""));
    let mut t5: Svg = Svg::new(Handle::from_path(""));

    if let (Some(_card1), Some(_card2)) = ( pc.get(0), pc.get(1) ) {
        p1 = svg_path_setup(_card1.to_owned());
        p2 = svg_path_setup(_card2.to_owned());

    }
    if let ( Some(_card1), Some(_card2), Some(_card3), _card4, _card5 ) = 
    ( tc.get(0), tc.get(1), tc.get(2), tc.get(3), tc.get(4)) 
    {
        
        t1 = svg_path_setup(_card1.to_owned());
        t2 = svg_path_setup(_card2.to_owned());
        t3 = svg_path_setup(_card3.to_owned());
        t4 = svg_path_setup(_card4.unwrap_or(&"".to_string()).to_owned());
        t5 = svg_path_setup(_card5.unwrap_or(&"".to_string()).to_owned());
    }  

    let row_player_cards = add_player_hands(group_cards);
    let mut player_displayed_cards: Row<'_, Message, Renderer> = Row::new();
    for p in row_player_cards {
        player_displayed_cards = row![
            p.0,
            p.1
        ]
    }
    let tr: Row<'_, Message, Renderer> = match table_cards.len() {
        3 => {
            row![
                t1,t2,t3
            ]
        }
        4 => {
            row![
                t1,t2,t3,t4
            ]
        }
        5 => {
            row![
                t1, t2, t3, t4, t5
            ]
        }
        _ => {
            row![t1, t2, t3, t4, t5]
        }
    };
    container(
        column![
            tr,
            row![
                p1,
                p2
            ].padding(20), 
            player_displayed_cards,
            container(start_next_game).width(Length::Fill).height(Length::Fill).center_x(),
            container(deal_turn_or_river).width(Length::Fill).height(Length::Fill).center_x()
        ],
        
    )
    // .padding(w.size.0 as f32/4.0)
    .align_y(iced::alignment::Vertical::Bottom)
    .align_x(iced::alignment::Horizontal::Center)
    .into()
}
