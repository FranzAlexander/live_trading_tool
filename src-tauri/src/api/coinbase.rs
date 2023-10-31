use hmac::{Hmac, Mac, NewMac};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use sha2::Sha256;

use crate::model::coinbase::MarketTrades;

const MARKET_TRADES_API:&str = "https://api.coinbase.com/api/v3/brokerage/products";
const MARKET_TRADES_REQUEST:&str = "/api/v3/brokerage/products";

type HmacSha256 = Hmac<Sha256>;

pub fn http_sign(
    secret_ket: &[u8],
    timestamp: &str,
    method: &str,
    request_path: &str,
    body: &str,
) -> String {
    let message = format!("{}{}{}{}", timestamp, method, request_path, body);

    let mut mac = HmacSha256::new_from_slice(secret_ket).expect("HMAC can take key of any size");
    mac.update(message.as_bytes());
    let result = mac.finalize();
    format!("{:x}", result.into_bytes())
}

pub fn create_headers(
    secret_key: &[u8],
    api_key: &str,
    method: &str,
    request_path: &str,
    body: &str,
) -> HeaderMap {
    let timestamp = format!("{}", chrono::Utc::now().timestamp());

    let signature = http_sign(secret_key, &timestamp, method, request_path, body);

    let mut headers = HeaderMap::new();
    headers.insert("CB-ACCESS-KEY", HeaderValue::from_str(api_key).unwrap());
    headers.insert("CB-ACCESS-SIGN", HeaderValue::from_str(&signature).unwrap());
    headers.insert(
        "CB-ACCESS-TIMESTAMP",
        HeaderValue::from_str(&timestamp).unwrap(),
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    headers
}

pub async fn send_get_request<T: for<'de> serde::Deserialize<'de>>(
    client: &reqwest::Client,
    url: &str,
    headers: HeaderMap,
) -> Result<T, reqwest::Error> {
    Ok(client
        .get(url)
        .headers(headers)
        .send()
        .await
        .unwrap()
        .json::<T>()
        .await
        .unwrap())
}

pub async fn get_market_trades(client: &reqwest::Client, secret_key: &[u8], api_key: &str){
    let path = format!("{}/{}/{}",MARKET_TRADES_REQUEST,"SOL-USD","ticker");
    let headers = create_headers(secret_key, api_key, "GET", &path, "");
    let url = format!("{}/{}/{}",MARKET_TRADES_API,"SOL-USD","ticker");

    let result = send_get_request::<MarketTrades>(client, &url, headers).await;

    println!("{:?}", result.unwrap())
}
