// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

// use ema::Ema;

use api::coinbase::get_market_trades;
use model::AppState;
// use macd::Macd;
// use model::{AppState, IndicatorState, OHLC};
use reqwest::Client;

// use crate::{market::get_asset_info, user::get_symbols};

// mod api;
// mod ema;
// mod indicator;
// mod macd;
// mod market;
// // mod model;
// mod user;
mod api;
mod model;

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
            api_key: std::env::var("COINBASE_API_KEY").unwrap(),
            secret_key: std::env::var("COINBASE_API_SECRET").unwrap(),
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
async fn app_start(app:tauri::State<'_, AppState>)->Result<String, String>{
    get_market_trades(&app.client, app.secret_key.as_bytes(), &app.api_key).await;
    Ok("s".to_string())
}