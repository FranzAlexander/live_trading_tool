pub mod coinbase;
pub mod kraken;

use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use tokio::sync::Mutex;

use crate::rolling_window::RollingWindow;

pub struct AppState {
    pub client: reqwest::Client,
    pub api_key: String,
    pub secret_key: String,
    pub range_data: Mutex<RangeData>,
}

#[derive(Debug)]
pub struct RangeData {
    pub range: f64,
    pub range_bars: VecDeque<RangeBar>,
    pub delta_bars: VecDeque<DeltaCandle>,
    pub count: usize,
}

impl RangeData {
    pub fn new(range: f64) -> Self {
        RangeData {
            range,
            range_bars: VecDeque::with_capacity(400),
            delta_bars: VecDeque::with_capacity(400),
            count: 0,
        }
    }

    pub fn update(&mut self, price: f64, size: f64, side: &str) {
        let delta = match side {
            "b" => size,
            "s" => -size,
            _ => 0.0,
        };

        if self.range_bars.back().is_none() {
            self.range_bars.push_back(RangeBar::new());
        }

        if let Some(current_bar) = self.range_bars.back_mut() {}

        // if self.range_bars.current_value_mut().is_none() {
        //     self.range_bars.push(RangeBar::new());
        // }

        // if let Some(current_bar) = self.range_bars.current_value_mut() {
        //     let (updated_bar, is_complete) = current_bar.update(price, self.range, self.count);

        //     *current_bar = updated_bar;

        //     if is_complete {
        //         self.count = 0;
        //         self.range_bars.push(RangeBar::new());
        //     } else {
        //         self.count += 1;
        //     }
        // }

        // // Updating delta_bars
        // if self.delta_bars.current_value_mut().is_none() {
        //     self.delta_bars.push(DeltaCandle::new());
        // }

        // if let Some(current_delta_candle) = self.delta_bars.current_value_mut() {
        //     let (updated_delta_candle, delta_complete) =
        //         current_delta_candle.update(delta, self.range);

        //     *current_delta_candle = updated_delta_candle;

        //     if delta_complete {
        //         self.delta_bars.push(DeltaCandle::new());
        //     }
        // }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaCandle {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub cumulative_delta: f64,
}

impl DeltaCandle {
    pub fn new() -> Self {
        DeltaCandle {
            open: 0.0,
            high: 0.0,
            low: 0.0,
            close: 0.0,
            cumulative_delta: 0.0,
        }
    }

    pub fn update(&mut self, delta: f64, range: f64) -> (DeltaCandle, bool) {
        if self.cumulative_delta == 0.0 {
            self.open = delta;
            self.high = delta;
            self.low = delta;
            self.close = delta;
            self.cumulative_delta = delta;
        } else {
            self.cumulative_delta += delta;
            self.close = self.cumulative_delta;
            if self.cumulative_delta > self.high {
                self.high = self.cumulative_delta;
            }
            if self.cumulative_delta < self.low {
                self.low = self.cumulative_delta;
            }
        }

        let is_complete = self.cumulative_delta.abs() >= range;
        if is_complete {
            self.reset();
        }

        (self.clone(), is_complete)
    }

    pub fn reset(&mut self) {
        self.open = 0.0;
        self.high = 0.0;
        self.low = 0.0;
        self.close = 0.0;
        self.cumulative_delta = 0.0;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeBar {
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
}

impl RangeBar {
    pub fn new() -> Self {
        RangeBar {
            close: 0.0,
            high: 0.0,
            low: 0.0,
            open: 0.0,
        }
    }

    pub fn update(&mut self, price: f64, range: f64, count: usize) -> (RangeBar, bool) {
        if count == 0 {
            self.open = price;
            self.high = price;
            self.low = price;
            self.close = price;
        } else {
            if price > self.high {
                self.high = price;
            }
            if price < self.low {
                self.low = price;
            }
            self.close = price;
        }

        let is_complete = self.is_complete(range);
        if is_complete {
            self.reset();
        }

        (self.clone(), is_complete)
    }

    pub fn is_complete(&self, range: f64) -> bool {
        (self.high - self.low) >= range
    }

    pub fn reset(&mut self) {
        self.close = 0.0;
        self.high = 0.0;
        self.low = 0.0;
        self.open = 0.0;
    }
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
