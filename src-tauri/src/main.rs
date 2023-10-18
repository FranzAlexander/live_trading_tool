// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

use api::{get_extended_balance, get_market_data, KRAKEN_OHLC_ENDPOINT};
use ema::Ema;
use futures::{SinkExt, StreamExt};
use hmac::{Hmac, Mac, NewMac};
use model::{AppState, OHLCResponse, OHLC};
use reqwest::blocking::Client;
use sha2::{Digest, Sha256, Sha512};
use tauri::{window, Manager, Window};
use tokio::time::sleep;
use tokio_tungstenite::{connect_async, tungstenite::Message};
// use tokio_tungstenite::{connect_async, tungstenite::Message};

use market::get_ohlc_history;

use crate::{
    api::get_trade_balance,
    ema::{create_ema, ema_history},
};

mod api;
mod ema;
mod indicator;
mod market;
mod model;

// const API_KEY: String = std::env::var("KRAKEN_API_KEY").unwrap();
// const SECRET_KEY: String = std::env::var("KRAKEN_SECRET_KEY").unwrap();

// const TOKEN_ENDPOINT: &str = "https://api.kraken.com/0/private/GetWebSocketsToken";

// fn get_kraken_signature(request_path: String, post_data: &str) -> String {
//     let secret_key = std::env::var("KRAKEN_SECRET_KEY").unwrap();
//     let nonce = get_nonce();

//     let p_data = if post_data.is_empty() {
//         format!("nonce={}", nonce)
//     } else {
//         format!("nonce={}&{}", nonce, post_data)
//     };

//     let hash_digest = Sha256::digest(p_data.as_bytes());
//     let private_key = base64::decode(secret_key.as_bytes()).unwrap();

//     let mut mac = HmacSha512::new_from_slice(&private_key).unwrap();
//     let mut hmac_data = request_path.into_bytes();
//     hmac_data.append(&mut hash_digest.to_vec());
//     mac.update(&hmac_data);
//     base64::encode(mac.finalize().into_bytes())
// }

// /// Get a nonce as suggsted by Kraken
// fn get_nonce() -> u64 {
//     SystemTime::now()
//         .duration_since(SystemTime::UNIX_EPOCH)
//         .unwrap()
//         .as_millis() as u64
// }

// #[tauri::command]
// async fn connect_to_websocket(window: Window) {
//     match connect_async(KRAKEN_URL).await {
//         Ok((mut socket, _)) => {
//             let subscription_msg = serde_json::json!({
//             "event": "subscribe",
//             "pair": ["BTC/USD"],
//             "subscription": {
//                 "name": "ticker"
//             }});

//             let _ = socket
//                 .send(tokio_tungstenite::tungstenite::Message::Text(
//                     subscription_msg.to_string(),
//                 ))
//                 .await;

//             // Spawn a new asynchronous task to listen for messages
//             tokio::spawn(async move {
//                 while let Some(message) = socket.next().await {
//                     match message {
//                         Ok(Message::Text(text)) => {
//                             // Emit the received message to the specific window
//                             window
//                                 .emit("websocket_message", Payload { message: text })
//                                 .expect("Failed to emit event");
//                         }
//                         Ok(Message::Binary(bin)) => {
//                             // For simplicity, converting binary to base64 and sending as string
//                             let b64_data = base64::encode(&bin);
//                             window
//                                 .emit("websocket_message", Payload { message: b64_data })
//                                 .expect("Failed to emit event");
//                         }
//                         _ => {}
//                     }
//                 }
//             });

//             // Ok("Connected and listening to WebSocket".into())
//         }
//         Err(_) => (),
//     }
// }

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[derive(Clone, serde::Serialize)]
struct OhlcPayload {
    message: Vec<OHLC>,
}

#[tauri::command]
async fn init_process(window: Window) {
    // tauri::async_runtime::spawn(async move {
    //     match connect_async(KRAKEN_URL).await {
    //         Ok((mut ws_stream, _)) => {
    //             let subscription_msg = serde_json::json!({
    //             "event": "subscribe",
    //             "pair": ["BTC/USD"],
    //             "subscription": {
    //                 "interval": 5,
    //                 "name": "ohlc"
    //             }});

    //             let _ = ws_stream
    //                 .send(Message::Text(subscription_msg.to_string()))
    //                 .await;

    //             while let Some(message) = ws_stream.next().await {
    //                 let msg = message.expect("Error reading  message");

    //                 let _ = match msg {
    //                     Message::Text(data) => {
    //                         window.emit(
    //                             "message-stream",
    //                             Payload {
    //                                 message: format!("{}", data).into(),
    //                             },
    //                         );
    //                         println!("MESSAGE");
    //                     }
    //                     Message::Binary(_)
    //                     | Message::Close(_)
    //                     | Message::Frame(_)
    //                     | Message::Ping(_)
    //                     | Message::Pong(_) => Ok(()),
    //                 };
    //             }
    //         }
    //         Err(_) => (),
    //     }
    // });
    // std::thread::spawn(move || loop {
    //     window
    //         .emit(
    //             "message-stream",
    //             Payload {
    //                 message: format!("{}, Count: {}", "Tauri is awesome!", count).into(),
    //             },
    //         )
    //         .unwrap();

    //     count += 1;
    // });
}

fn main() {
    dotenv::dotenv().ok();
    tauri::Builder::default()
        .manage(AppState {
            client: Client::new(),
            api_key: std::env::var("API_KEY").unwrap(),
            secret_key: std::env::var("SECRET_KEY").unwrap(),
        })
        .setup(|app| {
            dotenv::dotenv().ok();
            // `main` here is the window label; it is defined on the window creation or under `tauri.conf.json`
            // the default value is `main`. note that it must be unique
            // let main_window = app.get_window("main").unwrap();

            // tauri::async_runtime::spawn(test_market_data(main_window));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            init_process,
            get_ohlc_history,
            get_extended_balance,
            get_trade_balance,
            create_ema,
            ema_history
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// async fn start_websocket(window: tauri::Window) {
//     let (ws_stream, _) = connect_async(KRAKEN_URL)
//         .await
//         .expect("Error connecting to websocket!");

//     let subscription_msg = serde_json::json!({
//     "event": "subscribe",
//     "pair": ["BTC/USD"],
//     "subscription": {
//         "interval": 5,
//         "name": "ohlc"
//     }});

//     let (mut write, mut read) = ws_stream.split();

//     write
//         .send(Message::Text(subscription_msg.to_string()))
//         .await
//         .expect("failed to send sub message");

//     while let Some(msg) = read.next().await {
//         let message = msg.expect("Failed to read  message");

//         let _ = match message {
//             Message::Text(text) => window.emit(
//                 "message-stream",
//                 Payload {
//                     message: format!("{}", text).into(),
//                 },
//             ),
//             _ => Ok(()),
//         };
//     }
// }
