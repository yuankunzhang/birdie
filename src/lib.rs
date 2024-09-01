pub mod enums;
pub mod errors;
pub mod filters;
pub mod models;
pub mod rest;

use rest::{RestClient, RestError};

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
