use std::collections::HashMap;

use crate::model::{ApiResponse, ApiResponseMap, AppState, AssetInfo, OHLCResponse, OHLC};

#[tauri::command]
pub async fn get_asset_info(state: tauri::State<'_, AppState>) -> Result<Vec<String>, String> {
    let assets = state
        .client
        .get("http://127.0.0.1:3080/asset_info")
        .send()
        .await
        .unwrap()
        .json::<ApiResponseMap<AssetInfo>>()
        .await
        .unwrap();

    Ok(assets.result.keys().map(|k| k.to_owned()).collect())
}

// #[tauri::command]
// pub fn get_ohlc_history() -> Vec<OHLC> {
//     let mut url_value = HashMap::new();
//     url_value.insert("pair".to_string(), "XRPUSD".to_string());
//     url_value.insert("interval".to_string(), "5".to_string());

//     let ohlc_response =
//         get_market_data::<ApiResponse<OHLCResponse>>(KRAKEN_OHLC_ENDPOINT.to_string());

//     let mut coin_key = "";

//     for key in ohlc_response.result.tickers.keys() {
//         coin_key = key;
//     }

//     ohlc_response
//         .result
//         .tickers
//         .get(coin_key)
//         .unwrap()
//         .to_owned()
// }
