use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use super::string_or_float;

#[derive(Debug, Deserialize, Serialize)]
pub struct MarketTrades {
    pub trades: Vec<Trade>,
    pub best_bid:String,
    pub best_ask:String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Trade {
    pub trade_id: String,
    pub product_id: String,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub size: f64,
    pub time: DateTime<Utc>,
    pub side: String,
    pub bid: String,
    pub ask: String,
}
