# 👌justok
A _just ok_ chess engine that supports UCI. 

### Roadmap
#### Milestone 0: Exploration
Play around with the idea of making a chess engine. Search around and read online. Watch a youtube video.

Questions to research
- What programming challenges are there to writing a chess engine.
- Would it be fun to write a chess engine? What milestones would be fun to reach?
- How large an endevour would this be? What's a reasonable timeline and do you want to follow that?

#### Milestone 1: Move generation
A chess engine that is able to generate _all legal moves given any position_.
It should be able to import and export a position using a FEN string.
Move generation is thoroughly tested using a test suite and by comparing perft move counts.
**No optimisation** is an explicit goal, only correctness is of concern.

TODOs:
- [ ] Test suite is adapted from [https://github.com/schnitzi/rampart/tree/master/src/main/resources/testcases](here).
- [ ] Make a board representation that passes all the tests above.
- [ ] Implement perft test. Use multiple starting positions.
- [ ] Make the board representation pass all perft tests.

#### Milestone 2: Universal Chess Interface
Make it possible to play against the chess engine by implementing UCI. The engine should just play random but legal moves. 

#### Milestone 3: Setup some measurements
Establish some methods for measuring key metrics of the engine. Here's some numbers we might care about:
- Time spent on move generation
- Time spent on search
- Memory usage of search
- Playing performance of the engine (ELO?).

### Creating a chess variants engine.
I think I have a good idea for a chess game.
I'm not sure though, so I want to do some design work and test some ideas.
I want to be able to test many different variations on the regular rules of chess rapidly, to search for setups that are fun and balanced.

To do so, I am writing a chess engine - with a twist!
My engine won't assume that the board is 8x8. It may be any size.
It also won't assume what pieces each player has.
Instead, the engine will know about various piece behaviours that can be composed to try out new pieces.
Like, what if the knight could also be promoted like a pawn?
Or what if the board was 7x7, there were no kings and both rooks acted like kings? With check and everything.


Piece Movements
- March: Pawns
- Step: King
- (n, m) Leap
- Slide (diagonally, orthogonally)

Piece capture modes
- On top: Every piece other than pawns.
- Forward swipe: Pawns.

Piece Abilities
- Promotion: Pawns
- Leader: The king, check and all that
- Swap: Castling

Player Abilities
- Double turn: Move twice in a turn
- Recruit: Captured pieces turn color and can be placed back on the board.
- 



Setup fastchess-cli to play matches agaist different versions of justok.
Implement a _barely better move selection than random_ (maybe value captures higher?) and see if that version performs better.

./fastchess -engine cmd=./target/release/justok name=justok-release -engine cmd=./target-old/release/justok name=justok-old format=epd -each tc=10+0.1 -rounds 1 -games 1
