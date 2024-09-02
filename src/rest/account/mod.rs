//! Account endpoints
mod account_information;
mod account_trade_list;
mod query_prevented_matches;
mod query_unfilled_order_count;

pub use account_information::*;
pub use account_trade_list::*;
pub use query_prevented_matches::*;
pub use query_unfilled_order_count::*;

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
    route!(query_unfilled_order_count, QueryUnfilledOrderCountEndpoint);
    route!(query_prevented_matches, QueryPreventedMatchesEndpoint);
}
