use std::time::SystemTime;

use hmac::{Hmac, Mac, NewMac};
use reqwest::header::{HeaderMap, HeaderValue};
use sha2::{Digest, Sha256, Sha512};

type HmacSha512 = Hmac<Sha512>;

pub const KRAKEN_URL: &str = "wss://ws.kraken.com";

pub const KRAKEN_OHLC_ENDPOINT: &str = "https://api.kraken.com/0/public/OHLC?pair=";
const KRAKEN_EXT_BALANCE_ENDPOINT: &str = "https://api.kraken.com/0/private/BalanceEx";

pub fn get_market_data<T: for<'de> serde::Deserialize<'de>>(
    url: String,
    // params: HashMap<String, String>,
) -> T {
    let request_url = format!("{}{}&interval={}", url, "XRPUSD", "5");

    reqwest::blocking::get(&request_url)
        .unwrap()
        .json::<T>()
        .unwrap()
}

pub fn get_extended_balance() {
    let api_key = std::env::var("API_KEY").unwrap();
    let sign = get_kraken_signature(KRAKEN_EXT_BALANCE_ENDPOINT.to_string(), "".to_string());

    println!("{api_key}");
    println!("{sign}");

    let mut headers = HeaderMap::new();
    headers.insert("API-Key", HeaderValue::from_str(&api_key).unwrap());
    headers.insert("API-Sign", HeaderValue::from_str(&sign).unwrap());

    let client = reqwest::blocking::Client::new();
    let res = client
        .get(KRAKEN_EXT_BALANCE_ENDPOINT)
        .headers(headers)
        .send()
        .unwrap()
        .text()
        .unwrap();

    println!("{:?}", res)
}

pub fn get_kraken_signature(request_path: String, post_data: String) -> String {
    let secret_key = std::env::var("SECRET_KEY").unwrap();
    let nonce = get_nonce();

    let p_data = if post_data.is_empty() {
        format!("nonce={}", nonce)
    } else {
        format!("nonce={}&{}", nonce, post_data)
    };

    let hash_digest = Sha256::digest(p_data.as_bytes());
    let private_key = base64::decode(secret_key.as_bytes()).unwrap();

    let mut mac = HmacSha512::new_from_slice(&private_key).unwrap();
    let mut hmac_data = request_path.into_bytes();
    hmac_data.append(&mut hash_digest.to_vec());
    mac.update(&hmac_data);
    base64::encode(mac.finalize().into_bytes())
}

/// Get a nonce as suggsted by Kraken
fn get_nonce() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}
