# Atelier - Synthetic Data Generation

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

