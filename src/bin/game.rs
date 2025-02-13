#[allow(unused_imports)]
use holdem::{Game, Player, Hand, GameMaster};
use std::error::Error;


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
/*
Texas Hold'Em poker game simulator
This program simulates poker games with up to 8 players, 
using a lightweight implementation of the game rules. 

TODO
decision making process in CLI
for both player and AI the decision framework should be modular

wasm GUI

 */
fn main() -> Result<(), Box< dyn Error>> {
    // let hs = HandScore::new();
    // hs.display_point_values();
    let player_group = setup_players(Some(3));

    // main logic encapsulated in Game struct
    let mut game_handler = GameMaster::new(Game::new(), player_group);

    // game top level function
    game_handler.init()?;

    Ok(())

}