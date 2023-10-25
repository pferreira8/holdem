# Texas-Hold'em Project
GUI/eval-engine port/ is a major work in progress. So far this is the scaffolding of a free platform to play poker against AI.
Long term vision:
    enabling server-hosting
    smart contract vesting for buy-ins and payouts on Solana
    
## Monte-Carlo Args
You can pass the number of simulations you wish to perform as a command-line argument or override the default args.

To make running custom sims easier, you can throw them in a bash script to run the program:
```
command example | ex. script -> holdem.sh
cargo build --release && cargo run --release --bin holdem -- -n 1000 -r 5

```

### Args
```
-n -> number of simulations to run  
(default value: 1,000,000)


-r -> repeat a specified simulation an arbitrary number of times 
(default value: 0)
```                    
### Last Benchmark: 
~87ms process 1 million hands.

### To-Do
Add the option to select a predetermined hand and then count its occurrence over n hands dealt.

#### Process Logic
The simulator deals to n players, removing 2n+1 cards from the deck to account for player cards, plus the initial burn card.


