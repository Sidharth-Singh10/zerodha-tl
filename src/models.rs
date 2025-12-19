use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    LTP,
    Quote,
    Full,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::LTP
    }
}

#[derive(Debug, Default, Clone)]
pub struct Depth {
    pub quantity: i32,
    pub price: f64,
    pub orders: u16,
}

#[derive(Debug, Default, Clone)]
pub struct Tick {
    pub instrument_token: u32,
    pub mode: Mode,
    pub ltp: f64,

    // Available in Quote (44 bytes) and Full (184 bytes)
    pub last_traded_quantity: Option<i32>,
    pub average_traded_price: Option<f64>,
    pub volume: Option<i32>,
    pub total_buy_quantity: Option<i32>,
    pub total_sell_quantity: Option<i32>,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,

    // Available only in Full (184 bytes)
    pub last_traded_timestamp: Option<i32>,
    pub open_interest: Option<i32>,
    pub open_interest_day_high: Option<i32>,
    pub open_interest_day_low: Option<i32>,
    pub exchange_timestamp: Option<i32>,

    // Market Depth (5 bids, 5 offers)
    pub bids: Option<Vec<Depth>>,
    pub offers: Option<Vec<Depth>>,
}
