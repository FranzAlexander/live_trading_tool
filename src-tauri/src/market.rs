use std::collections::HashMap;

use crate::{
    api::{get_market_data, KRAKEN_OHLC_ENDPOINT},
    model::{MarketDataResponse, OHLCResponse, OHLC},
};

#[tauri::command]
pub fn get_ohlc_history() -> Vec<OHLC> {
    let mut url_value = HashMap::new();
    url_value.insert("pair".to_string(), "XRPUSD".to_string());
    url_value.insert("interval".to_string(), "5".to_string());

    let ohlc_response = get_market_data::<MarketDataResponse<OHLCResponse>>(
        KRAKEN_OHLC_ENDPOINT.to_string(),
        // url_value,
    );

    let mut coin_key = "";

    for key in ohlc_response.result.tickers.keys() {
        coin_key = key;
    }

    ohlc_response
        .result
        .tickers
        .get(coin_key)
        .unwrap()
        .to_owned()

    // let mut intervals: HashMap<i64, OHLC> = HashMap::new();

    // let interval_duration = 5 * 60; // 5 minutes in seconds

    // for ohlc in ohlc_data {
    //     let interval_start = (ohlc.time / interval_duration) * interval_duration;
    //     let interval = intervals.entry(interval_start).or_insert(OHLC {
    //         time: interval_start,
    //         open: ohlc.open,
    //         high: ohlc.high,
    //         low: ohlc.low,
    //         close: ohlc.close,
    //         vwap: ohlc.vwap,
    //         volume: ohlc.volume,
    //         count: ohlc.count,
    //     });

    //     // Update high and low within the interval
    //     if ohlc.high > interval.high {
    //         interval.high = ohlc.high;
    //     }
    //     if ohlc.low < interval.low {
    //         interval.low = ohlc.low;
    //     }

    //     // Update the close to the latest OHLC data point's close
    //     interval.close = ohlc.close;
    //     // You can similarly update vwap, volume, and count as needed.
    // }

    // intervals
    //     .into_iter()
    //     .map(|(_, interval)| interval)
    //     .collect()
}
