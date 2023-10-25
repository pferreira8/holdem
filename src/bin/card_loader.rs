use iced::ContentFit;
use iced::widget::{checkbox, column, container, Svg, svg, row};
use iced::{color, Element, Length, Sandbox, Settings};
use holdem::{HandScore, Game, Player, Hand, GameMaster};



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
    card_files: Vec<String>,
    apply_color_filter: bool,
    game_state: GameMaster,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    NewHandAlert(bool),
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
        let hero = game_handler.gamestate.get_player("phil").unwrap();
        let card_files = hero.get_cards_svg();
        println!("hero cards: {:?}", card_files);
        CardVisualizer { 
            card_files: card_files, 
            apply_color_filter: false,
            game_state: g_copy
         }

    }

    fn title(&self) -> String {
        String::from("SVG - Iced")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::NewHandAlert(apply_color_filter) => {
                self.apply_color_filter = apply_color_filter;
                
                self.game_state = GameMaster::new(Game::new(), self.game_state.clone().players);
                self.game_state.init().unwrap();
                let hero = self.game_state.clone().gamestate.get_player("phil").unwrap();
                
                // let mut v: Vec<String> = Vec::new();
                // self.card_files.clear();
                self.card_files = hero.get_cards_svg();
                println!("UPDATED: {:?}", self.card_files);
                // self.card_files = v;
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {

        let handle = svg::Handle::from_path(format!(
            "{}/img/custom/{}",
            env!("CARGO_MANIFEST_DIR"),
            self.card_files[0]
        ));
        let handle2 = svg::Handle::from_path(format!(
            "{}/img/custom/{}",
            env!("CARGO_MANIFEST_DIR"),
            self.card_files[1]
        ));

        let svg_card = svg(handle).content_fit(ContentFit::ScaleDown);
        let svg_card2: Svg = svg(handle2).content_fit(ContentFit::ScaleDown);

        let apply_color_filter = checkbox(
            "Click For New Hand",
            self.apply_color_filter,
            Message::NewHandAlert,
        );
        // let w: Window = Window::default();
        // println!("size: {:?}", w.size.0);
        container(
            column![
                row![
                svg_card,
                svg_card2,
                ],
                container(apply_color_filter).width(Length::Fill).height(Length::Fill).center_x()
            ]
        )
        // .padding(w.size.0 as f32/4.0)
        .align_y(iced::alignment::Vertical::Bottom)
        .align_x(iced::alignment::Horizontal::Center)
        .into()
    }
}

