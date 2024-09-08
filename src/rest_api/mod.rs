//! # Binance's REST API.
//!
//! - Spot trading API
//!   - [`account`] Account endpoints.
//!   - [`general`] General endpoints.
//!   - [`market`] Market endpoints.
//!   - [`trade`] Trade endpoints.
//!   - [`user_data_stream`] User data stream endpoints.
//! - [Margin trading API](`mod@margin`)
//!
//! ## How to Use
//!
//! To use the REST API client, create a instance of the client first:
//!
//! ```rust
//! let base_url = "https://api.binance.com";
//! let api_key = "your_api_key";
//! let api_secret = "your_api_secret";
//! let client = birdie::rest_api(base_url, api_key, api_secret).unwrap();
//! ```
//!
//! Once you have the client, the calling of a request is done with the
//! following hierarchical pattern:
//!
//! ```txt
//! let resp = client.general().test_connectivity().request(params).await
//!     ----   ------ --------- ------------------- ------- ------
//!     |      |      |         |                   |       |
//!     |      |      |         |                   |       \
//!     |      |      |         |                   \        The request params
//!     |      |      |         \                     The request
//!     |      |      \           The endpoint
//!     |      \        The endpoint category
//!     \        The REST API client
//!      The response
//! ```
//!
//! There are three important types for each endpoint: [`Endpoint`],
//! [`Params`], and [`Response`].
//!
//! - [`Endpoint`] represents an API endpoint.
//! - [`Params`] represents the request parameters that the endpoint accepts.
//! - [`Response`] represents the response that the endpoint returns.
//!
//! To make a request, you need to first create an instance of the [`Params`]
//! type, and then call the [`Endpoint::request`] method on the [`Endpoint`]
//! instance as shown in the pattern above. The [`Endpoint::request`] method
//! returns a [`Response`] instance or a [`RestApiError`] if an error occurs.
//!
//! For example, here is the code to retreive the order book of BTC:
//!
//! ```no_run
//! use birdie::{spot::market::OrderBookParams, Birdie};
//! use crate::birdie::rest_api::Endpoint;
//!
//! #[tokio::main]
//! async fn main() {
//!   let base_url = "https://api.binance.com";
//!   let api_key = "your_api_key";
//!   let api_secret = "your_api_secret";
//!   let client = birdie::rest_api(base_url, api_key, api_secret).unwrap();
//!
//!   let params = OrderBookParams::new("BTCUSDT").limit(10);
//!   let resp = client.market().order_book().request(params).await;
//!   assert!(resp.is_ok());
//! }
//! ```
//!
//! Now, try it yourself!

use reqwest::{Client, Method, RequestBuilder};
use serde::Serializer;
use thiserror::Error;
use tracing::debug;
use tracing::error;
use tracing::info;
use url::Url;

use crate::enums::SecurityType;
use crate::errors::BinanceError;
use crate::hmac_signature;

use crate::margin;
use crate::spot::account;
use crate::spot::general;
use crate::spot::market;
use crate::spot::trade;
use crate::spot::user_data_stream;
use crate::Params;
use crate::Response;

#[derive(Debug, Error)]
pub enum RestApiError {
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("url parse error: {0}")]
    Url(#[from] url::ParseError),
    #[error("json parse error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("query string parse error: {0}")]
    QueryString(#[from] serde_qs::Error),
    #[error("Hmac error: {0}")]
    Hmac(#[from] hmac::digest::InvalidLength),
    #[error("binance error: {0}")]
    Binance(String, Option<BinanceError>),
}

pub struct RestApiClient {
    pub(self) client: Client,
    pub(self) base_url: Url,
    api_key: String,
    secret_key: String,
}

impl RestApiClient {
    pub fn new(base_url: &str, api_key: &str, secret_key: &str) -> Result<Self, RestApiError> {
        let base_url = Url::parse(base_url)?;
        Ok(Self {
            client: Client::new(),
            base_url,
            api_key: api_key.to_string(),
            secret_key: secret_key.to_string(),
        })
    }

    pub fn account(&self) -> account::RestApiHandler {
        account::RestApiHandler::new(self)
    }

    pub fn general(&self) -> general::RestApiHandler {
        general::RestApiHandler::new(self)
    }

    pub fn market(&self) -> market::RestApiHandler {
        market::RestApiHandler::new(self)
    }

    pub fn trade(&self) -> trade::RestApiHandler {
        trade::RestApiHandler::new(self)
    }

    pub fn user_data_stream(&self) -> user_data_stream::RestApiHandler {
        user_data_stream::RestApiHandler::new(self)
    }

    pub fn margin(&self) -> margin::RestApiCategory {
        margin::RestApiCategory::new(self)
    }

    pub(self) async fn request<P, R>(
        &self,
        method: Method,
        endpoint: &str,
        params: P,
    ) -> Result<R, RestApiError>
    where
        P: Params,
        R: Response,
    {
        let mut url = self.base_url.join(endpoint)?;
        url.set_query(Some(&params.as_query()?));
        info!("send request to {url}");

        let req = self.client.request(method, url);
        Self::send_request(req).await
    }

    pub(self) async fn auth_request<P, R>(
        &self,
        method: Method,
        endpoint: &str,
        params: P,
    ) -> Result<R, RestApiError>
    where
        P: Params,
        R: Response,
    {
        let mut url = self.base_url.join(endpoint)?;
        url.set_query(Some(&params.as_query()?));
        info!("send auth request to {url}");

        let req = self
            .client
            .request(method, url)
            .header("X-MBX-APIKEY", &self.api_key);
        Self::send_request(req).await
    }

    pub(self) async fn signed_request<P, R>(
        &self,
        method: Method,
        endpoint: &str,
        params: P,
    ) -> Result<R, RestApiError>
    where
        P: Params,
        R: Response,
    {
        let mut url = self.base_url.join(endpoint)?;
        let query = params.as_query()?;
        let signature = hmac_signature(&self.secret_key, &query)?;
        let query = format!("{query}&signature={signature}");
        url.set_query(Some(&query));
        info!("send signed request to {url}");

        let req = self
            .client
            .request(method, url)
            .header("X-MBX-APIKEY", &self.api_key);
        Self::send_request(req).await
    }

    async fn send_request<R>(req: RequestBuilder) -> Result<R, RestApiError>
    where
        R: Response,
    {
        let res = req.send().await?;
        if res.status().is_success() {
            debug!("response: {res:?}");
            Ok(res.json().await?)
        } else {
            let status = res.status().to_string();
            let error = res.json::<BinanceError>().await.ok();
            error!("response: {status}, {error:?}");
            Err(RestApiError::Binance(status, error))
        }
    }
}

pub fn serialize_option_vec<S, T>(v: &Option<Vec<T>>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: ToString,
{
    match v.as_ref() {
        Some(v) => {
            let arr: Vec<_> = v.iter().map(|x| format!("\"{}\"", x.to_string())).collect();
            let str = format!("[{}]", arr.join(","));
            s.serialize_str(&str)
        }
        None => s.serialize_none(),
    }
}

#[async_trait::async_trait]
pub trait Endpoint {
    type Params: Params;
    type Response: Response;

    fn client(&self) -> &RestApiClient;
    fn path(&self) -> &str;
    fn method(&self) -> Method;
    fn security_type(&self) -> SecurityType;

    async fn request(&self, params: Self::Params) -> Result<Self::Response, RestApiError> {
        match self.security_type() {
            SecurityType::None => {
                self.client()
                    .request(self.method(), self.path(), params)
                    .await
            }
            SecurityType::UserStream => {
                self.client()
                    .auth_request(self.method(), self.path(), params)
                    .await
            }
            _ => {
                self.client()
                    .signed_request(self.method(), self.path(), params)
                    .await
            }
        }
    }
}

macro_rules! endpoint {
    ($path:literal, $method:expr, $name:ident, $params:ty, $response:ty) => {
        impl crate::Params for $params {}
        impl crate::Response for $response {}

        #[async_trait::async_trait]
        impl crate::rest_api::Endpoint for $name<'_> {
            type Params = $params;
            type Response = $response;

            fn client(&self) -> &crate::rest_api::RestApiClient {
                self.client
            }

            fn path(&self) -> &str {
                $path
            }

            fn method(&self) -> reqwest::Method {
                $method
            }

            fn security_type(&self) -> $crate::enums::SecurityType {
                $crate::enums::SecurityType::None
            }
        }
    };
    ($path:literal, $method:expr, $security:expr, $name:ident, $params:ty, $response:ty) => {
        impl crate::Params for $params {}
        impl crate::Response for $response {}

        #[async_trait::async_trait]
        impl crate::rest_api::Endpoint for $name<'_> {
            type Params = $params;
            type Response = $response;

            fn client(&self) -> &crate::rest_api::RestApiClient {
                self.client
            }

            fn path(&self) -> &str {
                $path
            }

            fn method(&self) -> reqwest::Method {
                $method
            }

            fn security_type(&self) -> $crate::enums::SecurityType {
                $security
            }
        }
    };
}

macro_rules! route {
    ($route:ident, $endpoint:ty) => {
        pub fn $route(&self) -> $endpoint {
            <$endpoint>::new(self.client)
        }
    };
}

pub(crate) use endpoint;
pub(crate) use route;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_vec() {
        #[derive(serde::Serialize)]
        struct Test {
            #[serde(serialize_with = "serialize_option_vec")]
            v: Option<Vec<String>>,
        }

        let t = Test { v: None };
        assert_eq!(serde_qs::to_string(&t).unwrap(), "");

        let t = Test {
            v: Some(vec!["a".to_string(), "b".to_string()]),
        };
        assert_eq!(
            serde_qs::to_string(&t).unwrap(),
            "v=%5B%22a%22%2C%22b%22%5D"
        );
    }
}
