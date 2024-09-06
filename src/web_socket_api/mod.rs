//! Binance's WebSocket API - stub

use std::{cell::RefCell, collections::HashMap};

use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::{mpsc, oneshot};
use tokio_tungstenite::{connect_async, tungstenite};
use tracing::{error, info};
use uuid::Uuid;

use crate::{errors::BinanceError, Params, Response};

const REQUEST_PARALALISM: usize = 1000;

#[derive(Clone, Copy, Debug)]
pub enum ConnectionStatus {
    Connected,
    PingReceived,
    PoingSent,
    Disconnected,
}

#[derive(Debug, Error)]
pub enum WebSocketApiError {
    #[error("websocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("json parse error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("binance error: {0}")]
    Binance(String, Option<BinanceError>),
    #[error("client error: {0}")]
    Client(String),
}

pub struct WebSocketApiClient {
    request_sender: RefCell<Option<mpsc::Sender<RequestEnvelope>>>,
    endpoint: String,
    _api_key: String,
    _secret_key: String,
}

impl WebSocketApiClient {
    pub fn new(endpoint: &str, api_key: &str, secret_key: &str) -> Self {
        Self {
            request_sender: RefCell::new(None),
            endpoint: endpoint.to_owned(),
            _api_key: api_key.to_owned(),
            _secret_key: secret_key.to_owned(),
        }
    }

    pub fn is_connected(&self) -> bool {
        self.request_sender
            .borrow()
            .as_ref()
            .map(|s| !s.is_closed())
            .unwrap_or(false)
    }

    pub async fn connect(
        &self,
        status: mpsc::Sender<ConnectionStatus>,
    ) -> Result<(), WebSocketApiError> {
        let (request_sender, mut request_receiver) = mpsc::channel(REQUEST_PARALALISM);
        self.request_sender.replace(Some(request_sender));

        let (stream, _) = connect_async(&self.endpoint).await?;
        let (mut write, mut read) = stream.split();
        let _ = status.send(ConnectionStatus::Connected).await;

        let mut pending_requests = HashMap::new();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Some((req, id, channel)) = request_receiver.recv() => {
                        let frame = tungstenite::Message::Text(req);
                        match write.send(frame).await {
                            Ok(_) => {
                                pending_requests.insert(id, channel);
                            }
                            Err(err) => {
                                error!("websocket write error: {err}");
                                drop(channel);
                            }
                        }
                    }
                    Some(frame) = read.next() => {
                        let frame = match frame {
                            Ok(frame) => frame,
                            Err(err) => {
                                error!("websocket read error: {err}");
                                break;
                            }
                        };

                        match frame {
                            tungstenite::Message::Text(text) => {
                                let res: ResponseFrame<serde_json::Value> = match serde_json::from_str(&text) {
                                    Ok(res) => res,
                                    Err(err) => {
                                        error!("json parse error: {err}");
                                        continue;
                                    }
                                };

                                match pending_requests.remove(&res.id) {
                                    Some(channel) => {
                                        let _ = channel.send(text);
                                    }
                                    None => {
                                        error!("unexpected response: {text}");
                                    }
                                }
                            }
                            tungstenite::Message::Ping(payload) => {
                                let _ = status.send(ConnectionStatus::PingReceived).await.unwrap();
                                info!("ping received");

                                let _ = write.send(tungstenite::Message::Pong(payload)).await;
                                let _ = status.send(ConnectionStatus::PoingSent).await.unwrap();
                                info!("pong sent");
                            }
                            tungstenite::Message::Close(_) => {
                                info!("websocket closed by server");
                                let _ = status.send(ConnectionStatus::Disconnected);
                                break;
                            }
                            _ => {
                                error!("unexpected message: {frame}")
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }

    pub async fn request<P, R>(&self, method: &str, params: P) -> Result<R, WebSocketApiError>
    where
        P: Params,
        R: Response,
    {
        let id = Uuid::new_v4().to_string();
        let req = RequestFrame {
            id: &id,
            method,
            params,
        };

        let (tx, rx) = oneshot::channel();
        let envelope = (serde_json::to_string(&req)?, id, tx);
        self.request_sender
            .borrow()
            .as_ref()
            .ok_or_else(|| WebSocketApiError::Client("not connected".to_owned()))?
            .send(envelope)
            .await
            .map_err(|err| WebSocketApiError::Client(format!("send request error: {err}")))?;

        let res = rx
            .await
            .map_err(|err| WebSocketApiError::Client(format!("receive response error: {err}")))?;
        let res: ResponseFrame<R> = serde_json::from_str(&res)?;

        if let Some(result) = res.result {
            Ok(result)
        } else {
            Err(WebSocketApiError::Binance(
                res.status.to_string(),
                res.error,
            ))
        }
    }
}

/// RequestEnvelope is a tuple of the request string, the request id, and a
/// oneshot sender to send the response back to the caller.
type RequestEnvelope = (String, String, oneshot::Sender<String>);

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RequestFrame<'a, P: Params> {
    id: &'a str,
    method: &'a str,
    params: P,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResponseFrame<R> {
    id: String,
    #[serde(default)]
    status: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<BinanceError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<R>,
}
