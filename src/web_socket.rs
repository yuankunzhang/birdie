use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite};
use tracing::{error, info};

#[derive(Clone, Copy, Debug)]
pub enum ConnectionStatus {
    Connected,
    PingReceived,
    PoingSent,
    Disconnected,
}

pub struct WebSocketClient {
    endpoint: String,
    read_channel: mpsc::Receiver<String>,
    write_channel: mpsc::Sender<String>,
    status_channel: mpsc::Sender<ConnectionStatus>,
}

impl WebSocketClient {
    pub fn new(
        endpoint: &str,
        read_channel: mpsc::Receiver<String>,
        write_channel: mpsc::Sender<String>,
        status_channel: mpsc::Sender<ConnectionStatus>,
    ) -> Self {
        Self {
            endpoint: endpoint.to_owned(),
            read_channel,
            write_channel,
            status_channel,
        }
    }

    pub async fn connect(mut self) -> Result<(), tungstenite::Error> {
        let (stream, _) = connect_async(&self.endpoint).await?;
        let (mut write, mut read) = stream.split();
        let _ = self.status_channel.send(ConnectionStatus::Connected).await;

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Some(msg) = self.read_channel.recv() => {
                        let msg = tungstenite::Message::Text(msg);
                        write.send(msg).await.unwrap_or_else(|err| {
                            error!("websocket write error: {err}");
                        })
                    }
                    Some(msg) = read.next() => {
                        let msg = match msg {
                            Ok(msg) => msg,
                            Err(err) => {
                                error!("websocket read error: {err}");
                                break;
                            }
                        };

                        match msg {
                            tungstenite::Message::Text(msg) => {
                                self.write_channel.send(msg).await.unwrap_or_else(|err| {
                                    error!("write channel error: {err}");
                                });
                            }
                            tungstenite::Message::Ping(payload) => {
                                info!("ping received");
                                self.status_channel.send(ConnectionStatus::PingReceived).await.unwrap_or_else(|err| {
                                    error!("status channel error: {err}");
                                });

                                write.send(tungstenite::Message::Pong(payload)).await.unwrap_or_else(|err| {
                                    error!("websocket write error: {err}");
                                });
                                info!("pong sent");

                                self.status_channel.send(ConnectionStatus::PoingSent).await.unwrap_or_else(|err| {
                                    error!("status channel error: {err}");
                                });
                            }
                            tungstenite::Message::Close(_) => {
                                self.status_channel.send(ConnectionStatus::Disconnected).await.unwrap_or_else(|err| {
                                    error!("status channel error: {err}");
                                });
                                break;
                            }
                            _ => {
                                error!("unexpected message: {msg}");
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }
}
