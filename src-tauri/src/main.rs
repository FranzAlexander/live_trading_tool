// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use ema::Ema;

use api::kraken::get_market_trades;
use model::{DeltaCandle, RangeBar};

// use macd::Macd;
// use model::{AppState, IndicatorState, OHLC};
use crate::model::{AppState, RangeData};
use reqwest::Client;
// use crate::{market::get_asset_info, user::get_symbols};
use tokio::sync::Mutex;
// mod api;
// mod ema;
// mod indicator;
// mod macd;
// mod market;
// // mod model;
// mod user;
mod api;
mod model;
mod range;
mod rolling_window;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

// #[derive(Clone, serde::Serialize)]
// struct OhlcPayload {
//     message: Vec<OHLC>,
// }

fn main() {
    dotenv::dotenv().ok();

    tauri::Builder::default()
        .manage(AppState {
            client: Client::new(),
            api_key: std::env::var("API_KEY").unwrap(),
            secret_key: std::env::var("SECRET_KEY").unwrap(),
            range_data: Mutex::new(RangeData::new(2.0)),
        })
        .setup(|app| {
            // `main` here is the window label; it is defined on the window creation or under `tauri.conf.json`
            // the default value is `main`. note that it must be unique
            // let main_window = app.get_window("main").unwrap();

            // tauri::async_runtime::spawn(test_market_data(main_window));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![app_start])
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn app_start(
    app: tauri::State<'_, AppState>,
) -> Result<(Vec<RangeBar>, Vec<DeltaCandle>), String> {
    let trades = get_market_trades(&app.client).await;
    for trade in trades.iter() {
        app.range_data
            .lock()
            .await
            .update(trade.price, trade.volume, &trade.buy_sell);

        println!("{:?}", trade);
    }

    println!("{:?}", app.range_data.lock().await.range_bars);

    let range_data = app.range_data.lock().await;

    Ok((
        range_data.range_bars.ordered_elements(),
        range_data.delta_bars.ordered_elements(),
    ))
}
