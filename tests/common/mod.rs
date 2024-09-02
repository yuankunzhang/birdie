use std::env;

use birdie::Birdie;

pub fn setup() -> Birdie {
    let base_url = "https://testnet.binance.vision/";
    let api_key = env::var("BINANCE_API_KEY").unwrap();
    let secret_key = env::var("BINANCE_SECRET_KEY").unwrap();
    Birdie::new(base_url, &api_key, &secret_key).unwrap()
}
