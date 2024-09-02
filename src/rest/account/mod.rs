//! Account endpoints
mod account_information;

pub use account_information::*;

use super::{route, RestClient};

pub struct Handler<'r> {
    client: &'r RestClient,
}

impl<'r> Handler<'r> {
    pub fn new(client: &'r RestClient) -> Self {
        Handler { client }
    }

    route!(account_information, AccountInformationEndpoint);
}
