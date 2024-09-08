mod get_cross_margin_transfer_history;
mod query_max_transfer_out_amount;

pub use get_cross_margin_transfer_history::*;
pub use query_max_transfer_out_amount::*;

use crate::rest_api::{route, RestApiClient};

pub struct RestApiHandler<'r> {
    client: &'r RestApiClient,
}

impl<'r> RestApiHandler<'r> {
    pub fn new(client: &'r RestApiClient) -> Self {
        RestApiHandler { client }
    }

    route!(
        get_cross_margin_transfer_history,
        GetCrossMarginTransferHistoryEndpoint
    );
    route!(
        query_max_transfer_out_amount,
        QueryMaxTransferOutAmountEndpoint
    );
}
