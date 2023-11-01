use super::string_or_float;

use std::collections::HashMap;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiResponse<T> {
    pub error: Vec<String>,
    pub result: T,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiResponseMap<T> {
    pub error: Vec<String>,
    pub result: HashMap<String, T>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct MarketTradeResponse {
    #[serde(flatten)]
    pub trades: HashMap<String, Vec<MarketTrade>>,
    pub last: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct MarketTrade {
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub volume: f64,
    #[serde(with = "string_or_float")]
    pub time: f64,
    pub buy_sell: String,
    pub market_limit: String,
    pub misc: String,
    pub trade_id: i64,
}
