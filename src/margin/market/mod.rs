mod cross_margin_collateral_ratio;
mod get_all_cross_margin_pairs;
mod get_all_isolated_margin_symbol;
mod get_all_margin_assets;
mod get_delist_schedule;
mod query_isolated_margin_tier_data;
mod query_liability_coin_leverage_bracket;
mod query_margin_available_inventory;
mod query_margin_price_index;

pub use cross_margin_collateral_ratio::*;
pub use get_all_cross_margin_pairs::*;
pub use get_all_isolated_margin_symbol::*;
pub use get_all_margin_assets::*;
pub use get_delist_schedule::*;
pub use query_isolated_margin_tier_data::*;
pub use query_liability_coin_leverage_bracket::*;
pub use query_margin_available_inventory::*;
pub use query_margin_price_index::*;

use crate::rest_api::{route, RestApiClient};

pub struct RestApiHandler<'r> {
    client: &'r RestApiClient,
}

impl<'r> RestApiHandler<'r> {
    pub fn new(client: &'r RestApiClient) -> Self {
        RestApiHandler { client }
    }

    route!(
        cross_margin_collateral_ratio,
        CrossMarginCollateralRatioEndpoint
    );
    route!(get_all_cross_margin_pairs, GetAllCrossMarginPairsEndpoint);
    route!(
        get_all_isolated_margin_symbol,
        GetAllIsolatedMarginSymbolEndpoint
    );
    route!(get_all_margin_assets, GetAllMarginAssetsEndpoint);
    route!(get_delist_schedule, GetDelistScheduleEndpoint);
    route!(
        query_isolated_margin_tier_data,
        QueryIsolatedMarginTierDataEndpoint
    );
    route!(query_margin_price_index, QueryMarginPriceIndexEndpoint);
    route!(
        query_margin_available_inventory,
        QueryMarginAvailableInventoryEndpoint
    );
    route!(
        query_liability_coin_leverage_bracket,
        QueryLiabilityCoinLeverageBracketEndpoint
    );
}
