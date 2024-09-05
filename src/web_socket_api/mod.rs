//! Binance's WebSocket API - stub

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::{
    sync::{mpsc, oneshot},
};
use tokio_tungstenite::connect_async;

use crate::enums::SecurityType;

#[derive(Debug, Error)]
pub enum WebSocketApiError {
    #[error("websocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("json parse error: {0}")]
    Json(#[from] serde_json::Error),
}

pub struct WebSocketApiClient {
    pub(self) request_sender: mpsc::Sender<RequestEnvelope>,
    _api_key: String,
    _secret_key: String,
}

impl WebSocketApiClient {
    pub async fn new(
        endpoint: &str,
        api_key: &str,
        secret_key: &str,
    ) -> Result<Self, WebSocketApiError> {
        let (request_sender, request_receiver) = mpsc::channel(100);
        let client = Self {
            request_sender,
            _api_key: api_key.to_owned(),
            _secret_key: secret_key.to_owned(),
        };

        client.run(endpoint, request_receiver).await?;
        Ok(client)
    }

    async fn run(
        &self,
        endpoint: &str,
        mut request_receiver: mpsc::Receiver<RequestEnvelope>,
    ) -> Result<(), WebSocketApiError> {
        let (_stream, _) = connect_async(endpoint).await?;
        // let (write, read) = stream.split();

        tokio::spawn(async move {
            while let Some((request, channel)) = request_receiver.recv().await {
                println!("request received: {}", request.id);
                if let Err(_) = channel.send(request.method) {
                    println!("the receiver dropped, maybe due to timeout");
                }
            }
        });

        Ok(())
    }

    pub async fn send_message(&self, payload: &str, response_sender: oneshot::Sender<String>) {
        let msg = Request {
            id: payload.to_owned(),
            method: "info".to_owned(),
            _params: "null".to_owned(),
        };
        self.request_sender.send((msg, response_sender)).await.unwrap();
    }
}

struct Request {
    id: String,
    method: String,
    _params: String,
}

type RequestEnvelope = (Request, oneshot::Sender<String>);

pub trait RequestFrame: Sized + Serialize {}

pub trait ResponseFrame: Sized + for<'de> Deserialize<'de> {}

pub trait Message {
    type Params: RequestFrame;
    type Response: ResponseFrame;

    fn client(&self) -> &WebSocketApiClient;
    fn method(&self) -> &str;
    fn security_type(&self) -> SecurityType;

    // async fn send(&self, params: Self::Params) -> Result<Self::Response, WebSocketApiError> {
    //     todo!()
    // }
}
