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

Setup fastchess-cli to play matches agaist different versions of justok.
Implement a _barely better move selection than random_ (maybe value captures higher?) and see if that version performs better.



### Some ideas

Chrona - The Timetraveling Tinkerer
Chrona is a small female mad scientist, that builds robots that fight for her.
At some point, Chrona discovered how to time-travel, albeit somewhat unrealiably. 
Every once in a while this lets her _have an extra turn_.


Fredrick - The Profesional 
Fredrick is a pro chess player who has been studying the game since childhood.
While Fredrick never smiles but he claims he feels joy now and then.
His deep understanding of the game lets him _see the best move in a position_ once in a while.


Golem - The Ancient
Golem is a sentient rock creature that brings life to it's surroundings.


- The Witch

