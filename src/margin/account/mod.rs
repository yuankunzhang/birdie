mod adjust_cross_margin_max_leverage;
mod disable_isolated_margin_account;
mod enable_isolated_margin_account;
mod get_bnb_burn_status;
mod get_summary_of_margin_account;
mod query_cross_isolated_margin_capital_flow;
mod query_cross_margin_account_details;
mod query_cross_margin_fee_data;
mod query_enabled_isolated_margin_account_limit;
mod query_isolated_margin_account_info;
mod query_isolated_margin_fee_data;
mod toggle_bnb_burn_on_spot_trade_and_margin_interest;

pub use adjust_cross_margin_max_leverage::*;
pub use disable_isolated_margin_account::*;
pub use enable_isolated_margin_account::*;
pub use get_bnb_burn_status::*;
pub use get_summary_of_margin_account::*;
pub use query_cross_isolated_margin_capital_flow::*;
pub use query_cross_margin_account_details::*;
pub use query_cross_margin_fee_data::*;
pub use query_enabled_isolated_margin_account_limit::*;
pub use query_isolated_margin_account_info::*;
pub use query_isolated_margin_fee_data::*;
pub use toggle_bnb_burn_on_spot_trade_and_margin_interest::*;

use crate::rest_api::{route, RestApiClient};

pub struct RestApiHandler<'r> {
    client: &'r RestApiClient,
}

impl<'r> RestApiHandler<'r> {
    pub fn new(client: &'r RestApiClient) -> Self {
        RestApiHandler { client }
    }

    route!(
        adjust_cross_margin_max_leverage,
        AdjustCrossMarginMaxLeverageEndpoint
    );
    route!(
        disable_isolated_margin_account,
        DisableIsolatedMarginAccountEndpoint
    );
    route!(
        enable_isolated_margin_account,
        EnableIsolatedMarginAccountEndpoint
    );
    route!(get_bnb_burn_status, GetBnbBurnStatusEndpoint);
    route!(
        get_summary_of_margin_account,
        GetSummaryOfMarginAccountEndpoint
    );
    route!(
        query_cross_margin_account_details,
        QueryCrossMarginAccountDetailsEndpoint
    );
    route!(query_cross_margin_fee_data, QueryCrossMarginFeeDataEndpoint);
    route!(
        query_enabled_isolated_margin_account_limit,
        QueryEnabledIsolatedMarginAccountLimitEndpoint
    );
    route!(
        query_isolated_margin_account_info,
        QueryIsolatedMarginAccountInfoEndpoint
    );
    route!(
        query_isolated_margin_fee_data,
        QueryIsolatedMarginFeeDataEndpoint
    );
    route!(
        toggle_bnb_burn_on_spot_trade_and_margin_interest,
        ToggleBnbBurnOnSpotTradeAndMarginInterestEndpoint
    );
    route!(
        query_cross_isolated_margin_capital_flow,
        QueryCrossIsolatedMarginCapitalFlowEndpoint
    );
}
