use holdem::{HandScore, Game, Player, Hand, GameMaster};
use std::{error::Error};


fn setup_players(count: Option<u8>) -> Vec<Player> {
    let mut vp: Vec<Player> = Vec::new();
    let hero = Player::new("phil".to_string() );
    vp.push(hero);

    if let Some(ct) = count {
        // define a specific amount of players 
        for _ in 0..ct {
            vp.push(Player::new("test".to_string()));
        }
    } else {
        //default to 8 player table
        for _ in 0..7 {
            vp.push(Player::new("test".to_string()));
        }

       
    }
    vp
}
fn main() -> Result<(), Box< dyn Error>> {
    // let hs = HandScore::new();
    // hs.display_point_values();
    let player_group = setup_players(Some(3 as u8));
    // main logic encapsulated in Game struct
    let mut game_handler = GameMaster::new(Game::new(), player_group);

    game_handler.play()?;

    for p in game_handler.clone().gamestate.players.unwrap() {
        println!("{:?}", p);
    }
    // let x = game_handler.gamestate.clone();
    // x.show_players();
    // x.deck.check_current_deck();
    game_handler.update_game_status();
    Ok(())

}