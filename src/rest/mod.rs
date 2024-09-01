//! Types and traits for working with Binance's REST API.
pub mod auto_invest;
pub mod blvt;
pub mod c2c;
pub mod convert;
pub mod crypto_loans;
pub mod fiat;
pub mod futures;
pub mod futures_algo;
pub mod gift_card;
pub mod isolated_margin_stream;
pub mod margin;
pub mod margin_stream;
pub mod market;
pub mod mining;
pub mod nft;
pub mod pay;
pub mod portfolio_margin;
pub mod rebase;
pub mod savings;
pub mod simple_earn;
pub mod spot_algo;
pub mod stream;
pub mod sub_account;
pub mod trade;
pub mod vip_loans;
pub mod wallet;

use bytes::Bytes;
use reqwest::{header::HeaderMap, Method, StatusCode};
use thiserror::Error;

use crate::errors::BinanceError;

#[derive(Debug, Error)]
pub enum RestError {
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("binance error: {0}")]
    Binance(String, Option<BinanceError>),
}

pub trait Params {
    fn as_query(&self) -> &str;
}

pub trait Response {
    type Body;

    fn headers(&self) -> &HeaderMap;
    fn status(&self) -> StatusCode;
    fn body(&self) -> Result<&Bytes, RestError>;
    fn json(&self) -> Result<Self::Body, RestError>;
}

#[async_trait::async_trait]
pub trait Endpoint {
    type Body;
    type Response;

    fn path(&self) -> &str;
    fn method(&self) -> Method;
    async fn request(&self) -> Result<Self::Body, RestError>;
    async fn request2(&self) -> Result<Self::Response, RestError>;
}
