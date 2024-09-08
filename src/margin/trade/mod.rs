mod cancel_all_open_orders;
mod cancel_oco;
mod cancel_order;
mod get_force_liquidation_record;
mod get_small_liability_exchange_coin_list;
mod get_small_liability_exchange_history;
mod margin_manual_liquidation;
mod new_oco;
mod new_order;
mod query_all_oco;
mod query_all_orders;
mod query_current_margin_order_count_usage;
mod query_oco;
mod query_open_oco;
mod query_open_orders;
mod query_order;
mod query_trade_list;
mod small_liability_exchange;

pub use cancel_all_open_orders::*;
pub use cancel_oco::*;
pub use cancel_order::*;
pub use get_force_liquidation_record::*;
pub use get_small_liability_exchange_coin_list::*;
pub use get_small_liability_exchange_history::*;
pub use margin_manual_liquidation::*;
pub use new_oco::*;
pub use new_order::*;
pub use query_all_oco::*;
pub use query_all_orders::*;
pub use query_current_margin_order_count_usage::*;
pub use query_oco::*;
pub use query_open_oco::*;
pub use query_open_orders::*;
pub use query_order::*;
pub use query_trade_list::*;
pub use small_liability_exchange::*;

use crate::rest_api::{route, RestApiClient};

pub struct RestApiHandler<'r> {
    client: &'r RestApiClient,
}

impl<'r> RestApiHandler<'r> {
    pub fn new(client: &'r RestApiClient) -> Self {
        RestApiHandler { client }
    }

    route!(
        get_force_liquidation_record,
        GetForceLiquidationRecordEndpoint
    );
    route!(
        get_small_liability_exchange_coin_list,
        GetSmallLiabilityExchangeCoinListEndpoint
    );
    route!(
        get_small_liability_exchange_history,
        GetSmallLiabilityExchangeHistoryEndpoint
    );
    route!(cancel_all_open_orders, CancelAllOpenOrdersEndpoint);
    route!(cancel_oco, CancelOcoEndpoint);
    route!(cancel_order, CancelOrderEndpoint);
    route!(new_oco, NewOcoEndpoint);
    route!(new_order, NewOrderEndpoint);
    route!(
        query_current_margin_order_count_usage,
        QueryCurrentMarginOrderCountUsageEndpoint
    );
    route!(query_all_oco, QueryAllOcoEndpoint);
    route!(query_all_orders, QueryAllOrdersEndpoint);
    route!(query_oco, QueryOcoEndpoint);
    route!(query_open_oco, QueryOpenOcoEndpoint);
    route!(query_open_orders, QueryOpenOrdersEndpoint);
    route!(query_order, QueryOrderEndpoint);
    route!(query_trade_list, QueryTradeListEndpoint);
    route!(small_liability_exchange, SmallLiabilityExchangeEndpoint);
    route!(margin_manual_liquidation, MarginManualLiquidationEndpoint);
}
