
# Engine Module

Within this module exists the definition of the validation processes for income events, routers for async order management, the logic of the matching engine. This module is organized in the following files:

1. `validation.rs`:
2. `routing.rs`: Order Routing Algorithms.
3. `matching.rs`: Matching Engine.

## System Design Taxonomy

Concurrent Publisher/Subscriber Pattern that uses unbounded and multi-producer/multi-consumer channels, with atomic events as messages, A routing algorithm to gather and inject the event's data into an execution queue to be consumed by an execution engine that ultimately alters the state represented in the Limit Order Book data structure. 

## General Description

- Inbound Events Pipeline (connectors, channels, producers):

An External Data Source produces data, then a connector acting as the Internal API endpoint fetches that data, either through a querie using a REST connector, or through a received data package using a WebSocket connector. Then data goes through a validation/pre-processing layer (to make it compliant with the event-message schema). After validation, data is pre-processed for compliance with the event-message schema and passed into a `Publisher` so it can publish the event into the corresponding channel. 

- Internal Routing:

Since there are different channels to take events from, an Order Routing Strategy will decide the order of the channels and the amount of events to be extracted from each of them, to then be sent to the Global Executable Tasks Queue.

- Execution:

From the Executable Tasks Queue, it will consume tasks acording to the Execution Strategy, and if successfully executed, it will effectively change the state of the Order Book.

## Communication Channels `channels.rs`

*(pending)*

## Order Queues `queues.rs`

- Cancel Limit Orders Queue: `cancel-lo-queue`
Supports Multiple Queues for the Cancel of Limit Orders requests, each queue covers a statict but config-defined
range of levels.

- New Market Orders Queue: `new-mo-queue`
Supports A single global queue for the New Market Order requests.

- Modifiy Limit Orders Queue: `modify-lo-queue`
Supports Multiple Queues for the Modification of Limit Orders requests, each queue covers a static but config-defined
range of levels.

- New Limit Order Queue: `new-lo-queue`
Supports Multiple Queues for the New Limit Order requests, each queue covers a static but config-defined 
range of levels.

## Order Management Interface `management.rs`

### Cancel Limit Order Queue (First priority):
A `cancel-lo-queue` contains the cancel requests only for limit orders because market orders are meant to be
instant, support for other type of orders is not mapped but should be considered.

Operational interactions among `omi`, the `queue-routing` algorithm, `cancel-lo-queue` nth-queue and `order-events` channel:

1. `omi` recieves the "cancel limit order request".
2. `omi` logs the message 'cancel limit order received'.
3. `omi` runs the 'queue-routing' for a "cancel limit order request" algorithm 
4. `omi` logs the message 'order route calculated'.
5. `omi` routes the order to the corresponding `cancel-lo-queue` nth-queue.
6. `omi` publishes the message 'cancel limit order routed' into the `order-events` channel.

### New Market Order Queue (Second priority):
The `new-mo-queue` Market orders to be executed with FIFO logic: First In, First Out. 

Operational interactions:

1. `omi` recieves the "new market order request"
2. `omi` routes the order to the only global `new-mo-queue` queue.
3. `omi` publishes the message `new market order routed` into the order-events channel.
4. `omi` logs the message `new market order recieved and routed`

### Modify Limit Order Queue (Third priority):
The `modify-lo-queue` contains limit orders to be modified, solely the amount, any other field is not modifiable.

Operational interactions:

1. `omi` recieves the "modify limit order request".
2. `omi` logs the message 'modify limit order received'.
3. `omi` runs the 'queue-routing' for a "modify limit order request" algorithm 
4. `omi` logs the message 'order route calculated'.
5. `omi` routes the order to the corresponding `modify-lo-queue` nth-queue.
6. `omi` publishes the message 'modify limit order routed' into the `order-events` channel.

### New Limit Orders queue (Fourth priority):
The `new-lo-queue` contains the new limit orders, to be either fully or partially filled. 

Operational interactions:

1. `omi` recieves the "new limit order request"
2. `omi` runs the 'queue-routing' for a "new limit order request" algorithm 
3. `omi` routes the order to the corresponding `new-lo-queue` nth-queue.
4. `omi` publishes the message 'new limit order routed' into the `order-events` channel.
5. `omi` logs the message `new market order recieved and routed`

## Matching Engine `matching.rs`

*(pending)*

