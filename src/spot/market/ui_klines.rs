use reqwest::Method;
use serde::Serialize;

use crate::{
    enums::{KlineInterval, SecurityType},
    rest_api::{Endpoint, RestApiClient},
    web_socket_api::web_socket,
    Params,
};

use super::Kline;

impl Params for UiKlinesParams {}

impl Endpoint for UiKlinesEndpoint<'_> {
    type Params = UiKlinesParams;
    type Response = UiKlinesResponse;

    fn client(&self) -> &RestApiClient {
        self.client
    }

    fn path(&self) -> &str {
        "/api/v3/uiKlines"
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn security_type(&self) -> SecurityType {
        SecurityType::None
    }
}

/// Kline/candlestick bars for a symbol. Klines are uniquely identified by their
/// open time.
///
/// - Weight: 2
/// - Data Source: Database
pub struct UiKlinesEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> UiKlinesEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UiKlinesParams {
    symbol: String,
    interval: KlineInterval,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}

impl UiKlinesParams {
    pub fn new(symbol: &str, interval: KlineInterval) -> Self {
        Self {
            symbol: symbol.to_owned(),
            interval,
            start_time: None,
            end_time: None,
            time_zone: None,
            limit: None,
        }
    }

    pub fn start_time(mut self, start_time: i64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: i64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /// Default: 0 (UTC).
    pub fn time_zone(mut self, time_zone: &str) -> Self {
        self.time_zone = Some(time_zone.to_owned());
        self
    }

    /// Default 500; max 1000.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}

pub type UiKlinesResponse = Vec<Kline>;

web_socket!(
    "uiKlines",
    UiKlinesWebSocket,
    UiKlinesParams,
    UiKlinesResponse
);

pub struct UiKlinesWebSocket<'w> {
    client: &'w crate::web_socket_api::WebSocketApiClient,
}

impl<'w> UiKlinesWebSocket<'w> {
    pub fn new(client: &'w crate::web_socket_api::WebSocketApiClient) -> Self {
        Self { client }
    }
}
