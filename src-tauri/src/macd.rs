use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{
    ema::{compute_ema, Ema},
    model::{IndicatorState, OHLC},
};

#[derive(Serialize, Deserialize)]
pub struct Macd {
    fast_ema: Ema,
    slow_ema: Ema,
    signal_ema: Ema,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MacdHistory {
    pub time: i64,
    pub macd: f64,
    pub signal: f64,
    pub histogram: f64,
}

impl Macd {
    pub fn new(fast_period: usize, slow_period: usize, signal_period: usize) -> Self {
        Macd {
            fast_ema: Ema::new(fast_period),
            slow_ema: Ema::new(slow_period),
            signal_ema: Ema::new(signal_period),
        }
    }
}

#[tauri::command]
pub fn macd_history(indicator_state: State<IndicatorState>, ohlcs: Vec<OHLC>) -> Vec<MacdHistory> {
    let mut macd_state = indicator_state.macd.lock().unwrap();

    let mut history = Vec::new();

    for price in ohlcs.iter() {
        let fast_ema = compute_ema(price.close, &mut macd_state.fast_ema);
        let slow_ema = compute_ema(price.close, &mut macd_state.slow_ema);

        let macd_value = fast_ema - slow_ema;

        let signal = compute_ema(macd_value, &mut macd_state.signal_ema);

        let histogram = macd_value - signal;

        history.push(MacdHistory {
            time: price.time,
            macd: macd_value,
            signal,
            histogram,
        });
    }

    history
}
