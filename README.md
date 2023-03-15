# Texas-Hold'em Draw Simulator
This is a work-in-progress. Currently, it can detect pairs and suited hands. You can pass the number of simulations you wish to perform as a command-line argument or in the main function.

To make life easier, you can throw this in a bash script to run the program:
```
cargo build --release && cargo run --release
```
release benchmark: it takes ~309ms to evaluate one million dealt hands.

## To-Do
Build a dealer class that manages state for each simulation and encapsulate the logic within that.
Add the option to select a predetermined hand and then evaluate its occurrence over n hands dealt.
Create a full game simulator that doesn't factor in betting. It takes a parameter 'x' for the number of players and proceeds to deal hands and evaluate hand rank preflop and also on each street determine winning odds of hand at all stages of the game, including dead cards. Track results via a dictionary approach (player #, tuple(bool-> winning hand?, hand)).

### Process Logic
The simulator deals to players (pop 2n+1) cards from the deck to account for player cards 2(n), plus an initial burn card.

Community cards are split into flop, turn, and river and are stored in separate Vec variables. The following sequence is encapsulated into a function that modifies the Deck variable: pop burn-card, push flop, pop burn, push turn, pop burn, push river.

The flop, turn, and river are then joined through Vec.
