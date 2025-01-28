
# Agent

The _agents_ module provides the functionality to deliver an _agentic_ way to implement an 
event-driven approach for generating synthetic progressions of a market. 


## TL;DR

In the context of Atelier, here is a naive but practical definition you can start with.

Consider an _agent_ as a piece of code that basically has a way to _perceive_ states of an _environment_, map those into _actions_, and will do this in order to maximize a _reward_ function. In the context of atelier, the environment is a market, say btcusdt in Binance. The way to _perceive_ it is subscribing to streams of limit order book events, then it conducts some actions in the form of data modeling and decision making, then takes an action in the form of, for example, sending a new buy market order. Next it could execute this in a loop of requesting the market price and do nothing until it is higher than its entry price. To then ultimately do it again, any number of times, in order to increase the account balance it is operating.

An _agent_ is capable of: 
- **perceptions**: receive market data from a stream (e.g. orderbook updates), etc.
- **actions**: ask for market data, compute metrics, send orders, etc.
- **map perceptions into actions**: after a new orderbook is received, compute a metric, compare to a reference, make a decision, send order.
- **receive a reward**: an economic profit after liquidate a position and calculate balance.

An environment:

A financial market is a collection of orders, trades, orderbook structure, public trades structure, etc.

- A live financial market, accessed through a data subscription, like a websocket to Btc/Usdt in Binance.
- A synthetic financial Market, like one generating using only generated data.
 

## In depth definitions.

Although we do call this features _agentic_, such term tend to be more broad and informal than we would like to in the scientific community, not to mention the vast amount of other semantically closer terms like _agent_, _autonomous agent_ and so on. And yet, here at the same paragraph, we will do exactly that, to use this _agentic_ term a bit loosy, with the goal of cognitive resonance and clarify its practical implications on the task of creating an event-driven process. 

We take the following foundational terms from [1]:

- Environment: A collection of entities.
- Entity: A collection of attributes.
- Object: An entity that can change an environment through its performed actions.
- Agent: An object with a set of {perceptions, actions, states, goals}

In the context of Atelier: 

- Environment: A financial market as the collection of the Orderbooks and PublicTrades
- Entity: The Orderbook structure, the Publictrades structure.
- Object: An order (active/cancelled), a trade (a matched tupple of orders).
- Perceptions: To get information about an entity, an object, an agent's state.
- Actions: The interaction with an object that exist within the environment's context.
- State: A set with the values of some of the environment's entities, or, the agent's state.
- Goal: A state to be achived after performing actions within the environment.

In the context of Reinforcement Learning [2]:

An agent: 

1. Can perceive the state of the environment
2. Can take actions that alter the state of the environment
3. Can use a value function to decide what actions to taken
4. Can use a model of the environment in order to pre-evaluate any given set of action or inactions

5. Has a policy of behavior
	1. A mapping from perceived states of the environment to actions to be taken when in those states
6. Receives a reward signal
	1. produced by the environment as a consequence of the agent's action.
7. Uses its value function
	1. To estimate the posible value generated from the expected stream of future rewards, given a sequence of actions and observations an agent makes.
8. Uses a model of the environment
	1. To make inferences about how the environment could change, produce a reward, from an action or inaction of the agent.



## Agent types:

In general, we propose these two categories of agents. 

- **stateless agent**: Select actions only with the basis of the current percept.

In general, stateless is closer to _reflexive_ and not necessarily random, or simplistic. For instance, an Agent that is not taking historical registries can still execute computations to make decisions, that would alter its internal state, and doing this by using the external state of the environment.

- **stateful agent**: Select actions given the captures and stores information within internal state. These actions could be interocepted, i.e. about the agent's own internal state, or, exterocepted, i.e. about the external environment's state.

A Stateful agent is more apealling, mainly because storing information enables to do other complex operations, like having _memory_ to compute and record the consequences of its actions, which later could be used in a _goal_ setting step, to have a sort of _self-correcting_ behavior.

## Agent's Implementations

These are the methods within the `agents/` module. Feel free to take the agents definition and implement your own functionality on top of it. 

###  Perceptions:

endogenous
- get_internals: Get its own description.

exogenous
- get_orderbook: Get the current LOB.
- get_orders: Get its current active orders (limit).
- get_historical_orders: Get registries of private orders (opened, filled, modified, cancelled).
- get_historical_orderbooks: Get registries of orderbooks.
- get_historical_publictrades: Get registries of public trades (filled orders).

### Actions:

exogenous
- send_new_order: Send a request to open an order (limit or market).
- send_modify_order: Send the request to modify an active order (limit).
- send_delete_order: Send the request to cancel an active order (limit).

endogenous
- gen_event: Generate an event (new MO, new LO, modify LO, cancel LO).
- compute_position: From one or a set of orders, compute the present economic value.
- compute_balance: From orderbooks and orders, compute the present or historical economic value.

### States:

- position_value: Computes the economic value of a given order, using a provided market.
- account_balance: The algebraic sum of the values of all the selected orders (current and/or past).

### Goals:

- maximize_metric: Compute a metric to be used as a goal. 

