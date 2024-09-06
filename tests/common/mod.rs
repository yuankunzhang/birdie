#![allow(dead_code)]

use std::env;

use birdie::{web_socket_api::WebSocketApiClient, Birdie};

pub fn setup() -> Birdie {
    let base_url = env::var("BINANCE_BASE_URL").unwrap();
    let api_key = env::var("BINANCE_API_KEY").unwrap();
    let secret_key = env::var("BINANCE_SECRET_KEY").unwrap();
    Birdie::new(&base_url, &api_key, &secret_key).unwrap()
}

pub fn setup_web_socket_api_client() -> WebSocketApiClient {
    let endpoint = env::var("BINANCE_WEB_SOCKET_API_ENDPOINT").unwrap();
    let api_key = env::var("BINANCE_WEB_SOCKET_API_KEY").unwrap();
    let secret_key = env::var("BINANCE_WEB_SOCKET_SECRET_KEY").unwrap();
    WebSocketApiClient::new(&endpoint, &api_key, &secret_key)
}
