//! Account endpoints
mod account_information;
mod account_trade_list;

pub use account_information::*;
pub use account_trade_list::*;

use super::{route, RestClient};

pub struct Handler<'r> {
    client: &'r RestClient,
}

impl<'r> Handler<'r> {
    pub fn new(client: &'r RestClient) -> Self {
        Handler { client }
    }

    route!(account_information, AccountInformationEndpoint);
    route!(account_trade_list, AccountTradeListEndpoint);
}
