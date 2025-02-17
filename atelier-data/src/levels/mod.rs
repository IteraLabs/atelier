use crate::orders::{Order, OrderSide};

// ------------------------------------------------------------------------------------ LEVEL -- //
// ------------------------------------------------------------------------------------ ----- -- //

/// Represents a price level in an order book.
///
/// The `Level` struct contains details about a specific price level, including
/// its unique identifier, side (buy/sell), price, total volume at that price,
/// and a vector of orders associated with that level.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Level {
    pub level_id: u32,
    pub side: OrderSide,
    pub price: f64,
    pub volume: f64,
    pub orders: Vec<Order>,
}

impl Level {
    // ---------------------------------------------------------------------------- New Level -- //
    // ---------------------------------------------------------------------------- --------- -- //

    /// Creates a new instance of `Level`.
    ///
    /// # Parameters
    ///
    /// - `level_id`: The unique identifier for the price level.
    /// - `side`: The side of the order book, either `Side::Bids` or `Side::Asks`.
    /// - `price`: The price at which orders are placed at this level.
    /// - `volume`: The total volume of orders at this price level.
    /// - `orders`: A vector of `Order` representing the orders at this level.
    ///
    /// # Returns
    ///
    /// Returns a new `Level` instance with the specified parameters.

    pub fn new(level_id: u32, side: OrderSide, price: f64, volume: f64, orders: Vec<Order>) -> Self {
        match side {
            OrderSide::Bids => Level {
                level_id,
                side: OrderSide::Bids,
                price,
                volume,
                orders: orders.clone(),
            },
            OrderSide::Asks => Level {
                level_id,
                side: OrderSide::Asks,
                price,
                volume,
                orders: orders.clone(),
            },
        };

        Level {
            level_id,
            side,
            price,
            volume,
            orders,
        }
    }
}

