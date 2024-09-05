//! Account endpoints
//!
//! - [`account_information`](AccountInformationEndpoint)
//! - [`account_trade_list`](AccountTradeListEndpoint)
//! - [`query_unfilled_order_count`](QueryUnfilledOrderCountEndpoint)
//! - [`query_prevented_matches`](QueryPreventedMatchesEndpoint)
//! - [`query_allocations`](QueryAllocationsEndpoint)
//! - [`query_commission_rates`](QueryCommissionRatesEndpoint)
mod account_information;
mod account_trade_list;
mod query_allocations;
mod query_commission_rates;
mod query_prevented_matches;
mod query_unfilled_order_count;

pub use account_information::*;
pub use account_trade_list::*;
pub use query_allocations::*;
pub use query_commission_rates::*;
pub use query_prevented_matches::*;
pub use query_unfilled_order_count::*;

use crate::rest_api::{route, RestApiClient};

pub struct RestApiHandler<'r> {
    client: &'r RestApiClient,
}

impl<'r> RestApiHandler<'r> {
    pub fn new(client: &'r RestApiClient) -> Self {
        RestApiHandler { client }
    }

    route!(account_information, AccountInformationEndpoint);
    route!(account_trade_list, AccountTradeListEndpoint);
    route!(query_unfilled_order_count, QueryUnfilledOrderCountEndpoint);
    route!(query_prevented_matches, QueryPreventedMatchesEndpoint);
    route!(query_allocations, QueryAllocationsEndpoint);
    route!(query_commission_rates, QueryCommissionRatesEndpoint);
}
