use serde::Serialize;

use super::kraken::OhlcMessage;

#[derive(Serialize, Clone)]
pub struct OhlcPayload {
    pub name: String,
    pub time: i64,
    pub etime: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub vwap: f64,
    pub volume: f64,
}

impl From<OhlcMessage> for OhlcPayload {
    fn from(value: OhlcMessage) -> Self {
        let round_num: i64 = if value.name == "ohlc-1" { 60 } else { 300 };
        OhlcPayload {
            name: value.name,
            time: (value.ohlc.etime) as i64 - round_num,
            etime: (value.ohlc.etime) as i64,
            open: value.ohlc.open,
            high: value.ohlc.high,
            low: value.ohlc.low,
            close: value.ohlc.close,
            vwap: value.ohlc.vwap,
            volume: value.ohlc.volume,
        }
    }
}
