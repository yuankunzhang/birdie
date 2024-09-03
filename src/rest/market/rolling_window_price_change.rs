use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::TickerType, rest::endpoint};

use super::{TickerFull, TickerMini};

endpoint!(
    "/api/v3/ticker",
    Method::GET,
    RollingWindowPriceChangeEndpoint,
    RollingWindowPriceChangeParams,
    RollingWindowPriceChangeResponse
);

/// **Note**: This endpoint is different from the GET /api/v3/ticker/24hr
/// endpoint.
///
/// The window used to compute statistics will be no more than 59999ms from the
/// requested windowSize.
///
/// The `openTime` always starts on a minute, while the `closeTime` is the
/// current time of the request. As such, the effective window will be up to
/// 59999ms wider than `windowSize`.
///
/// E.g. If the `closeTime` is 1641287867099 (January 04, 2022 09:17:47:099
/// UTC), and the `windowSize` is 1d. the openTime will be: 1641201420000
/// (January 3, 2022, 09:17:00).
///
/// - Weight:
/// - Data Source: Memory
pub struct RollingWindowPriceChangeEndpoint<'r> {
    client: &'r crate::rest::RestClient,
}

impl<'r> RollingWindowPriceChangeEndpoint<'r> {
    pub fn new(client: &'r crate::rest::RestClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RollingWindowPriceChangeParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    symbols: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    window_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<TickerType>,
}

impl Default for RollingWindowPriceChangeParams {
    fn default() -> Self {
        Self::new()
    }
}

impl RollingWindowPriceChangeParams {
    pub fn new() -> Self {
        Self {
            symbol: None,
            symbols: None,
            window_size: None,
            r#type: None,
        }
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_owned());
        self
    }

    pub fn symbols(mut self, symbols: &[&str]) -> Self {
        self.symbols = Some(symbols.iter().map(|s| s.to_string()).collect());
        self
    }

    /// Defaults to 1d if no parameter provided.
    ///
    /// Supported windowSize values:
    ///
    /// - 1m,2m....59m for minutes
    /// - 1h, 2h....23h - for hours
    /// - 1d...7d - for days
    ///
    /// Units cannot be combined (e.g. 1d2h is not allowed).
    pub fn window_size(mut self, window_size: &str) -> Self {
        self.window_size = Some(window_size.to_owned());
        self
    }

    /// If none provided, the default is FULL.
    pub fn r#type(mut self, r#type: TickerType) -> Self {
        self.r#type = Some(r#type);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RollingWindowPriceChangeResponse {
    Full(Box<TickerFull>),
    FullVec(Box<Vec<TickerFull>>),
    Mini(Box<TickerMini>),
    MiniVec(Box<Vec<TickerMini>>),
}
