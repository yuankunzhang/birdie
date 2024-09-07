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
//! - [`query_order_lists`](QueryOrderListsEndpoint)
//! - [`query_all_order_lists`](QueryAllOrderListsEndpoint)
//! - [`query_open_order_lists`](QueryOpenOrderListsEndpoint)
//! - [`new_order_using_sor`](NewOrderUsingSorEndpoint)
//! - [`test_new_order_using_sor`](TestNewOrderUsingSorEndpoint)
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
mod new_order_using_sor;
mod query_all_order_lists;
mod query_open_order_lists;
mod query_order;
mod query_order_lists;
mod test_new_order;
mod test_new_order_using_sor;

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
pub use new_order_using_sor::*;
pub use query_all_order_lists::*;
pub use query_open_order_lists::*;
pub use query_order::*;
pub use query_order_lists::*;
pub use test_new_order::*;
pub use test_new_order_using_sor::*;

use crate::{
    rest_api::{route, RestApiClient},
    web_socket_api::{ws_route, WebSocketApiClient},
};

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
    route!(query_order_lists, QueryOrderListsEndpoint);
    route!(query_all_order_lists, QueryAllOrderListsEndpoint);
    route!(query_open_order_lists, QueryOpenOrderListsEndpoint);
    route!(new_order_using_sor, NewOrderUsingSorEndpoint);
    route!(test_new_order_using_sor, TestNewOrderUsingSorEndpoint);
}

pub struct WebSocketApiHandler<'w> {
    client: &'w WebSocketApiClient,
}

impl<'w> WebSocketApiHandler<'w> {
    pub fn new(client: &'w WebSocketApiClient) -> Self {
        WebSocketApiHandler { client }
    }

    ws_route!(new_order, NewOrderWebSocket);
    ws_route!(test_new_order, TestNewOrderWebSocket);
    ws_route!(query_order, QueryOrderWebSocket);
    ws_route!(cancel_order, CancelOrderWebSocket);
    ws_route!(cancel_all_open_orders, CancelAllOpenOrdersWebSocket);
    ws_route!(cancel_replace_order, CancelReplaceOrderWebSocket);
    ws_route!(current_open_orders, CurrentOpenOrdersWebSocket);
    // ws_route!(all_orders, AllOrdersWebSocket);
    ws_route!(new_order_list_oco, NewOrderListOcoWebSocket);
    ws_route!(new_order_list_oto, NewOrderListOtoWebSocket);
    ws_route!(new_order_list_otoco, NewOrderListOtocoWebSocket);
    ws_route!(cancel_order_list, CancelOrderListWebSocket);
    ws_route!(query_order_lists, QueryOrderListsWebSocket);
    // ws_route!(query_all_order_lists, QueryAllOrderListsWebSocket);
    ws_route!(query_open_order_lists, QueryOpenOrderListsWebSocket);
    ws_route!(new_order_using_sor, NewOrderUsingSorWebSocket);
    ws_route!(test_new_order_using_sor, TestNewOrderUsingSorWebSocket);
}
