use std::time::{SystemTime, UNIX_EPOCH};

/// TradeSide
///
/// Enum for identification of either a buy or sell side
/// used to describe the Trade side.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum TradeSide {
    Buy,
    Sell,
}

/// TradeType
///
/// Enum for identification of the supported Trade Types
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum TradeType {
    Dex,
    Cex,
}

/// Represents a single trade.
///
/// The `Trade` struct contains details about an individual trade, including
/// its unique identifier, timestamp, type, side (Buy/Sell), price, and amount.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Trade {
    pub trade_id: u64,
    pub trade_ts: u64,
    pub trade_type: TradeType,
    pub side: TradeSide,
    pub price: Option<f64>,
    pub amount: Option<f64>,
}

impl Trade {
    pub fn builder() -> TradeBuilder {
        TradeBuilder::new()
    }
    
    /// Encoded Trade ID formation
    ///
    /// The trade_id field is an u64 containing encoded info about: side, type,
    /// timestamp. The Bit allocation is the following:
    ///
    /// 00TTSSSS SSSSSSSS SSSSSSSS SSSSSSSS SSSSSSSS SSSSSSSS SSSSSSSS SSSSSSSS
    /// ||└ 60 bits for timestamp (valid until ~2079)
    /// |└─ 1 bit for type (0=Cex, 1=Dex)
    /// └── 1 bit for side (0=Buy, 1=Sell)
    pub fn encode_trade_id(trade_side: TradeSide, trade_type: TradeType, trade_ts: u64) -> u64 {
        // Highest bit
        let side_bit = match trade_side {
            TradeSide::Buy => 0,
            TradeSide::Sell => 1,
        } << 63;
        // Second highest bit
        let type_bit = match trade_type {
            TradeType::Dex => 0,
            TradeType::Cex => 1,
        } << 62;
        // 60 bits starting at position 2
        let timestamp_bits = (trade_ts & ((1 << 60) - 1)) << 2;

        side_bit | type_bit | timestamp_bits
    }

    /// Decode Trade ID encoded formation. check `encode_trade_id` for more
    /// details.
    pub fn decode_trade_id(trade_id: u64) -> (TradeSide, TradeType, u64) {
        // Highest bit
        let trade_side = if (trade_id >> 63) & 1 == 0 {
            TradeSide::Buy
        } else {
            TradeSide::Sell
        };
        // Second highest bit
        let trade_type = if (trade_id >> 62) & 1 == 0 {
            TradeType::Cex
        } else {
            TradeType::Dex
        };
        // 60 bits starting at position 2
        let trade_ts = (trade_id >> 2) & ((1 << 60) - 1);

        (trade_side, trade_type, trade_ts)
    }
}

// ------------------------------------------------------------------------------------ TRADE -- //
// ------------------------------------------------------------------------------------ -------- //

#[derive(Debug, Copy, Clone)]
pub struct TradeBuilder {
    trade_ts: Option<u64>,
    trade_type: Option<TradeType>,
    side: Option<TradeSide>,
    price: Option<f64>,
    amount: Option<f64>,
}

impl TradeBuilder {
    pub fn new() -> Self {
        TradeBuilder {
            side: None,
            trade_type: None,
            trade_ts: None,
            price: None,
            amount: None,
        }
    }

    pub fn side(mut self, side: TradeSide) -> Self {
        self.side = Some(side);
        self
    }

    pub fn trade_type(mut self, trade_type: TradeType) -> Self {
        self.trade_type = Some(trade_type);
        self
    }

    pub fn trade_ts(mut self, trade_ts: u64) -> Self {
        self.trade_ts = Some(trade_ts);
        self
    }

    pub fn price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }

    pub fn amount(mut self, amount: f64) -> Self {
        self.amount = Some(amount);
        self
    }

    ///
    /// Builder pattern to create a new instance of an `Trade`.
    /// it requires to have the fields defined in the following order
    ///
    /// side: TradeSide
    /// trade_type: TradeType
    /// trade_ts: u64
    ///
    /// then it forms the `trade_id` by calling the `encode_trade_id`
    /// which will be a u64 formed with side, trade_type, trade_ts.
    pub fn build(self) -> Result<Trade, &'static str> {
        let trade_side = self.side.ok_or("Missing side")?;
        let trade_type = self.trade_type.ok_or("Missing trade_type")?;
        let trade_ts = self.trade_ts.unwrap_or_else(|| {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_micros() as u64
        });

        let trade_id = Trade::encode_trade_id(trade_side, trade_type, trade_ts);

        Ok(Trade {
            trade_id,
            trade_ts,
            trade_type,
            side: trade_side,
            price: self.price,
            amount: self.amount,
        })
    }
}

