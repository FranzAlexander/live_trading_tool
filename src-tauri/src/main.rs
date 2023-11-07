// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use ema::Ema;

use std::{collections::VecDeque, sync::Arc};

use api::kraken::{get_market_trades, KRAKEN_WS_URL};
use chart::MinData;
use futures::{SinkExt, StreamExt};
use tauri::Manager;
use tokio_tungstenite::{connect_async, tungstenite::Message};

// use macd::Macd;
// use model::{AppState, IndicatorState, OHLC};
use crate::{
    api::kraken::{get_extended_balance, get_tradeable_asset_pair},
    model::{kraken::KrakenEvent, payload::OhlcPayload, AppState},
};
use reqwest::Client;
// use crate::{market::get_asset_info, user::get_symbols};
use tokio::sync::Mutex;

use range::{
    delta_bar::{self, DeltaBar},
    range_bar::RangeBar,
    Bar, RangeData, RangePayload,
};
// mod api;
// mod ema;
// mod indicator;
// mod macd;
// mod market;
// // mod model;
// mod user;
mod api;
mod chart;
mod model;
mod range;
mod rolling_window;

fn main() {
    dotenv::dotenv().ok();

    tauri::Builder::default()
        .manage(AppState {
            client: Client::new(),
            api_key: std::env::var("API_KEY").unwrap(),
            secret_key: std::env::var("SECRET_KEY").unwrap(),
            range_data: Arc::new(Mutex::new(RangeData::new(2.0 / 100.0))),
            one_min_data: Arc::new(Mutex::new(MinData::new("ohlc-1".to_string()))),
        })
        .setup(|app| {
            // `main` here is the window label; it is defined on the window creation or under `tauri.conf.json`
            // the default value is `main`. note that it must be unique
            // let main_window = app.get_window("main").unwrap();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            app_start,
            get_extended_balance,
            get_tradeable_asset_pair
        ])
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn app_start(
    app: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<(Vec<RangeBar>, Vec<DeltaBar>), String> {
    let trades = get_market_trades(&app.client).await;
    for trade in trades.iter() {
        app.range_data
            .lock()
            .await
            .update(trade.price, trade.volume, &trade.buy_sell, trade.time);
    }

    let (range_bars, delta_bars) = app.range_data.lock().await.get_bars();

    let mut sorted_range_bars: Vec<RangeBar> = range_bars.into_iter().collect();

    let mut sorted_delta_bars: Vec<DeltaBar> = delta_bars.into_iter().collect();

    let ws_range_data = app.range_data.clone();

    sorted_range_bars.sort_by_key(|item| item.start_time);
    sorted_delta_bars.sort_by_key(|item| item.start_time);

    let one_min_data = app.one_min_data.clone();

    tauri::async_runtime::spawn(kraken_websocket(ws_range_data, one_min_data, app_handle));

    Ok((sorted_range_bars, sorted_delta_bars))
}

async fn kraken_websocket(
    range_bars: Arc<Mutex<RangeData>>,
    one_min_data: Arc<Mutex<MinData>>,
    app_handle: tauri::AppHandle,
) {
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

    let subscription_msg = serde_json::json!({
    "event": "subscribe",
    "pair": ["SOL/USD"],
    "subscription": {
        "name":"ohlc",
        "interval":1
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
                let kraken_event: KrakenEvent = serde_json::from_str(&text).unwrap();

                match kraken_event {
                    KrakenEvent::MarketOrder(order) => {
                        let mut range_bar_lock = range_bars.lock().await;
                        let mut one_min_data = one_min_data.lock().await;
                        for trade in order.trades {
                            let new_bar = range_bar_lock.update(
                                trade.price,
                                trade.volume,
                                &trade.side,
                                trade.time,
                            );

                            one_min_data.update(
                                trade.price,
                                trade.volume,
                                &trade.side,
                                trade.time.round() as i64,
                            );

                            if let (Some(range_bar), Some(delta_bar)) = new_bar {
                                let _ = app_handle.emit_all("new_bar", range_bar);
                                let _ = app_handle.emit_all("new_delta_bar", delta_bar);
                            }
                        }
                    }
                    KrakenEvent::OhlcEvent(ohlc) => {
                        if ohlc.name == "ohlc-1" {
                            let mut one_min_data = one_min_data.lock().await;
                            one_min_data.update_end_time(ohlc.ohlc.etime as i64);
                            let payload = OhlcPayload::from(ohlc);
                            if let Some(one_min_cvd) = one_min_data.get_cvd() {
                                let _ = app_handle.emit_all("oneMinCVD", one_min_cvd);
                            }
                            let _ = app_handle.emit_all("oneMinOhlc", payload);
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
}
