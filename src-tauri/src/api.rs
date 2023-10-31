// use std::{collections::HashMap, fmt::format, time::SystemTime};

// use hmac::{Hmac, Mac, NewMac};
// use reqwest::{
//     blocking::Client,
//     header::{HeaderMap, HeaderValue},
// };
// use sha2::{Digest, Sha256, Sha512};

// use crate::model::{
//     ApiResponse, ApiResponseMap, AppState, AssetInfo, ExtendedBalance, TradeBalance,
// };

// type HmacSha512 = Hmac<Sha512>;

// pub const KRAKEN_URL: &str = "wss://ws.kraken.com";

// pub const KRAKEN_OHLC_ENDPOINT: &str = "https://api.kraken.com/0/public/OHLC?pair=";
// pub const KRAKEN_TRADEABLE_PAIR_ENDPOINT: &str = "https://api.kraken.com/0/public/AssetPairs?pair=";

// const KRAKEN_API_URL: &str = "https://api.kraken.com";

// const KRAKEN_EXT_BALANCE_ENDPOINT: &str = "/0/private/BalanceEx";
// const KRAKEN_TRADE_BALANCE_ENDPOINT: &str = "/0/private/TradeBalance";

// pub fn post_request<T: for<'de> serde::Deserialize<'de>>(
//     client: &Client,
//     url: String,
//     headers: HeaderMap,
//     post_data: String,
// ) -> T {
//     client
//         .post(&url)
//         .headers(headers)
//         .body(post_data)
//         .send()
//         .unwrap()
//         .json::<T>()
//         .unwrap()
// }

// pub fn get_market_data<T: for<'de> serde::Deserialize<'de>>(url: String) -> T {
//     let request_url = format!("{}{}&interval={}", url, "XRPUSD", "5");

//     reqwest::blocking::get(&request_url)
//         .unwrap()
//         .json::<T>()
//         .unwrap()
// }

// #[tauri::command]
// pub fn get_tradeable_assets(symbols: Vec<String>, state: tauri::State<AppState>) {
//     let pairs = symbols.join(",");
//     let url = format!("{}{}", KRAKEN_TRADEABLE_PAIR_ENDPOINT, pairs);

//     let res = state.client.get(&url).send().unwrap().text().unwrap();
//     println!("{:?}", res);
// }

// #[tauri::command]
// pub fn get_extended_balance(
//     state: tauri::State<AppState>,
// ) -> HashMap<std::string::String, ExtendedBalance> {
//     let api_key = &state.api_key;

//     let mut data = HashMap::new();
//     data.insert("nonce".to_string(), get_nonce().to_string());

//     let (post_data, sign) =
//         get_kraken_signature(KRAKEN_EXT_BALANCE_ENDPOINT, &data, &state.secret_key);

//     let mut headers = HeaderMap::new();
//     headers.insert("API-Key", HeaderValue::from_str(&api_key).unwrap());
//     headers.insert("API-Sign", HeaderValue::from_str(&sign).unwrap());

//     let res = post_request::<ApiResponseMap<ExtendedBalance>>(
//         &state.client,
//         format!("{}{}", KRAKEN_API_URL, KRAKEN_EXT_BALANCE_ENDPOINT),
//         headers,
//         post_data,
//     );

//     res.result
// }

// #[tauri::command]
// pub fn get_trade_balance(state: tauri::State<AppState>) -> TradeBalance {
//     let api_key = &state.api_key;

//     let mut data = HashMap::new();
//     data.insert("nonce".to_string(), get_nonce().to_string());
//     let (post_data, sign) =
//         get_kraken_signature(KRAKEN_TRADE_BALANCE_ENDPOINT, &data, &state.secret_key);

//     let mut headers = HeaderMap::new();
//     headers.insert("API-Key", HeaderValue::from_str(&api_key).unwrap());
//     headers.insert("API-Sign", HeaderValue::from_str(&sign).unwrap());

//     let res = post_request::<ApiResponse<TradeBalance>>(
//         &state.client,
//         format!("{}{}", KRAKEN_API_URL, KRAKEN_TRADE_BALANCE_ENDPOINT),
//         headers,
//         post_data,
//     );

//     res.result
// }

// pub fn get_kraken_signature(
//     url_path: &str,
//     data: &HashMap<String, String>,
//     secret_key: &str,
// ) -> (String, String) {
//     let post_data: String = data
//         .iter()
//         .map(|(k, v)| format!("{}={}", k, v))
//         .collect::<Vec<String>>()
//         .join("&");

//     let sha2_result = {
//         let nonce = data.get("nonce").unwrap();
//         let mut hasher = Sha256::default();
//         hasher.update(nonce.to_string());
//         hasher.update(post_data.clone().as_bytes());
//         hasher.finalize()
//     };

//     let private_key = base64::decode(secret_key.as_bytes()).unwrap();

//     let mut mac = HmacSha512::new_from_slice(&private_key).unwrap();
//     mac.update(url_path.as_bytes());
//     mac.update(&sha2_result);

//     (post_data, base64::encode(mac.finalize().into_bytes()))
// }

// /// Get a nonce as suggsted by Kraken
// fn get_nonce() -> u64 {
//     SystemTime::now()
//         .duration_since(SystemTime::UNIX_EPOCH)
//         .unwrap()
//         .as_millis() as u64
// }
