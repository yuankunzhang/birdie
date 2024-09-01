pub mod enums;
pub mod errors;
pub mod filters;
pub mod models;
pub mod rest;

use rest::{RestClient, RestError};
use serde::Serializer;

pub struct Birdie {
    rest: RestClient,
}

impl Birdie {
    pub fn new(base_url: &str, api_key: &str, secret_key: &str) -> Result<Self, RestError> {
        let rest = RestClient::new(base_url, api_key, secret_key)?;
        Ok(Self { rest })
    }

    pub fn rest(&self) -> &RestClient {
        &self.rest
    }
}

pub fn serde_query_option_vec<S, T, V>(v: V, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: ToString,
    V: AsRef<Option<Vec<T>>>,
{
    match v.as_ref() {
        Some(v) => {
            let arr: Vec<_> = v.iter().map(|x| x.to_string()).collect();
            let str = format!("[{}]", arr.join(","));
            s.serialize_str(&str)
        }
        None => s.serialize_none(),
    }
}
