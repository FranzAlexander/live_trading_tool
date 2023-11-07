use super::string_or_float;

use std::collections::HashMap;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum KrakenEvent {
    Connection(ConnectionMessage),
    Subscription(SubscribeMessage),
    Heartbeat(HeartbeatMessage),
    MarketOrder(MarketTradeMessage),
    OhlcEvent(OhlcMessage),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
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
pub struct OhlcMessage {
    pub id: i64,
    pub ohlc: OhlcCandleMessage,
    pub name: String,
    pub pair: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OhlcCandleMessage {
    #[serde(with = "string_or_float")]
    pub time: f64,
    #[serde(with = "string_or_float")]
    pub etime: f64,
    #[serde(with = "string_or_float")]
    pub open: f64,
    #[serde(with = "string_or_float")]
    pub high: f64,
    #[serde(with = "string_or_float")]
    pub low: f64,
    #[serde(with = "string_or_float")]
    pub close: f64,
    #[serde(with = "string_or_float")]
    pub vwap: f64,
    #[serde(with = "string_or_float")]
    pub volume: f64,
    pub count: i64,
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

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy)]
pub struct ExtendedBalance {
    #[serde(with = "string_or_float")]
    pub balance: f64,
    #[serde(with = "string_or_float")]
    pub hold_trade: f64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct PairInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    altname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wsname: Option<String>,
    aclass_base: String,
    base: String,
    aclass_quote: String,
    quote: String,
    lot: String,
    pair_decimals: i32,
    cost_decimals: i32,
    lot_decimals: i32,
    lot_multiplier: i32,
    leverage_buy: Vec<i32>,
    leverage_sell: Vec<i32>,
    fees: Vec<Fees>,
    fees_maker: Vec<Fees>,
    fee_volume_currency: String,
    margin_call: i32,
    margin_stop: i32,
    #[serde(with = "string_or_float")]
    ordermin: f64,
    #[serde(with = "string_or_float")]
    costmin: f64,
    #[serde(with = "string_or_float")]
    tick_size: f64,
    status: String,
    long_position_limit: i32,
    short_position_limit: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Fees {
    pub volume: i32,
    #[serde(with = "string_or_float")]
    pub fee: f64,
}
