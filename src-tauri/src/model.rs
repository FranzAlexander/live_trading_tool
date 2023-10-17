use std::collections::HashMap;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MarketDataResponse<T> {
    pub error: Vec<String>,
    pub result: T,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OHLCResponse {
    #[serde(flatten)]
    pub tickers: HashMap<String, Vec<OHLC>>,
    pub last: i64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy)]
pub struct OHLC {
    pub time: i64,
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
    pub count: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct AccountBalance {
    pub balances: HashMap<String, Balance>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy)]
pub struct Balance {
    #[serde(with = "string_or_float")]
    pub balance: f64,
    #[serde(with = "string_or_float_opt")]
    pub credit: Option<f64>,
    #[serde(with = "string_or_float_opt")]
    pub credit_used: Option<f64>,
    #[serde(with = "string_or_float")]
    pub hold_trade: f64,
}

pub(crate) mod string_or_float {
    use std::fmt;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Float(f64),
        }

        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => {
                if s == "INF" {
                    Ok(f64::INFINITY)
                } else {
                    s.parse().map_err(de::Error::custom)
                }
            }
            StringOrFloat::Float(i) => Ok(i),
        }
    }
}

pub(crate) mod string_or_float_opt {
    use std::fmt;

    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        match value {
            Some(v) => crate::model::string_or_float::serialize(v, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Float(f64),
        }

        Ok(Some(crate::model::string_or_float::deserialize(
            deserializer,
        )?))
    }
}
