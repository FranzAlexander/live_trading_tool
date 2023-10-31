#[derive(Debug,serde::Deserialize,serde::Serialize)]
pub struct MarketTrades{

}

#[derive(Debug,serde::Deserialize,serde::Serialize)]
pub struct Trade{
    trade_id: String,
    product_id:String,
    price:String,
}