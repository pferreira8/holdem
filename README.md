# Texas-Hold'em Draw Simulator
This is a work-in-progress. Currently, it runs a Monte-Carlo simulation to showcase the pair distribution. This project is primarily for me to understand the variance impact of # of hands dealt in a simulation.

You can pass the number of simulations you wish to perform as a command-line argument or in the main function.

To make life easier, you can throw this in a bash script to run the program:
```
file-example name holdem.sh
cargo build --release && cargo run --release

execute holdem.sh in bash shell
sh holdem.sh
```

## Args
-n -> number of simulations to run  (default value: 1,000,000) \n
-r -> repeat a specified simulation an arbitrary number of times (default value: 0)
                    
###updated benchmark: 
it takes ~87ms to evaluate a pair distribution of one million dealt hands.

###$ To-Do
Build a dealer class that manages state for each simulation and encapsulate the logic within that.
Add the option to select a predetermined hand and then evaluate its occurrence over n hands dealt.
Create a full game simulator that doesn't factor in betting. It takes a parameter 'x' for the number of players and proceeds to deal hands and evaluate hand rank preflop and also on each street determine winning odds of hand at all stages of the game, including dead cards. Track results via a dictionary approach (player #, tuple(bool-> winning hand?, hand)).

####$ Process Logic
The simulator deals to players (pop 2n+1) cards from the deck to account for player cards 2(n), plus an initial burn card.

Community cards are split into flop, turn, and river and are stored in separate Vec variables. The following sequence is encapsulated into a function that modifies the Deck variable: pop burn-card, push flop, pop burn, push turn, pop burn, push river.

The flop, turn, and river are then joined through Vec.
