//! Trade endpoints.
mod all_orders;
mod cancel_all_open_orders;
mod cancel_order;
mod cancel_replace_order;
mod current_open_orders;
mod new_order;
mod new_order_list_oco;
mod new_order_list_oto;
mod new_order_list_otoco;
mod query_order;
mod test_new_order;

pub use all_orders::*;
pub use cancel_all_open_orders::*;
pub use cancel_order::*;
pub use cancel_replace_order::*;
pub use current_open_orders::*;
pub use new_order::*;
pub use new_order_list_oco::*;
pub use new_order_list_oto::*;
pub use new_order_list_otoco::*;
pub use query_order::*;
pub use test_new_order::*;

use super::rest_api::{route, RestApiClient};

pub struct RestApiHandler<'r> {
    client: &'r RestApiClient,
}

impl<'r> RestApiHandler<'r> {
    pub fn new(client: &'r RestApiClient) -> Self {
        RestApiHandler { client }
    }

    route!(new_order, NewOrderEndpoint);
    route!(test_new_order, TestNewOrderEndpoint);
    route!(query_order, QueryOrderEndpoint);
    route!(cancel_order, CancelOrderEndpoint);
    route!(cancel_all_open_orders, CancelAllOpenOrdersEndpoint);
    route!(cancel_replace_order, CancelReplaceOrderEndpoint);
    route!(all_orders, AllOrdersEndpoint);
    route!(current_open_orders, CurrentOpenOrdersEndpoint);
}
