# atelier

A Computational Workshop for Market Microstructure Modeling, Synthetic Simulation and Historical Replay.

## Overview

Use `atelier` for modeling market microstructure dynamics, it supports an `orderbook-based` market structure. Whether you need a high quality reproduction of any given market within a defined period of time (market replay), or, to generate `what-if` scnearios either completely random, or, with model specification (market simulation). 

## Application Case Taxonomy

A discrete, time-bounded, a-priori event-generation approach to produce synthetic progressions of the market microstructure.

- **discrete**: The unit of change is Order Events, an event-driven approach, event-by-event, with "procedural" agents. i.e. agents contain conditions for behavior but without an evaluation of outcomes.

- **time-bounded**: Each step length is pre-defined and always known, not necessarily constant, and the scale of time is also defined a-priori, not bounded by the time it takes for the computations to occur. i.e. The time step value is arbitrarly chosen, events are synched within each step.

- **a-priori**: Order Events happen within a pre-defined time period, so the generation of the events is done in a step-by-step basis, and they are generated computationally independent but synched within each step.

Steps for implementation

1. Define parameters of the progression
    1. No. of progressions: u32
    2. No. of participating agents: u32
    3. time-step : u128 | Vec<u128>

2. Define parameters for each agent
    1. agent_id: created randomly.
    2. agent_type: decided according to a particular purpose.
    3. agent_balance: decided according to a particular purpose.

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

