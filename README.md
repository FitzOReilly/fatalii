[![Tests](https://github.com/FitzOReilly/fatalii/actions/workflows/tests.yml/badge.svg)](https://github.com/FitzOReilly/fatalii/actions/workflows/tests.yml)
[![codecov](https://codecov.io/gh/FitzOReilly/fatalii/branch/main/graph/badge.svg?token=KJNHD6Z7ZM)](https://codecov.io/gh/FitzOReilly/fatalii)

UCI compliant chess engine

## Play online
Challenge me on Lichess: https://lichess.org/@/FataliiBot

## Download
Binaries for Linux and Windows are available on the
[release page](https://github.com/FitzOReilly/fatalii/releases).

## Building from source
To build the engine from source, you need a [Rust](https://www.rust-lang.org/)
compiler. Clone the repo:
```
# Using SSH
git clone git@github.com:FitzOReilly/fatalii.git
# Alternatively, using HTTPS
git clone https://github.com/FitzOReilly/fatalii.git
```
and build the engine:
```
cd fatalii
cargo build --profile release-lto --bin fatalii
```
The binary will be in `target/release-lto/fatalii`.

## Usage
Fatalii supports the UCI protocol (universal chess interface), so it can be used
with a UCI compatible GUI. Some choices are
[Cute Chess](https://cutechess.com/), [Arena](http://www.playwitharena.de/) or
[Lucas Chess](https://lucaschess.pythonanywhere.com/).


## UCI options
- `Hash` \
  The size of the hash table in MB
- `Move Overhead` \
  Subtract this value from the movetime to compensate for network delays or GUI overheads
- `UCI_Chess960` \
  Enable Chess960 if this is set to true

## Supported variants
Fatalii supports both standard chess and Chess960 (a.k.a. Fischer Random Chess).

## Features
- Bitboards using file rank mapping
- Evaluation
  - Piece square tables (symmetrical)
  - Pawn structure: passed, isolated, backward and doubled pawns
  - Mobility
  - Bishop pair
  - Tempo
  - Tapered evaluation for all parameters
  - Tuned with training positions from the
    [Zurichess dataset quiet-labeled.v7](https://bitbucket.org/zurichess/tuner/downloads/quiet-labeled.v7.epd.gz)
- Search
  - Iterative deepening
  - Principal variation search
  - Aspiration windows
  - Quiescence search
  - Move ordering
    - Root move ordering based on the previous iteration
    - Principal variation move
    - Hash move from the transposition table (if there are multiple TT entries
      for the position, all of them will be used)
    - Queen promotions
    - Most valuable victim - least valuable attacker (MVV-LVA)
    - Killer heuristic
    - Countermove heuristic
    - History heuristic
    - Underpromotions last
  - Pruning
    - Alpha-Beta pruning
    - Null move pruning
    - Futility pruning
    - Reverse futility pruning
- Transposition table
  - Zobrist hashing
  - 4 entries per bucket
  - Replacement scheme based on entry age and depth
- Draw detection
  - 3-fold repetition
  - 50 move rule
  - Insufficient material

## Thanks to
- The [Chess Programming Wiki](https://www.chessprogramming.org). It has been
  extremely helpful during development. A lot of ideas have been taken from it
  and I've also learned a lot from it.
- Other open source chess engines.
- [Lichess](https://lichess.org/) for being an awesome chess site and for
  letting bots play there. And https://github.com/lichess-bot-devs/lichess-bot
  for making it easy to create such a bot.
- The folks at [CCRL](http://ccrl.chessdom.com/ccrl/404/) for rating the engine.
- [Cute Chess](https://cutechess.com/). The CLI was extensively used for
  self-play testing.

### More helpful resources
- The UCI specification: https://www.shredderchess.com/download/div/uci.zip
- http://talkchess.com/forum3/index.php
- https://andrewra.dev/2019/08/05/testing-in-rust-writing-to-stdout/
- Evaluation tuning
  - Andrew Grant's paper:
    https://github.com/AndyGrant/Ethereal/blob/master/Tuning.pdf
  - Good explanation of gradient descent optimization algorithms:
    https://www.ruder.io/optimizing-gradient-descent/
  - The Zurichess datasets: https://bitbucket.org/zurichess/tuner/downloads/
