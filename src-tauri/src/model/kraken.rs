use super::string_or_float;

use std::collections::HashMap;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum KrakenEvent {
    Connection(ConnectionMessage),
    Subscription(SubscribeMessage),
    Heartbeat(HeartbeatMessage),
    MarketOrder(Vec<MarketTradeMessage>)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ConnectionMessage {
    #[serde(rename = "connectionID")]
    pub connection_id: i64,
    pub event: String,
    pub status: String,
    pub version: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SubscribeMessage {
    #[serde(rename = "channelID")]
    pub channel_id: i32,
    #[serde(rename = "channelName")]
    pub channel_name: String,
    pub event: String,
    pub pair: String,
    pub status: String,
    pub subscription: SubscriptionMessage,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionMessage {
    pub name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct HeartbeatMessage {
    pub event: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MarketTradeMessage {
    pub id: i64,
    pub trades: Vec<MarketOrderMessage>,
    pub name: String,
    pub pair: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MarketOrderMessage {
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub volume: f64,
    #[serde(with = "string_or_float")]
    pub time: f64,
    pub side: String,
    pub order_type: String,
    pub misc: String,
}

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
