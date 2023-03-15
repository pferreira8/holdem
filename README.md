# holdem
Texas-Hold'em draw simulator
╔╗      ╔╗   ╔╗╔╗        
║║      ║║   ║║║║        
║╚═╗╔══╗║║ ╔═╝║╚╝╔══╗╔╗╔╗
║╔╗║║╔╗║║║ ║╔╗║  ║╔╗║║╚╝║
║║║║║╚╝║║╚╗║╚╝║  ║║═╣║║║║
╚╝╚╝╚══╝╚═╝╚══╝  ╚══╝╚╩╩╝
                         
                         

W.I.P
throw this in a bash script to make life easier:
  cargo build --release && cargo run --release
  
OVERVIEW:
simulator is in early-build stage 
for now it detects pairs and suited hands.

pass the # of simulations you wish to perform as a command-line arg or in the main function.

release benchmark:
evaluate 1 million dealt hands ~= 309ms

simple todo:
would make sense to build a dealer class that manages state for each simulation
and encapsulate the logic within that

add the option to select a predetermined hand 
and then evaluate its occurence over n hands dealt.

advanced todo:
full game simulator that doesn't factor in betting
takes parameter 'x' for number of players
proceeds to deal hands and evaluate hand rank preflop and also on each street
determine winning odds of hand at all stages of game including dead cards
----------------------------------------------------------------------------


Track results via dictionary approach 
  (player #, tuple(bool-> winning hand?, hand)


Process Logic:
  deal to players
    (pop 2n+1) cards from the deck
    to account for player cards 2(n), plus initial burn card.
    
  Community Cards:
  split flop, turn, river into seperate Vec<Card> variables
  
  encapsulate the following sequence into a function that modifies Deck variable
    pop burn-card, push flop, pop burn, push turn, pop burn, push river
  
  join flop, turn, and river through Vec<Card> 

