use std::env;

use birdie::{rest_api::Endpoint, spot::account::AccountInformationParams};

#[tokio::main]
async fn main() {
    let endpoint =
        env::var("BINANCE_REST_API_ENDPOINT").expect("BINANCE_REST_API_ENDPOINT is required");
    let api_key = env::var("BINANCE_API_KEY").expect("BINANCE_API_KEY is required");
    let secret_key = env::var("BINANCE_SECRET_KEY").expect("BINANCE_SECRET_KEY is required");
    let client =
        birdie::rest_api(&endpoint, &api_key, &secret_key).expect("Failed to create client");

    let params = AccountInformationParams::new().omit_zero_balances(true);
    let account_info = client
        .spot()
        .account()
        .account_information()
        .request(params)
        .await
        .expect("Failed to get account information");

    println!("Account Balances\n");
    println!(" Asset   | Free         | Locked");
    println!("---------+--------------+------------");
    for balance in account_info.balances {
        println!(
            "{:<8} | {:<12} | {:<12}",
            balance.asset, balance.free, balance.locked
        );
    }
}
