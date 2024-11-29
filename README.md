# atelier

A Computational Workshop for Market Microstructure Modeling, Synthetic Simulation and Historical Replay.

## Overview

Use `atelier` for modeling market microstructure dynamics, it supports an `orderbook-based` market structure. Whether you need a high quality reproduction of any given market within a defined period of time (market replay), or, to generate `what-if` scnearios either completely random, or, with model specification (market simulation). 

## Contribute

Feel free to contribute, just make sure you check the [CONTRIBUTING]() guidelines. Also consider to pick-up the existing [issues]().

## Roadmap

### Order struct

Incorporate new fields in the `Order` struct:

- `customer_id`: To link the order to the uniquely corresponding customer that generated it.
- `asset_id`: To link the order to the uniquely corresponding asset (even if is a pair).
- `partial_fill`: To indicate if the filling of the `Order` can be partial, or, is it has to ve fully filled at once.
- `expiration_ts`: Time units (must be a globally consistent nanosecs) of the order getting automatically cancel if not filled.

Upgrade existing fields:

- Design and implement a protocol to have a standardize, informative and memory efficient `order_id`.

### Order Fill result

Design and implement a new struct to hold result data after:

an Order fill is produced: 

- order is partially filled (two recipients per amount-matched, maker and taker)
- order is fully filled (two recipients per amount-matched, for all amounts, maker and taker)

an Order gets cancel: 

- order is canceled (one recipient, maker)
- order expires (one recipient, maker)

### Success/Error messages

- Create and implement the acknowledgement messages of success for all engine operations.
- Finish the implementation of the error messages for all engine operations.

### Fee schedule

- Create a new struct to hold data about trading fees, bps for maker and for taker, in tiers.
- Create discount mechanism to charge fees.

