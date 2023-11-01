pub mod coinbase;
pub mod kraken;

pub struct AppState {
    pub client: reqwest::Client,
    pub api_key: String,
    pub secret_key: String,
}

pub struct RangeBar{
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub range: usize,
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

pub(crate) mod string_or_i64 {
    use serde::{de, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrI64 {
            String(String),
            Int(i64),
        }

        match StringOrI64::deserialize(deserializer)? {
            StringOrI64::String(s) => s.parse().map_err(de::Error::custom),
            StringOrI64::Int(i) => Ok(i),
        }
    }
}
