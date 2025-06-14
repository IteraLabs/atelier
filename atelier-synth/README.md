# atelier-synth

# Overview

Synthetic Data Generation for the atelier-rs engine.

## Synthetic Data

Currently, the way to generate data is by starting with some basic definitions
according to the templated configuration toml files. For every market, there needs
to be defined the following sections: 

1. `[[experiments]]`: information about the experiment of synthetic data generation. 
2. `[[models]]`: One model per exchange since there will be generated one order book per exchange as well.
3. `[[exchanges]]`: Besides basic info like `id`, or, `name`, the sub-table `[exchanges.orderbook]` will contain the particular elements to generate the progressions of suc order book associated to the given exchange. 

## Experiments

Simple content like `id` and `n_progressions`.

## Models

Since each order book is generated using one model, here are the parameters
necessary for the model used.

## Exchanges 

Order book generation base parameters like `bid_price` and `ask_price` as the initial
prices to start the generation. 

# Workspace

These are the other published crates members of the workspace: 

- [atelier-data](https://crates.io/crates/atelier-data): Core data structures and I/O tools.
- [atelier-dcml](https://crates.io/crates/atelier-dcml): Distributed Convex Machine Learning. 
- [atelier-generators](https://crates.io/crates/atelier-generators): Probabilistic generators and events simulation.
- [atelier-results](https://crates.io/crates/atelier-results): Standardized results, errors and successes.
- [atelier-synth](https://crates.io/crates/atelier-synth): Synthetic Data Generation for the atelier-rs engine.

Github hosted:

- [benches](https://github.com/IteraLabs/atelier-rs/tree/main/benches)
- [examples](https://github.com/IteraLabs/atelier-rs/tree/main/examples)
- [tests](https://github.com/IteraLabs/atelier-rs/tree/main/tests)

<br>

---
atelier-synth is a member of the [atelier-rs](https://github.com/iteralabs/atelier-rs) workspace
