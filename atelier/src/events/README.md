# Events Module

## Folder structure

```
Events/ 
├── channels.rs
├── mod.rs
├── publishers.rs
├── subscribers.rs
└── templates.rs
```

1. `channels.rs`: Where events are published to, and/or, subscribed from.
2. `publishers.rs`: The publishers of the events.
3. `subscribers.rs` The subscribers of the events.
4. `templates.rs`: Definition of templates for the events and other uses.

## Market Event Types

These are defined within the `src/events/templates.rs` script, as variants of the `MarketEventType` enum, which is being created dynamically with a macro:

1. `CancelLimitOrder`: To cancel a single and specified limit order.
2. `NewMarketOrder`: To create a single new market order.
3. `ModifyLimitOrder`: To modify only the amount for a single and id-identified, limit order.
4. `NewLimitOrder`: To create a single new limit order.

## Market Event

### Main structure

The struct that is being carried as the container of the actual market event, defined in `src/events/templates.rs`, it has the 
following structure: 

```
MarketEvent { 
    event_info: EventInfo,
    event_content: EventContent,
}
```
### Event's information (kind of a meta-data) type structure

```
EventInfo {
    event_received_ts,
    event_type,
    user_Id,
}
```

### Event's content type symbolic structure

```
EventContent {
    OrderCreation,
    OrderCancelation,
    OrderModification,
}
```

The necessary contents to be included in the `EventContent` given each of the `MarketEventType` are defined like this:

| EventInfo::event_type              | EventContent::event_object     | 
|------------------------------------|--------------------------------|
| MarketEventType::CancelLimitOrder  | order_id                       |
| MarketEventType::NewMarketOrder    | market::Order                  |
| MarketEventType::ModifyLimitOrder  | order_id, order_amount         |
| MarketEventType::NewLimitOrder     | market::Order

With both the `order_id` and `order_amount` being the same type as the one in the `market::Order` struct.


# Channels

Within the `src/engine/channels.rs`, for each `MarketEventType`, there will be one particular channel:

1. A `CancelLimitOrder` event goes into the `cancel_lo_channel`.
2. A NewMarketOrder event goes into the `new_mo_channel`.
3. A `ModifyLimitOrder` event goes into the `modify_lo_channel`.
4. A `NewLimitOrder` event goes into the `new_lo_channel`.

Each channel will be unbounded and supports multiple-producers/multiple-consumers logic.

# Publishers

A market participant (exchange client) is the publisher of the MarketEvents, this interaction will be represented by the
pseudo-random generation of such `MarketEvents` by a collection of `generator` instances. Any of these can generate MarketEvents 
of any type, and thus, to publish in any of the corresponding channel.

There can be more than 1 `Publisher` at/during any given point/period of time.

# Subscribers

Every `MarketEvent` published by a `Publisher` will pass through an efficient `OrderValidity` process, then will be injected into a channel, 
from which eventually it will be picked by an instance of an `OrderRouting` process (as a subscriber), which ultimately is in charge of 
applying the `Routing Algorithm` to send the `MarketEvent` to the `ExecutionQueue` and have the `MatchingEngine` execute it and update 
the state of the `LimitOrderBook`.

