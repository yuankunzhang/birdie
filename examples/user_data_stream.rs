use std::env;

use birdie::{
    rest_api::Endpoint,
    spot::user_data_stream::{StartUserDataStreamParams, UserDataStreamPayload},
    web_socket::ConnectionStatus,
    web_socket_stream::connect_raw_stream,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let rest_base_url = env::var("BINANCE_BASE_URL").expect("BINANCE_BASE_URL is not set");
    let rest_api_key = env::var("BINANCE_API_KEY").expect("BINANCE_API_KEY is not set");
    let rest_secret_key = env::var("BINANCE_SECRET_KEY").expect("BINANCE_SECRET_KEY is not set");

    let rest_api = birdie::rest_api(&rest_base_url, &rest_api_key, &rest_secret_key)
        .expect("Failed to initialize Rest API Client");

    let params = StartUserDataStreamParams::new();
    let resp = rest_api
        .user_data_stream()
        .start_user_data_stream()
        .request(params)
        .await
        .expect("Failed to start user data stream");
    let listen_key = resp.listen_key;
    println!("listen_key={listen_key}");

    let stream_endpoint = env::var("BINANCE_WEB_SOCKET_STREAM_ENDPOINT")
        .expect("BINANCE_WEB_SOCKET_STREAM_ENDPOINT is not set");

    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    let (data_writer, mut data_reader) = tokio::sync::mpsc::channel(1024);
    connect_raw_stream::<UserDataStreamPayload>(&stream_endpoint, &listen_key, data_writer, tx)
        .await
        .unwrap();

    loop {
        tokio::select! {
            Some(status) = rx.recv() => {
                match status {
                    ConnectionStatus::Connected => {
                        println!("connected");
                    }
                    ConnectionStatus::PingReceived => {
                        println!("ping");
                    }
                    ConnectionStatus::PoingSent => {
                        println!("pong");
                    }
                    ConnectionStatus::Disconnected => {
                        println!("disconnected");
                    }
                }
            }
            Some(data) = data_reader.recv() => {
                println!("{:?}", data);
            }
        }
    }
}
