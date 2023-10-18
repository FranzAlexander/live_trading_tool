use serde::{Deserialize, Serialize};

use crate::model::OHLC;

#[derive(Serialize, Deserialize)]
pub struct Ema {
    period: usize,
    multiplier: f64,
    current: Option<f64>,
    count: usize,
}

#[tauri::command]
pub fn create_ema(period: usize) -> Ema {
    Ema {
        period,
        multiplier: 2.0 / (period as f64 + 1.0),
        current: None,
        count: 0,
    }
}

#[derive(Serialize, Deserialize)]
pub struct EmaHistory {
    pub time: i64,
    pub value: f64,
}

#[tauri::command]
pub fn ema_history(period: usize, multi: f64, ohlcs: Vec<OHLC>) -> Vec<EmaHistory> {
    let mut count = 0;
    let mut inital_sum = 0.0_f64;
    let mut current = 0.0_f64;
    let mut values = Vec::new();

    for price in ohlcs.iter() {
        if count < period {
            inital_sum += price.close;
            count += 1;

            if count == period {
                current = inital_sum / period as f64;

                values.push(EmaHistory {
                    time: price.time,
                    value: current,
                });
            }
        } else {
            let new_ema = price.close * multi + current * (1.0 - multi);
            let rounded_ema = (new_ema * 100_000.0).round() / 100_000.0;
            values.push(EmaHistory {
                time: price.time,
                value: rounded_ema,
            });
            current = new_ema;
        }
    }

    values
}
