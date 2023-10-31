pub mod kraken;
pub mod coinbase;

pub struct AppState{
pub client: reqwest::Client,
pub api_key:String,
pub secret_key:String,
}