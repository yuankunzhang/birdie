use std::env;

use birdie::{enums::KlineInterval, rest_api::Endpoint, spot::market::KlinesParams};

#[tokio::main]
async fn main() {
    let endpoint =
        env::var("BINANCE_REST_API_ENDPOINT").expect("BINANCE_REST_API_ENDPOINT is required");
    let api_key = env::var("BINANCE_API_KEY").expect("BINANCE_API_KEY is required");
    let secret_key = env::var("BINANCE_SECRET_KEY").expect("BINANCE_SECRET_KEY is required");
    let client =
        birdie::rest_api(&endpoint, &api_key, &secret_key).expect("Failed to create client");

    let params = KlinesParams::new("BTCUSDT", KlineInterval::OneMinute).limit(5);
    let klines = client
        .spot()
        .market()
        .klines()
        .request(params)
        .await
        .expect("Failed to get account information");

    println!(" Timestamp     | Open           | High           | Low            | Close");
    println!("---------------+----------------+----------------+----------------+----------------");

    for kline in klines {
        let (ts, open, high, low, close) = (kline.0, kline.1, kline.2, kline.3, kline.4);
        println!(" {ts} | {open} | {high} | {low} | {close}",);
    }
}
