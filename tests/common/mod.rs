#![allow(dead_code)]

use std::env;

use birdie::{rest_api::RestApiClient, web_socket_api::WebSocketApiClient};

pub fn setup_rest_api_client() -> RestApiClient {
    let base_url = env::var("BINANCE_BASE_URL").unwrap();
    let api_key = env::var("BINANCE_API_KEY").unwrap();
    let secret_key = env::var("BINANCE_SECRET_KEY").unwrap();
    RestApiClient::new(&base_url, &api_key, &secret_key).unwrap()
}

pub fn setup_web_socket_api_client() -> WebSocketApiClient {
    let endpoint = env::var("BINANCE_WEB_SOCKET_API_ENDPOINT").unwrap();
    let api_key = env::var("BINANCE_WEB_SOCKET_API_KEY").unwrap();
    let secret_key = env::var("BINANCE_WEB_SOCKET_SECRET_KEY").unwrap();
    WebSocketApiClient::new(&endpoint, &api_key, &secret_key)
}
