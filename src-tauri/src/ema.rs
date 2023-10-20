use serde::{Deserialize, Serialize};
use tauri::State;

use crate::model::{IndicatorState, OHLC};

#[derive(Serialize, Deserialize)]
pub struct Ema {
    pub period: usize,
    pub multiplier: f64,
    pub current: Option<f64>,
    count: usize,
    initial_sum: f64,
}

impl Ema {
    pub fn new(period: usize) -> Self {
        Ema {
            period,
            multiplier: 2.0 / (period as f64 + 1.0),
            current: None,
            count: 0,
            initial_sum: 0.0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct EmaHistory {
    pub time: i64,
    pub value: f64,
}

#[tauri::command]
pub fn ema_history(indicator_state: State<IndicatorState>, ohlcs: Vec<OHLC>) -> Vec<EmaHistory> {
    let mut ema_indicator = indicator_state.ema.lock().unwrap();
    let mut count = 0;
    let mut inital_sum = 0.0_f64;
    let mut current = 0.0_f64;
    let mut values = Vec::new();

    for price in ohlcs.iter() {
        if count < ema_indicator.period {
            inital_sum += price.close;
            count += 1;

            if count == ema_indicator.period {
                current = inital_sum / ema_indicator.period as f64;

                values.push(EmaHistory {
                    time: price.time,
                    value: current,
                });
            }
        } else {
            let new_ema =
                price.close * ema_indicator.multiplier + current * (1.0 - ema_indicator.multiplier);
            let rounded_ema = (new_ema * 100_000.0).round() / 100_000.0;
            values.push(EmaHistory {
                time: price.time,
                value: rounded_ema,
            });
            current = new_ema;
            ema_indicator.current = Some(new_ema);
        }
    }

    values
}

// Helper function for MACD.
pub fn compute_ema(price: f64, ema_indicator: &mut Ema) -> f64 {
    if ema_indicator.count < ema_indicator.period {
        ema_indicator.initial_sum += price;
        ema_indicator.count += 1;

        if ema_indicator.count == ema_indicator.period {
            let current = ema_indicator.initial_sum / ema_indicator.period as f64;
            ema_indicator.current = Some(current);
            current
        } else {
            0.0 // If the count is still not equal to period, the EMA is not ready yet.
        }
    } else {
        let current = ema_indicator
            .current
            .expect("EMA should have a current value by this point.");
        let new_ema = price * ema_indicator.multiplier + current * (1.0 - ema_indicator.multiplier);
        let result = (new_ema * 100_000.0).round() / 100_000.0;
        ema_indicator.current = Some(result);
        result
    }
}
