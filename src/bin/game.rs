use holdem::{HandScore, Game, Player, Hand};
use std::borrow::{BorrowMut};
use std::{error::Error};


fn setup_players() -> Vec<Player> {
    let tplayer = Player::new("jay".to_string() );
    let tplayer2 = Player::new("phil".to_string() );

    vec![tplayer, tplayer2]
}
fn main() -> Result<(), Box< dyn Error>> {
    let hs = HandScore::new();
    hs.display_point_values();

    // main logic encapsulated in Game struct
    let mut g = Game::new();

    let player_group = setup_players();

    for mut p in player_group {
        let rng_hand = g.borrow_mut().deck.deal(2).map(|cards| Hand::new(cards));

        if let Some(h) = rng_hand {
            match h {
                Ok(h) => {
                    p.add_hand(h);
                    p.eval_hand();

                    g.add_player(p)?;
                }
                Err(err) => {
                    eprintln!("{:?}", err);
                }
            }
        }
    }

    let flop = g.borrow_mut().deck.deal(3).map(|cards| Hand::new(cards));
    
    // ADD FLOP TO GAME OBJECT
    if let Some(h) = flop {
        match h {
            Ok(h) => {
                g.table_cards = Some(h.cards);
            }
            Err(err) => {
                eprintln!("{:?}", err);
            }
        }
    }
    // NOW THE FLOP IS WITHIN the g : Game object
    g.show_players();

    Ok(())

}