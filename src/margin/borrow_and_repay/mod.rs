mod get_future_hourly_interest_rate;
mod get_interest_history;
mod margin_account_borrow_repay;
mod query_borrow_repay_records;
mod query_margin_interest_rate_history;
mod query_max_borrow;

pub use get_future_hourly_interest_rate::*;
pub use get_interest_history::*;
pub use margin_account_borrow_repay::*;
pub use query_borrow_repay_records::*;
pub use query_margin_interest_rate_history::*;
pub use query_max_borrow::*;

use crate::rest_api::{route, RestApiClient};

pub struct RestApiHandler<'r> {
    client: &'r RestApiClient,
}

impl<'r> RestApiHandler<'r> {
    pub fn new(client: &'r RestApiClient) -> Self {
        RestApiHandler { client }
    }

    route!(
        get_future_hourly_interest_rate,
        GetFutureHourlyInterestRateEndpoint
    );
    route!(get_interest_history, GetInterestHistoryEndpoint);
    route!(
        margin_account_borrow_repay,
        MarginAccountBorrowRepayEndpoint
    );
    route!(query_borrow_repay_records, QueryBorrowRepayRecordsEndpoint);
    route!(
        query_margin_interest_rate_history,
        QueryMarginInterestRateHistoryEndpoint
    );
    route!(query_max_borrow, QueryMaxBorrowEndpoint);
}
