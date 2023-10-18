use std::collections::HashMap;

use crate::{
    api::{get_market_data, KRAKEN_OHLC_ENDPOINT},
    model::{ApiResponse, OHLCResponse, OHLC},
};

#[tauri::command]
pub fn get_ohlc_history() -> Vec<OHLC> {
    let mut url_value = HashMap::new();
    url_value.insert("pair".to_string(), "XRPUSD".to_string());
    url_value.insert("interval".to_string(), "5".to_string());

    let ohlc_response =
        get_market_data::<ApiResponse<OHLCResponse>>(KRAKEN_OHLC_ENDPOINT.to_string());

    let mut coin_key = "";

    for key in ohlc_response.result.tickers.keys() {
        coin_key = key;
    }

    ohlc_response
        .result
        .tickers
        .get(coin_key)
        .unwrap()
        .to_owned()
}
