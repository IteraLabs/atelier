# CHANGELOG

## v0.0.90
---

Synthetic structure & Metrics

### Added

- `tests/generators/synthetizer.rs` with one test for the synthetic progressions generation. 
- `atelier/src/generators/synthetizer.rs` with synthetic progression generator base logic.
- `atelier/src/generators/mod.rs` to include generators module.

- `atelier/src/data/market.rs` gets an implementation of builder pattern for new instances of Order, Level and Orderbook

### Modified

- `Orderbook::synthetize` moved from `market::Orderbook` to `generators::synthetizer`
- `Orderbook::random()` is the replacement for the previous `Orderbook::synthetize`
- `process/` renamed to `generators/`

### Deleted

- `ob_gbm_synthetic.rs`, `ob_gbm_synthetic.rs`, `ob_visualizations.rs`, `probabilistic_generators.rs` where deleted from the `examples/` folder.

### Roadmap

- Orderbook.orderbook_id needs a hashed way to be generated
