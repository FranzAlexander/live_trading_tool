// use std::{collections::HashMap, sync::Mutex};

// use crate::{macd::Macd, Ema};

// pub struct AppState {
//     pub client: reqwest::Client,
//     pub api_key: String,
//     pub secret_key: String,
// }

// pub struct IndicatorState {
//     pub ema: Mutex<Ema>,
//     pub macd: Mutex<Macd>,
// }

// #[derive(Debug, serde::Serialize, serde::Deserialize)]
// pub struct ApiResponse<T> {
//     pub error: Vec<String>,
//     pub result: T,
// }

// #[derive(Debug, serde::Serialize, serde::Deserialize)]
// pub struct ApiResponseMap<T> {
//     pub error: Vec<String>,
//     pub result: HashMap<String, T>,
// }

// #[derive(Debug, serde::Serialize, serde::Deserialize)]
// pub struct OHLCResponse {
//     #[serde(flatten)]
//     pub tickers: HashMap<String, Vec<OHLC>>,
//     pub last: i64,
// }

// #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy)]
// pub struct OHLC {
//     pub time: i64,
//     #[serde(with = "string_or_float")]
//     pub open: f64,
//     #[serde(with = "string_or_float")]
//     pub high: f64,
//     #[serde(with = "string_or_float")]
//     pub low: f64,
//     #[serde(with = "string_or_float")]
//     pub close: f64,
//     #[serde(with = "string_or_float")]
//     pub vwap: f64,
//     #[serde(with = "string_or_float")]
//     pub volume: f64,
//     pub count: i32,
// }

// #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy)]
// pub struct ExtendedBalance {
//     #[serde(with = "string_or_float")]
//     pub balance: f64,
//     #[serde(with = "string_or_float")]
//     pub hold_trade: f64,
// }

// #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy)]
// pub struct TradeBalance {
//     #[serde(with = "string_or_float_opt")]
//     #[serde(rename = "eb")]
//     pub equivalent: Option<f64>,

//     #[serde(with = "string_or_float_opt")]
//     #[serde(rename = "tb")]
//     pub trade: Option<f64>,

//     #[serde(with = "string_or_float_opt")]
//     #[serde(rename = "m")]
//     pub margin: Option<f64>,

//     #[serde(with = "string_or_float_opt")]
//     #[serde(rename = "n")]
//     pub unrealized_pl: Option<f64>,

//     #[serde(with = "string_or_float_opt")]
//     #[serde(rename = "c")]
//     pub cost: Option<f64>,

//     #[serde(with = "string_or_float_opt")]
//     #[serde(rename = "v")]
//     pub valuation: Option<f64>,

//     #[serde(with = "string_or_float_opt")]
//     #[serde(rename = "e")]
//     pub equity: Option<f64>,

//     #[serde(with = "string_or_float_opt")]
//     #[serde(rename = "mf")]
//     pub free_magin: Option<f64>,

//     #[serde(default, with = "string_or_float_opt", rename = "ml")]
//     pub margin_level: Option<f64>,

//     #[serde(with = "string_or_float_opt")]
//     #[serde(rename = "uv")]
//     pub unexecuted_value: Option<f64>,
// }

// #[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
// pub struct AssetInfo {
//     pub aclass: String,
//     pub altname: String,
//     pub decimals: i32,
//     pub display_decimals: i32,
//     #[serde(default)]
//     pub collateral_value: Option<f64>,
//     pub status: String,
// }

// #[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
// pub struct PairInfo {
//     altname: String,
//     wsname: String,
//     aclass_base: String,
//     base: String,
//     aclass_quote: String,
//     quote: String,
//     lot: String,
//     pair_decimals: i32,
//     cost_decimals: i32,
//     lot_decimals: i32,
//     lot_multiplier: i32,
//     leverage_buy: Vec<i32>,
//     leverage_sell: Vec<i32>,
//     fees: Vec<(String, String)>,
//     fees_maker: Vec<(String, String)>,
//     fee_volume_currency: String,
//     margin_call: i32,
//     margin_stop: i32,
//     ordermin: String,
//     costmin: String,
//     tick_size: String,
//     status: String,
//     long_position_limit: i32,
//     short_position_limit: i32,
// }

// pub(crate) mod string_or_float {
//     use std::fmt;

//     use serde::{de, Deserialize, Deserializer, Serializer};

//     pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         T: fmt::Display,
//         S: Serializer,
//     {
//         serializer.collect_str(value)
//     }

//     pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         #[derive(Deserialize)]
//         #[serde(untagged)]
//         enum StringOrFloat {
//             String(String),
//             Float(f64),
//         }

//         match StringOrFloat::deserialize(deserializer)? {
//             StringOrFloat::String(s) => {
//                 if s == "INF" {
//                     Ok(f64::INFINITY)
//                 } else {
//                     s.parse().map_err(de::Error::custom)
//                 }
//             }
//             StringOrFloat::Float(i) => Ok(i),
//         }
//     }
// }

// pub(crate) mod string_or_float_opt {
//     use std::fmt;

//     use serde::{Deserialize, Deserializer, Serializer};

//     pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         T: fmt::Display,
//         S: Serializer,
//     {
//         match value {
//             Some(v) => crate::model::string_or_float::serialize(v, serializer),
//             None => serializer.serialize_none(),
//         }
//     }

//     pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         #[derive(Deserialize)]
//         #[serde(untagged)]
//         enum StringOrFloatOpt {
//             String(String),
//             Float(f64),
//             None,
//         }

//         match StringOrFloatOpt::deserialize(deserializer) {
//             Ok(StringOrFloatOpt::String(s)) => {
//                 if s == "INF" {
//                     Ok(Some(f64::INFINITY))
//                 } else {
//                     s.parse().map(Some).map_err(serde::de::Error::custom)
//                 }
//             }
//             Ok(StringOrFloatOpt::Float(i)) => Ok(Some(i)),
//             Ok(StringOrFloatOpt::None) | Err(_) => Ok(None), // Handle both the explicit None variant and errors as None
//         }
//     }
// }
