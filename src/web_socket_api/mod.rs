//! Binance's WebSocket API
mod auth;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::{mpsc, oneshot};
use tracing::error;
use uuid::Uuid;

use crate::{
    enums::SecurityType,
    errors::BinanceError,
    spot::{account, general, market, trade},
    web_socket::{ConnectionStatus, WebSocketClient},
    Params, Response,
};
use auth::*;

const CHANNEL_BUFFER: usize = 1024;

#[derive(Debug, Error)]
pub enum WebSocketApiError {
    #[error("websocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("json parse error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("pkcs8 error: {0}")]
    Pkcs8(#[from] ed25519_dalek::pkcs8::Error),
    #[error("binance error: {0}")]
    Binance(String, Option<BinanceError>),
    #[error("client error: {0}")]
    Client(String),
}

pub struct WebSocketApiClient {
    request_sender: Option<mpsc::Sender<RequestEnvelope>>,
    endpoint: String,
    api_key: String,
    ed25519_key: String,
}

impl WebSocketApiClient {
    pub fn new(endpoint: &str, api_key: &str, ed25519_key: &str) -> Self {
        Self {
            request_sender: None,
            endpoint: endpoint.to_owned(),
            api_key: api_key.to_owned(),
            ed25519_key: ed25519_key.to_owned(),
        }
    }

    pub fn account(&self) -> account::WebSocketApiHandler {
        account::WebSocketApiHandler::new(self)
    }

    pub fn general(&self) -> general::WebSocketApiHandler {
        general::WebSocketApiHandler::new(self)
    }

    pub fn market(&self) -> market::WebSocketApiHandler {
        market::WebSocketApiHandler::new(self)
    }

    pub fn trade(&self) -> trade::WebSocketApiHandler {
        trade::WebSocketApiHandler::new(self)
    }

    pub async fn connect(
        &mut self,
        status_sender: mpsc::Sender<ConnectionStatus>,
    ) -> Result<(), WebSocketApiError> {
        let (request_sender, mut request_receiver) = mpsc::channel(CHANNEL_BUFFER);
        self.request_sender = Some(request_sender);

        let (write_channel, peer_read_channel) = mpsc::channel(CHANNEL_BUFFER);
        let (peer_write_channel, mut read_channel) = mpsc::channel(CHANNEL_BUFFER);
        let (status_relay_tx, mut status_relay_rx) = mpsc::channel(CHANNEL_BUFFER);

        let client = WebSocketClient::new(
            &self.endpoint,
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
                        let res: ResponseFrame<serde_json::Value> = match serde_json::from_str(&msg) {
                            Ok(res) => res,
                            Err(err) => {
                                error!("json parse error: {err}");
                                continue;
                            }
                        };

                        match pending_requests.remove(&res.id) {
                            Some(channel) => {
                                let _ = channel.send(msg);
                            }
                            None => {
                                error!("unexpected message: {msg}");
                            }
                        }
                    }
                    Some(status) = status_relay_rx.recv() => {
                        status_sender.send(status).await.unwrap_or_else(|err| {
                            error!("status relay error: {err}");
                        });
                    }
                }
            }
        });

        self.logon().await?;
        Ok(())
    }

    pub async fn logon(&self) -> Result<(), WebSocketApiError> {
        let mut params = LogonParams::new(&self.api_key);
        params.sign(&self.ed25519_key)?;
        let _: LogonResponse = self.request("session.logon", params).await?;
        Ok(())
    }

    pub async fn logout(&self) -> Result<(), WebSocketApiError> {
        let params = LogoutParams {};
        let _: LogoutResponse = self.request("session.logout", params).await?;
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

#[async_trait::async_trait]
pub trait WebSocket {
    type Params: Params;
    type Response: Response;

    fn client(&self) -> &WebSocketApiClient;
    fn method(&self) -> &str;
    fn security_type(&self) -> SecurityType;

    async fn request(&self, params: Self::Params) -> Result<Self::Response, WebSocketApiError> {
        self.client().request(self.method(), params).await
    }
}

macro_rules! web_socket {
    ($method:literal, $name:ident, $params:ty, $response:ty) => {
        #[async_trait::async_trait]
        impl crate::web_socket_api::WebSocket for $name<'_> {
            type Params = $params;
            type Response = $response;

            fn client(&self) -> &crate::web_socket_api::WebSocketApiClient {
                self.client
            }

            fn method(&self) -> &str {
                $method
            }

            fn security_type(&self) -> $crate::enums::SecurityType {
                $crate::enums::SecurityType::None
            }
        }
    };
    ($method:literal, $security:expr, $name:ident, $params:ty, $response:ty) => {
        #[async_trait::async_trait]
        impl crate::web_socket_api::WebSocket for $name<'_> {
            type Params = $params;
            type Response = $response;

            fn client(&self) -> &crate::web_socket_api::WebSocketApiClient {
                self.client
            }

            fn method(&self) -> &str {
                $method
            }

            fn security_type(&self) -> $crate::enums::SecurityType {
                $security
            }
        }
    };
}

macro_rules! ws_route {
    ($target:ident, $endpoint:ty) => {
        pub fn $target(&self) -> $endpoint {
            <$endpoint>::new(self.client)
        }
    };
}

pub(crate) use web_socket;
pub(crate) use ws_route;
