use std::collections::HashMap;

pub const KRAKEN_OHLC_ENDPOINT: &str = "https://api.kraken.com/0/public/OHLC?pair=";

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
