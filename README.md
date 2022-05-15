[![Tests](https://github.com/FitzOReilly/fatalii/actions/workflows/tests.yml/badge.svg)](https://github.com/FitzOReilly/fatalii/actions/workflows/tests.yml)
[![codecov](https://codecov.io/gh/FitzOReilly/fatalii/branch/main/graph/badge.svg?token=KJNHD6Z7ZM)](https://codecov.io/gh/FitzOReilly/fatalii)

UCI compatible chess engine

Challenge me on Lichess: https://lichess.org/@/FataliiBot

## Building from source
To compile the engine, you need to have [Rust](https://www.rust-lang.org/) installed. Then clone the repo
```
git clone git@github.com:FitzOReilly/fatalii.git
# or, if you prefer https:
# git clone https://github.com/FitzOReilly/fatalii.git
```
and build the engine:
```
cd fatalii
cargo build --profile release-lto
```
The binary will be in `target/release-lto/fatalii`.

## Supported features
- Bitboards using file rank mapping
- Evaluation currently solely based on piece square tables
- Alpha-Beta search
  - Quiescence search
  - Iterative deepening
  - Move ordering
    - Principal variation move
    - Hash move
    - Most valuable victim - least valueable attacker (MVV-LVA)
- Transposition table with Zobrist hashing
- Detect draws by 3-fold repetition
- Detect draws by 50 move rule

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
