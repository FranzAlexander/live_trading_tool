// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use ema::Ema;

use std::{collections::VecDeque, sync::Arc};

use api::kraken::{get_market_trades, KRAKEN_WS_URL};
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

// use macd::Macd;
// use model::{AppState, IndicatorState, OHLC};
use crate::model::AppState;
use reqwest::Client;
// use crate::{market::get_asset_info, user::get_symbols};
use tokio::sync::Mutex;

use range::{
    delta_bar::{self, DeltaBar},
    range_bar::RangeBar,
    Bar, RangeData,
};
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
            range_data: Arc::new(Mutex::new(RangeData::new(0.02))),
        })
        .setup(|app| {
            // `main` here is the window label; it is defined on the window creation or under `tauri.conf.json`
            // the default value is `main`. note that it must be unique
            // let main_window = app.get_window("main").unwrap();

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
) -> Result<(Vec<RangeBar>, VecDeque<DeltaBar>), String> {
    let trades = get_market_trades(&app.client).await;
    for trade in trades.iter() {
        println!("Trade:{:?}", trade);
        app.range_data
            .lock()
            .await
            .update(trade.price, trade.volume, &trade.buy_sell, trade.time);
    }

    let (range_bars, delta_bars) = app.range_data.lock().await.get_bars();

    let mut sorted_range_bars: Vec<RangeBar> = range_bars.into_iter().collect();

    let ws_range_data = app.range_data.clone();

    sorted_range_bars.sort_by_key(|item| item.start_time);

    tauri::async_runtime::spawn(kraken_websocket(ws_range_data));

    Ok((sorted_range_bars, delta_bars))
}

async fn kraken_websocket(range_bars: Arc<Mutex<RangeData>>) {
    let (mut ws_stream, _) = connect_async(KRAKEN_WS_URL).await.unwrap();

    let subscription_msg = serde_json::json!({
    "event": "subscribe",
    "pair": ["SOL/USD"],
    "subscription": {
        "name":"trade"
    }});

    ws_stream
        .send(tokio_tungstenite::tungstenite::Message::Text(
            subscription_msg.to_string(),
        ))
        .await
        .unwrap();

    while let Some(msg) = ws_stream.next().await {
        let event = msg.unwrap();
        match event {
            Message::Text(text) => {
                println!("{}", text)
            }
            _ => (),
        }
    }
}
