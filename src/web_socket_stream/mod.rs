use serde::Deserialize;
use thiserror::Error;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite;
use tracing::error;

use crate::web_socket::{ConnectionStatus, WebSocketClient};

const CHANNEL_BUFFER: usize = 2048;

#[derive(Debug, Error)]
pub enum WebSocketStreamError {
    #[error("websocket error: {0}")]
    WebSocket(#[from] tungstenite::Error),
}

pub async fn connect_streams<P>(
    endpoint: &str,
    streams: &[&str],
    data_channel: mpsc::Sender<P>,
    status_channel: mpsc::Sender<ConnectionStatus>,
) -> Result<(), WebSocketStreamError>
where
    P: Payload,
{
    let (_, peer_read_channel) = mpsc::channel(CHANNEL_BUFFER);
    let (peer_write_channel, mut read_channel) = mpsc::channel(CHANNEL_BUFFER);
    let (status_relay_tx, mut status_relay_rx) = mpsc::channel(CHANNEL_BUFFER);

    let client = WebSocketClient::new(
        &format!("{endpoint}/stream?streams={}", streams.join("/")),
        peer_read_channel,
        peer_write_channel,
        status_relay_tx,
    );
    client.connect().await?;

    tokio::spawn(async move {
        loop {
            tokio::select! {
                Some(msg) = read_channel.recv() => {
                    println!("msg: {msg}");
                    let payload = match serde_json::from_str::<P>(&msg) {
                        Ok(payload) => payload,
                        Err(err) => {
                            error!("json parse error: {err}");
                            continue;
                        }
                    };
                    data_channel.send(payload).await.unwrap_or_else(|err| {
                        error!("data channel error: {err}");
                    });
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

pub trait Payload: for<'de> Deserialize<'de> + Clone + Send + 'static {}
