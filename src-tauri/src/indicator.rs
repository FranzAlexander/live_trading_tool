// use crate::model::{AppState, OHLC};

// #[tauri::command]
// pub fn get_ema_history(state: &mut tauri::State<'_, AppState>, ohlc: Vec<OHLC>) -> Vec<f64> {
//     let mut values = Vec::new();

//     for prices in ohlc.iter() {
//         state.ema.update(prices.close);
//     }

//     values
// }
