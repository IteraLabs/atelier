![atelier](assets/images/atelier_banner.png)

<br>

[![Crates.io][badge-crates]][url-crates]
[![Rust][badge-rust]][url-rust]
[![Build][badge-build]][url-build]
[![Docs.rs][badge-docs]][url-docs]
[![Workspace][badge-workspace]][url-workspace]
[![Apache-V2 licensed][badge-license]][url-license]

[badge-crates]: https://img.shields.io/crates/v/atelier-rs.svg
[url-crates]: https://crates.io/crates/atelier-rs

[badge-rust]: https://img.shields.io/badge/rust-1.84.1%2B-orange.svg?maxAge=3600
[url-rust]: https://github.com/iteralabs/atelier-rs

[badge-build]: https://github.com/iteralabs/atelier-rs/actions/workflows/rust.yml/badge.svg
[url-build]: https://github.com/iteralabs/atelier-rs/actions

[badge-docs]: https://docs.rs/atelier-rs/badge.svg
[url-docs]: https://docs.rs/atelier-rs

[badge-workspace]: https://img.shields.io/badge/workspace-atelier--rs-00baf5
[url-workspace]: https://github.com/iteralabs/atelier-rs

[badge-license]: https://img.shields.io/badge/license-apachev2-00baf5.svg
[url-license]: https://github.com/iteralabs/atelier/blob/develop/LICENSE

<br>

# Overview

At a high level it provides the following major components: A full orderbook granularity, stochastic process and functions for synthetic data generation, Distributed convex methods for model training/inference.

### Full Limit Order Book

From the standard representation **\(level_price, level_volume\)** Levels, to a by the level order-queue granularity **\(level_orders \[ \(order_id, order_price, order_amount\), \(order_id, order_price, order_amount\), ... \] \)** to provide a true order-driven market representation structure for enriched models.

### Stochastic Process

Stochastic process generators for rich/complex simulations, implementations include: Uniform, Brownian, Hawkes.

e## Distributed Convex Methods

Distributed Convex Methods for Linear Models Training and inference. Implementations include: Undirected Acyclic Compute Graph with Gradient Consensus. 

# Usage

## Local clone

Clone the repository

```shell
git clone
cd atelier
cargo run \
    -- --template "atelier-sync/templates/single_orderbook.toml" \
    --output-dir "./examples"
```

## Docker (recommended)

If you are using a mac with Apple sillicon, you just need to build with `--platform linux/amd64` in order to cross compile, within the OSx system, the linux vm in the container, otherwise just do not include it.

```shell
docker build \
    --platform linux/amd64 \
    --target runner \
    --file .Dockerfile \
    --tag atelier-torch \
    --no-cache . 
```

the `builder` stage, to compile the rust binary, and the `runner` stage to have a 
minimalistic container to expose a service provided by the binary execution.

Generating results by running the containerized atelier.

```shell
docker run \
    --platform linux/amd64 \
    atelier-torch \ 
    --template "templates/single_orderbook.toml" \
    --output-dir "."
```

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

# License

This project is licensed under the Apache V2 license. Any contribution intentionally submitted for inclussion in Atelier by you, shall be licensed as Apache V2, without any additional terms or conditions. 
