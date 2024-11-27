# Events

The current version supports 4 types of Market Events, as specified in the `MarketEventType` enum: 

1. `CancelLimitOrder`: To cancel a single and specified limit order.
2. `NewMarketOrder`: To create a single new market order.
3. `ModifyLimitOrder`: To modify a single and identified limit order (only the amount).
4. `NewLimitOrder`: To create a single new limit order.

Each of the events has a generation `random_<MarketEventType>_template` process that serves as a pseudo-random 
generator of instances of events for testing purposes.

For each type of the above market events, there will be one channel:

1. A `CancelLimitOrder` event goes into the `cancel_lo_channel`
2. A NewMarketOrder event goes into the `new_mo_channel`
3. A `ModifyLimitOrder` event goes into the `modify_lo_channel`
4. A `NewLimitOrder` event goes into the `new_lo_channel`

There is an `event_stream` process that can produce any number of random instances for one or all of the 
different market events (as specified by `event_template`), to then use the stream for for testing purposes
of one or all of the created `channel` instances.

Each channel will be unbounded, and also will support.
 
- Multiple producers: Can receive multiple market events (of the same type).
- Multi consumers: Can be queried by multiple execution processes (spawned by the matching engine).

