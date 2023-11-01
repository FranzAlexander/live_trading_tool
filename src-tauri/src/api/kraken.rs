use hmac::{Hmac, Mac, NewMac};
use reqwest::{header::HeaderMap, Client};
use sha2::{Digest, Sha256, Sha512};
use std::{collections::HashMap, time::SystemTime};

use crate::model::kraken::{ApiResponse, MarketTrade, MarketTradeResponse};

type HmacSha512 = Hmac<Sha512>;

pub const KRAKEN_API_URL: &str = "https://api.kraken.com";
pub const KRAKEN_TRADES_ENDPOINT: &str = "/0/public/Trades?pair=";

pub async fn post_request<T: for<'de> serde::Deserialize<'de>>(
    client: &Client,
    url: &str,
    headers: HeaderMap,
    post_data: String,
) -> T {
    client
        .post(url)
        .headers(headers)
        .body(post_data)
        .send()
        .await
        .unwrap()
        .json::<T>()
        .await
        .unwrap()
}

pub async fn get_request<T: for<'de> serde::Deserialize<'de>>(client: &Client, url: &str) -> T {
    client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<T>()
        .await
        .unwrap()
}

pub fn get_kraken_signature(
    url_path: &str,
    data: &HashMap<String, String>,
    secret_key: &str,
) -> (String, String) {
    let post_data: String = data
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<String>>()
        .join("&");

    let sha2_result = {
        let nonce = data.get("nonce").unwrap();
        let mut hasher = Sha256::default();
        hasher.update(nonce.to_string());
        hasher.update(post_data.clone().as_bytes());
        hasher.finalize()
    };

    let private_key = base64::decode(secret_key.as_bytes()).unwrap();

    let mut mac = HmacSha512::new_from_slice(&private_key).unwrap();
    mac.update(url_path.as_bytes());
    mac.update(&sha2_result);

    (post_data, base64::encode(mac.finalize().into_bytes()))
}

/// Get a nonce as suggsted by Kraken
fn get_nonce() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

pub async fn get_market_trades(client: &reqwest::Client) -> Vec<MarketTrade> {
    let url = format!("{}{}{}", KRAKEN_API_URL, KRAKEN_TRADES_ENDPOINT, "SOLUSD");

    let response = get_request::<ApiResponse<MarketTradeResponse>>(client, &url).await;

    if let Some((_, market_trades)) = response.result.trades.iter().next() {
        return market_trades.to_owned();
    }

    Vec::new()
}
