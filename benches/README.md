# benches

# Overview

Benchmarks for the atelier-rs engine.

## Single order book generation

For both sides of the orderbook, these are the boundaries to the pseudo-random
generation. 

### Params

- Levels: [2, 20, 50, 100]
- Orders: [(5,10), (100, 200), (500, 700), (1000, 1300)]

e.g. There will be a case where the generated orderbook will have 2 levels per side, and a random number (Uniformly distributed) of orders from 
2 up to 10. Or, the other extreme, 100 levels for each side, and, for each level, an amount of orders that could go from 1,000 up to 1,300 orders
per level, thus, it will be measured the time it takes to generate an orderbook with up to 1,300,000 randomly generated orders in it.

To run the benchmark

```shell
cargo bench --color=never 2>&1 | tee benches/orderbook_benchmark.txt
```
and this is an example of the output, as saved in the `orderbook_benchmark_raw.txt` file

```shell

```

### Plots

Criterion.rs automatically generates detailed plots as part of its benchmarking process. Here's how to access and use them:

Using macOS.

```
open target/criterion/report/index.html
```

Using Linux.

```
xdg-open target/criterion/report/index.html
```

## PDF and SD

| PDF (times)               |  Histogram Std dev (times)    |
|:-------------------------:|:------------------------------:
|<img src="https://raw.githubusercontent.com/IteraLabs/atelier/refs/heads/main/assets/images/benches/orderbook_generation/b_l_100_b_o_Some((1000%2C%201300))_a_l_100_a_o_Some((1000%2C%201300))_1.svg" width = "500" height = "500"> | <img src="https://raw.githubusercontent.com/IteraLabs/atelier/refs/heads/main/assets/images/benches/orderbook_generation/b_l_100_b_o_Some((1000%2C%201300))_a_l_100_a_o_Some((1000%2C%201300))_2.svg" width = "500" height = "500">
| <img src="https://raw.githubusercontent.com/IteraLabs/atelier/refs/heads/main/assets/images/benches/orderbook_generation/b_l_2_b_o_Some((5%2C%2010))_a_l_2_a_o_Some((5%2C%2010))_1.svg" width = "500" height="500"> | <img src="https://raw.githubusercontent.com/IteraLabs/atelier/refs/heads/main/assets/images/benches/orderbook_generation/b_l_2_b_o_Some((5%2C%2010))_a_l_2_a_o_Some((5%2C%2010))_2.svg" width = "500" height = "500"> |


### Cases

|    Case    | Levels per side | Orders per level | Median Timen to Execute (Milliseconds) |
|:----------:|:---------------:|:----------------:|:--------------------------------------:|
| Shallowest |        2        |  (1,000 : 1,300) |                  0.002                 |
| Deepest    |       100       |  (1,000 : 1,300) |                  11.78                 |

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
benches is a member of the [atelier-rs](https://github.com/iteralabs/atelier-rs) workspace

