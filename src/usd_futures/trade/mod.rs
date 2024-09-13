mod account_trade_list;
mod auto_cancel_all_open_orders;
mod cancel_all_open_orders;
mod cancel_multiple_orders;
mod cancel_order;
mod change_initial_leverage;
mod change_margin_type;
mod change_multi_assets_mode;
mod change_position_mode;
mod get_order_modify_history;
mod get_position_margin_change_history;
mod modify_isolated_position_margin;
mod modify_multiple_orders;
mod modify_order;
mod new_order;
mod place_multiple_orders;
mod position_adl_quantile_estimation;
mod position_informatin_v2;
mod position_informatin_v3;
mod query_all_orders;
mod query_current_all_open_orders;
mod query_current_open_order;
mod query_force_orders;
mod query_order;
mod test_new_order;

pub use account_trade_list::*;
pub use auto_cancel_all_open_orders::*;
pub use cancel_all_open_orders::*;
pub use cancel_multiple_orders::*;
pub use cancel_order::*;
pub use change_initial_leverage::*;
pub use change_margin_type::*;
pub use change_multi_assets_mode::*;
pub use change_position_mode::*;
pub use get_order_modify_history::*;
pub use get_position_margin_change_history::*;
pub use modify_isolated_position_margin::*;
pub use modify_multiple_orders::*;
pub use modify_order::*;
pub use new_order::*;
pub use place_multiple_orders::*;
pub use position_adl_quantile_estimation::*;
pub use position_informatin_v2::*;
pub use position_informatin_v3::*;
pub use query_all_orders::*;
pub use query_current_all_open_orders::*;
pub use query_current_open_order::*;
pub use query_force_orders::*;
pub use query_order::*;
pub use test_new_order::*;

use crate::rest_api::{route, RestApiClient};

pub struct RestApiHandler<'r> {
    client: &'r RestApiClient,
}

impl<'r> RestApiHandler<'r> {
    pub fn new(client: &'r RestApiClient) -> Self {
        RestApiHandler { client }
    }

    route!(new_order, NewOrderEndpoint);
    route!(place_multiple_orders, PlaceMultipleOrdersEndpoint);
    route!(modify_order, ModifyOrderEndpoint);
    route!(modify_multiple_orders, ModifyMultipleOrdersEndpoint);
    route!(get_order_modify_history, GetOrderModifyHistoryEndpoint);
    route!(cancel_order, CancelOrderEndpoint);
    route!(cancel_multiple_orders, CancelMultipleOrdersEndpoint);
    route!(cancel_all_open_orders, CancelAllOpenOrdersEndpoint);
    route!(auto_cancel_all_open_orders, AutoCancelAllOpenOrdersEndpoint);
    route!(query_order, QueryOrderEndpoint);
    route!(query_all_orders, QueryAllOrdersEndpoint);
    route!(
        query_current_all_open_orders,
        QueryCurrentAllOpenOrdersEndpoint
    );
    route!(query_current_open_orders, QueryCurrentOpenOrderEndpoint);
    route!(query_users_force_orders, QueryForceOrdersEndpoint);
    route!(account_trade_list, AccountTradeListEndpoint);
    route!(change_margin_type, ChangeMarginTypeEndpoint);
    route!(change_position_mode, ChangePositionModeEndpoint);
    route!(change_initial_leverage, ChangeInitialLeverageEndpoint);
    route!(change_multi_assets_mode, ChangeMultiAssetsModeEndpoint);
    route!(
        modify_isolated_position_margin,
        ModifyIsolatedPositionMarginEndpoint
    );
    route!(position_informatin_v2, PositionInformationV2Endpoint);
    route!(position_informatin_v3, PositionInformationV3Endpoint);
    route!(
        position_adl_quantile_estimation,
        PositionAdlQuantileEstimationEndpoint
    );
    route!(
        get_position_margin_change_history,
        GetPositionMarginChangeHistoryEndpoint
    );
    route!(test_new_order, TestNewOrderEndpoint);
}
