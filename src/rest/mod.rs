//! Binance's REST API.
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

use reqwest::{Client, Method};
use serde::{Deserialize, Serialize, Serializer};
use thiserror::Error;
use url::Url;

use crate::errors::BinanceError;

pub struct RestClient {
    pub(self) client: Client,
    pub(self) base_url: Url,
    api_key: String,
    secret_key: String,
}

impl RestClient {
    pub fn new(base_url: &str, api_key: &str, secret_key: &str) -> Result<Self, RestError> {
        let base_url = Url::parse(base_url)?;
        Ok(Self {
            client: Client::new(),
            base_url,
            api_key: api_key.to_string(),
            secret_key: secret_key.to_string(),
        })
    }

    pub fn market(&self) -> market::Handler {
        market::Handler::new(self)
    }

    pub(self) async fn request<P, R>(
        &self,
        method: Method,
        endpoint: &str,
        params: P,
    ) -> Result<R, RestError>
    where
        P: Params,
        R: Response,
    {
        let mut url = self.base_url.join(endpoint)?;
        url.set_query(Some(&params.as_query()?));

        let res = self.client.request(method, url).send().await?;
        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            let status = res.status().to_string();
            let error = res.json::<BinanceError>().await.ok();
            Err(RestError::Binance(status, error))
        }
    }
}

#[derive(Debug, Error)]
pub enum RestError {
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("url parse error: {0}")]
    Url(#[from] url::ParseError),
    #[error("json parse error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("query string parse error: {0}")]
    QueryString(#[from] serde_qs::Error),
    #[error("binance error: {0}")]
    Binance(String, Option<BinanceError>),
}

/// Query parameters for a REST API endpoint.
pub trait Params: Sized + Send + Serialize {
    fn as_query(&self) -> Result<String, RestError> {
        Ok(serde_qs::to_string(self)?)
    }
}

/// Response data for a REST API endpoint.
pub trait Response: Sized + for<'de> Deserialize<'de> {}

#[async_trait::async_trait]
pub trait Endpoint {
    type Params: Params;
    type Response: Response;

    fn client(&self) -> &RestClient;
    fn path(&self) -> &str;
    fn method(&self) -> Method;

    async fn request(&self, params: Self::Params) -> Result<Self::Response, RestError> {
        self.client()
            .request(Method::GET, self.path(), params)
            .await
    }
}

macro_rules! endpoint {
    ($path:literal, $method:expr, $name:ident, $params:ty, $response:ty) => {
        impl crate::rest::Params for $params {}
        impl crate::rest::Response for $response {}

        #[async_trait::async_trait]
        impl crate::rest::Endpoint for $name<'_> {
            type Params = $params;
            type Response = $response;

            fn client(&self) -> &crate::rest::RestClient {
                self.client
            }

            fn path(&self) -> &str {
                $path
            }

            fn method(&self) -> reqwest::Method {
                $method
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

use endpoint;
use route;

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
