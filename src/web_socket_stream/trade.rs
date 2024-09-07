use serde::Deserialize;
use tokio::sync::mpsc;

use super::{Payload, RequestEnvelope, Stream, WebSocketStreamClient};

pub struct TradeStream<'w> {
    client: &'w WebSocketStreamClient,
    request_sender: Option<mpsc::Sender<RequestEnvelope>>,
}

impl<'w> TradeStream<'w> {
    pub fn new(client: &'w WebSocketStreamClient) -> Self {
        Self {
            client,
            request_sender: None,
        }
    }
}

impl Stream for TradeStream<'_> {
    type Payload = TradeStreamPayload;

    fn name(&self) -> &str {
        "btcusdt@trade"
    }

    fn client(&self) -> &WebSocketStreamClient {
        self.client
    }

    fn request_sender(&self) -> &Option<mpsc::Sender<RequestEnvelope>> {
        &self.request_sender
    }

    fn request_sender_mut(&mut self) -> &mut Option<mpsc::Sender<RequestEnvelope>> {
        &mut self.request_sender
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct TradeStreamPayload {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "t")]
    pub trade_id: i64,
    #[serde(rename = "p")]
    pub price: String,
    #[serde(rename = "q")]
    pub quantity: String,
    #[serde(rename = "T")]
    pub trade_time: i64,
    #[serde(rename = "m")]
    pub is_market_maker: bool,
    #[serde(rename = "M")]
    pub ignore: bool,
}

impl Payload for TradeStreamPayload {}
