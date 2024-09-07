use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::{mpsc, oneshot};
use tracing::error;
use uuid::Uuid;

use crate::web_socket::{ConnectionStatus, WebSocketClient};

pub mod trade;

const CHANNEL_BUFFER: usize = 1024;

#[derive(Debug, Error)]
pub enum WebSocketStreamError {
    #[error("websocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("json parse error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("client error: {0}")]
    Client(String),
}

pub struct WebSocketStreamClient {
    // request_sender: Option<mpsc::Sender<RequestEnvelope>>,
    endpoint: String,
}

impl WebSocketStreamClient {
    pub fn new(endpoint: &str) -> Self {
        Self {
            // request_sender: None,
            endpoint: endpoint.to_owned(),
        }
    }
}

#[async_trait::async_trait]
pub trait Stream {
    type Payload: Payload;

    fn name(&self) -> &str;
    fn client(&self) -> &WebSocketStreamClient;
    fn request_sender(&self) -> &Option<mpsc::Sender<RequestEnvelope>>;
    fn request_sender_mut(&mut self) -> &mut Option<mpsc::Sender<RequestEnvelope>>;

    async fn connect(
        &mut self,
        data_channel: mpsc::Sender<Self::Payload>,
        status_channel: mpsc::Sender<ConnectionStatus>,
    ) -> Result<(), WebSocketStreamError> {
        let (request_sender, mut request_receiver) = mpsc::channel(256);
        self.request_sender_mut().replace(request_sender);

        let (write_channel, peer_read_channel) = mpsc::channel(CHANNEL_BUFFER);
        let (peer_write_channel, mut read_channel) = mpsc::channel(CHANNEL_BUFFER);
        let (status_relay_tx, mut status_relay_rx) = mpsc::channel(CHANNEL_BUFFER);

        let client = WebSocketClient::new(
            &format!("{}/ws/{}", self.client().endpoint, self.name()),
            peer_read_channel,
            peer_write_channel,
            status_relay_tx,
        );
        client.connect().await?;

        let mut pending_requests = HashMap::new();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Some((req, id, channel)) = request_receiver.recv() => {
                        match write_channel.send(req).await {
                            Ok(_) => {
                                pending_requests.insert(id, channel);
                            }
                            Err(err) => {
                                error!("write error: {err}");
                                drop(channel);
                            }
                        }
                    }
                    Some(msg) = read_channel.recv() => {
                        println!("msg: {msg}");
                        match serde_json::from_str::<ResponseFrame>(&msg) {
                            Ok(res) => {
                                match pending_requests.remove(&res.id) {
                                    Some(channel) => {
                                        let _ = channel.send(msg);
                                    }
                                    None => {
                                        error!("unexpected message: {msg}");
                                    }
                                }
                            }
                            Err(_) => {
                                match serde_json::from_str::<Self::Payload>(&msg) {
                                    Ok(payload) => {
                                        let _ = data_channel.send(payload).await;
                                    }
                                    Err(err) => {
                                        error!("json parse error: {err}");
                                    }
                                }
                            }
                        }
                    }
                    Some(status) = status_relay_rx.recv() => {
                        status_channel.send(status).await.unwrap_or_else(|err| {
                            error!("status relay error: {err}");
                        });
                    }
                }
            }
        });

        Ok(())
    }

    async fn subscribe(&self, params: &[&str]) -> Result<(), WebSocketStreamError> {
        let method = "SUBSCRIBE";
        let params = params.iter().map(|s| (*s).to_owned()).collect();
        self.request(method, Some(params)).await.map(|_| ())
    }

    async fn unsubscribe(&self, params: &[&str]) -> Result<(), WebSocketStreamError> {
        let method = "UNSUBSCRIBE";
        let params = params.iter().map(|s| (*s).to_owned()).collect();
        self.request(method, Some(params)).await.map(|_| ())
    }

    async fn list_subscriptions(
        &self,
        params: &[&str],
    ) -> Result<Vec<String>, WebSocketStreamError> {
        let method = "LIST_SUBSCRIPTIONS";
        let params = params.iter().map(|s| (*s).to_owned()).collect();
        self.request(method, Some(params))
            .await
            .map(|res| res.result.unwrap_or_default())
    }

    async fn request(
        &self,
        method: &str,
        params: Option<Vec<String>>,
    ) -> Result<ResponseFrame, WebSocketStreamError> {
        let id = Uuid::new_v4().to_string();
        let req = RequestFrame {
            id: id.clone(),
            method: method.to_owned(),
            params,
        };

        let (tx, rx) = oneshot::channel();
        let envelope = (serde_json::to_string(&req)?, id, tx);
        self.request_sender()
            .as_ref()
            .ok_or_else(|| WebSocketStreamError::Client("not connected".to_owned()))?
            .send(envelope)
            .await
            .map_err(|err| WebSocketStreamError::Client(format!("send request error: {err}")))?;

        let res = rx.await.map_err(|err| {
            WebSocketStreamError::Client(format!("receive response error: {err}"))
        })?;
        Ok(serde_json::from_str(&res)?)
    }
}

/// RequestEnvelope is a tuple of the request string, the request id, and a
/// oneshot sender to send the response back to the caller.
type RequestEnvelope = (String, String, oneshot::Sender<String>);

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestFrame {
    id: String,
    method: String,
    params: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct ResponseFrame {
    id: String,
    result: Option<Vec<String>>,
}

pub trait Payload: for<'de> Deserialize<'de> + Clone + Send + 'static {}
