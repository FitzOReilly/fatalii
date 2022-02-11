[![Tests](https://github.com/FitzOReilly/fatalii/actions/workflows/tests.yml/badge.svg)](https://github.com/FitzOReilly/fatalii/actions/workflows/tests.yml)
[![codecov](https://codecov.io/gh/FitzOReilly/fatalii/branch/main/graph/badge.svg?token=KJNHD6Z7ZM)](https://codecov.io/gh/FitzOReilly/fatalii)

UCI compatible chess engine

Challenge me on Lichess: https://lichess.org/@/FataliiBot

## Building from source
To compile the engine, you need to have [Rust](https://www.rust-lang.org/) installed. Then clone the repo and build the engine:
```
git clone git@github.com:FitzOReilly/fatalii.git
cd fatalii
cargo build --release
```
The binary will be in `target/release/fatalii`.

## Supported features
- [x] Bitboards
- [x] Alpha-Beta pruning
- [x] Quiescence search
- [x] Iterative deepening
- [x] Transposition table with Zobrist hashing

### Coming soon
- [ ] Better move ordering (to prune more nodes during the Alpha-Beta search)
- [ ] Better evaluation
- [ ] Multithreaded search

### Coming one day
- [ ] Better time management
- [ ] Magic bitboards (to speed up the move generator)

## Helpful resources
- The [Chess Programming Wiki](https://www.chessprogramming.org)
- Open source chess engines, especially Stockfish and FabChess
- https://github.com/ShailChoksi/lichess-bot
- https://andrewra.dev/2019/08/05/testing-in-rust-writing-to-stdout/
