//! Trade endpoints.
//!
//! - [`new_order`](NewOrderEndpoint)
//! - [`test_new_order`](TestNewOrderEndpoint)
//! - [`query_order`](QueryOrderEndpoint)
//! - [`cancel_order`](CancelOrderEndpoint)
//! - [`cancel_all_open_orders`](CancelAllOpenOrdersEndpoint)
//! - [`cancel_replace_order`](CancelReplaceOrderEndpoint)
//! - [`current_open_orders`](CurrentOpenOrdersEndpoint)
//! - [`all_orders`](AllOrdersEndpoint)
//! - [`current_open_orders`](CurrentOpenOrdersEndpoint)
//! - [`new_order_list_oco`](NewOrderListOcoEndpoint)
//! - [`new_order_list_oto`](NewOrderListOtoEndpoint)
//! - [`new_order_list_otoco`](NewOrderListOtocoEndpoint)
//! - [`cancel_order_list`](CancelOrderListEndpoint)
mod all_orders;
mod cancel_all_open_orders;
mod cancel_order;
mod cancel_order_list;
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
pub use cancel_order_list::*;
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
    route!(current_open_orders, CurrentOpenOrdersEndpoint);
    route!(all_orders, AllOrdersEndpoint);
    route!(new_order_list_oco, NewOrderListOcoEndpoint);
    route!(new_order_list_oto, NewOrderListOtoEndpoint);
    route!(new_order_list_otoco, NewOrderListOtocoEndpoint);
    route!(cancel_order_list, CancelOrderListEndpoint);
}
