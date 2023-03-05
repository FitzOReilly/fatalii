[![Tests](https://github.com/FitzOReilly/fatalii/actions/workflows/tests.yml/badge.svg)](https://github.com/FitzOReilly/fatalii/actions/workflows/tests.yml)
[![codecov](https://codecov.io/gh/FitzOReilly/fatalii/branch/main/graph/badge.svg?token=KJNHD6Z7ZM)](https://codecov.io/gh/FitzOReilly/fatalii)

UCI compliant chess engine

## Play online
Challenge me on Lichess: https://lichess.org/@/FataliiBot

## Usage
Fatalii supports the UCI protocol (universal chess interface), so it can be used with a UCI compatible GUI.
Some choices are [Cute Chess](https://cutechess.com/), [Arena](http://www.playwitharena.de/) or
[Lucas Chess](https://lucaschess.pythonanywhere.com/).

## UCI options
- `Hash` \
  The size of the hash table in MB
- `Move Overhead` \
  Subtract this value from the movetime to compensate for network delays or GUI overheads
- `UCI_Chess960` \
  Enable Chess960 if this is set to true

## Building from source
To compile the engine, you need a [Rust](https://www.rust-lang.org/) compiler. Then clone the repo
```
git clone https://github.com/FitzOReilly/fatalii.git
```
and build the engine:
```
cd fatalii
cargo build --profile release-lto
```
The binary will be in `target/release-lto/fatalii`.

## Supported variants
Fatalii supports both standard chess and Chess960 (a.k.a. Fischer Random Chess).

## Features
- Bitboards using file rank mapping
- Evaluation
  - Piece square tables
  - Tempo
- Search
  - Iterative deepening
  - Principal variation search
  - Quiescence search
  - Move ordering
    - Principal variation move
    - Hash move
    - Queen promotions
    - Most valuable victim - least valuable attacker (MVV-LVA)
    - Killer heuristic
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
  letting bots play there. And https://github.com/ShailChoksi/lichess-bot for
  making it easy to create such a bot.
- The folks at [CCRL](http://ccrl.chessdom.com/ccrl/404/) for rating the engine.
- [Cute Chess](https://cutechess.com/). The CLI was extensively used for
  self-play testing.

### More helpful resources
- The UCI specification: https://www.shredderchess.com/download/div/uci.zip
- http://talkchess.com/forum3/index.php
- https://andrewra.dev/2019/08/05/testing-in-rust-writing-to-stdout/
