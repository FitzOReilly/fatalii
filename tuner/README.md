# Using the tuner

## Building the tuner

Inside the project folder, run:

```
cargo build --profile release-lto --bin tuner
```

Create a folder outside the project structure (e.g. `tuner`) and copy the executable (`target/release-lto/tuner`) to it.

## Getting training data

Download training data, e.g. from Zurichess: https://bitbucket.org/zurichess/tuner/downloads/quiet-labeled.v7.epd.gz. Extract it and put it inside the tuner folder (e.g. `tuner/training-data`).

## Tuning evaluation parameters

Create another folder for storing tuning weights, e.g. `tuner/my-feature`.

Initialize the weights with material values only (100/300/300/500/900 for pawns/knights/bishops/rooks/queens):

```
./tuner init-default-weights --weight-file-prefix my-feature/epoch-
```

Or with the current engine weights:

```
./tuner init-engine-weights --weight-file-prefix my-feature/epoch-
```

The folder structure should now look like this:

```
tuner
├── my-feature
│   └── epoch-0000.json
├── training-data
│   └── quiet-labeled.epd
└── tuner
```

Adjust some values in `epoch-0000.json` if needed (especially the `learning_rate`) and start optimizing:

```
./tuner optimize --training-data-file training-data/quiet-labeled.epd --weight-file-prefix my-feature/epoch- --num-epochs 40 --start-epoch 0
```

The weights will be stored every 10 epochs. Then you can adjust `epoch-0040.json` and continue from there:

```
./tuner optimize --training-data-file training-data/quiet-labeled.epd --weight-file-prefix my-feature/epoch- --num-epochs 40 --start-epoch 40
```

This can be useful for starting with a higher `learning_rate` and lowering it after every couple epochs.

## Storing the results

When finished (let's say after 200 epochs), print the resulting evaluation parameters with

```
./tuner print --weight-file my-feature/epoch-0200.json
```

Or write them to a file, which is probably more useful:

```
./tuner print --weight-file my-feature/epoch-0200.json > my-feature/tuned_params.rs
```
