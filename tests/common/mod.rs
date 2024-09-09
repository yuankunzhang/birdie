#![allow(dead_code)]

use std::env;

use birdie::{rest_api::RestApiClient, web_socket_api::WebSocketApiClient};

pub fn setup_rest_api_client() -> RestApiClient {
    let base_url =
        env::var("BINANCE_REST_API_ENDPOINT").expect("BINANCE_REST_API_ENDPOINT is not set");
    let api_key = env::var("BINANCE_API_KEY").expect("BINANCE_API_KEY is not set");
    let secret_key = env::var("BINANCE_SECRET_KEY").expect("BINANCE_SECRET_KEY is not set");
    RestApiClient::new(&base_url, &api_key, &secret_key).expect("Failed to create RestApiClient")
}

pub fn setup_web_socket_api_client() -> WebSocketApiClient {
    let endpoint = env::var("BINANCE_WEB_SOCKET_API_ENDPOINT")
        .expect("BINANCE_WEB_SOCKET_API_ENDPOINT is not set");
    let api_key =
        env::var("BINANCE_WEB_SOCKET_API_KEY").expect("BINANCE_WEB_SOCKET_API_KEY is not set");
    let secret_key = env::var("BINANCE_WEB_SOCKET_SECRET_KEY")
        .expect("BINANCE_WEB_SOCKET_SECRET_KEY is not set");
    WebSocketApiClient::new(&endpoint, &api_key, &secret_key)
}
