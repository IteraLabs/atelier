# atelier-demm

Decentralized Market Making Modeling.

# Dynamic Spread Strategy OnChain

## Description

it has some initial values, a definition of models

## Initial values

- **base_inventory**: f64 = Amount of tokens for the base part of the price.
- **quote_inventory**: f64 = Amount of tokens for the quote part of the price.
- **ref_ask_spread**: f64 = Initial value for ask spread calculation (decimal value representing a percentage).
- **ref_bid_spread**: f64 = Initial value for bid spread calculation (decimal value representing a percentage).
- **ref_history**: u32 = The amount of trade history used for the reference price computation.
- **priority_gas_fee**: f64 = Amount of tokens to pay priority fee (gas fees) of the network to process the transaction.

## Models

- **ask_spread_model** = (function, parameters)(step, value) -> f32
- **ask_amount_model** = (function, parameters)(step, active_trades) -> f32
- **bid_spread_model** = (function, parameters)(step, value) -> f32
- **bid_amount_model** = (function, parameters)(step, active_trades) -> f32
- **fee_model** = (function, parameters)(priority_fee) -> f32
- **ref_price_model** = (function, parameters)(active_trades)

## Metrics 

- **cumulative_trades**: To track accumulated all executed trades (does not clear after reset).
- **cumulative_fees**: To track accumulated all payed fees (does not clear after reset).
- **time_since_reset**: To track time units since the last reset (clears at every reset).
- **drawdown**: Capital drawdown based on on open trades and current price.
- **roic**: Return Over Invested Capital. Current market value of the position over the initial capital value.
- **resets**: To track the amount of resets that took place.

### Pseudo-code

step 0 : 

- initialize models: 
    - ask_spread_model, ask_amount_model
    - bid_spread_model, bid_amount_model
    - fee_model, ref_price_model

- initialize variables: 
    - base_inventory
    - quote_inventory
    - ref_ask_spread
    - ref_bid_spread
    - priority_gas_fee
    - ref_history

- initialize metrics:
    - cumulative_trades
    - cumulative_fees
    - time_since_reset
    - reset_occurrences
    - metric_drawdown
    - metric_roic

- initialize trackers:
    - active_trades = []
    - ref_price = market_price

- Compute:
    - ask_price = ref_price * (1 + ask_spread_model(step, ref_ask_spread))
    - bid_price = ref_price * (1 - bid_spread_model(step, ref_bid_spread))

step 1..N:

- market_price == ref_price:
    - reset event

- market_price =< bid_price:
    - yes:
        - buy trade event

            # fields
            - order_price = bid_price
            - order_amount = bid_amount_model(step, active_trades[ref_history])
            - order_side = +1 # +1: BUY, -1: SELL
            - order_type = DEX
            
            # processes
            fn_order_formation(order_price, order_amount, order_side, order_type) -> Order
            fn_order_fill(Order) -> Trade
            fn_inventory_metrics_update(Trade) -> InventoryMetrics
            fn_financial_metrics_update(Trade) -> FinancialMetrics
            fn_active_trades_update(Trade) -> ActiveTrades
            fn_ref_price_update(ActiveTrades) -> RefPrice

- market_price >= ask_price
    - yes: 
        - sell trade event

            # fields
            - order_price = ask_price
            - order_amount = ask_amount_model(step, active_trades[ref_history])
            - order_side = -1 # +1: BUY, -1: SELL
            - order_type = DEX
           
            # processes
            fn_order_formation(order_price, order_amount, order_side, order_type) -> Order
            fn_order_fill(Order) -> Trade
            fn_inventory_metrics_update(Trade) -> InventoryMetrics
            fn_financial_metrics_update(Trade) -> FinancialMetrics
            fn_active_trades_update(Trade) -> ActiveTrades
            fn_ref_price_update(ActiveTrades) -> RefPrice

    - no: continue
        - update indicators
            - time_since_reset += 1 time unit
    
    
- reference_price <= bid_price
    no: continue
    yes: buy trade event



### Parameters

bid_amount: No. of base tokens to use for the next trade
bid_spread: % of price distance from reference price
ask_amount: No. of quote tokens to use for the next trade
ask_spread: % of price distance from the reference price

historical_trades_window: Amount of trades to compute the reference price

### Calculations

trades_vwap: Using trades executed prices and amounts, calculate the vwap, given a predefined historical trades window.

### State tracking metrics

base_inventory: Cumulative amount of the base tokens acquired
quote_inventory: Cumulative amount of the quote tokens acquired
cum_trades: Cumulative number of executed trades
time_since_reset: time since the last inventory reset

### Financial Performance Metrics

drawdown: tracker of the position value in comparisson with the openning one.
roi: Revenue over invested capital.
traded volume (base): Amount of traded base tokens
traded volume (quote): Amount of traded quote tokens
no_resets: Amount of inventory resets

