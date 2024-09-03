use super::{route, RestClient};

mod cancel_all_open_orders;
mod cancel_order;
mod new_order;
mod query_order;
mod test_new_order;

pub use cancel_all_open_orders::*;
pub use cancel_order::*;
pub use new_order::*;
pub use query_order::*;
pub use test_new_order::*;

pub struct Handler<'r> {
    client: &'r RestClient,
}

impl<'r> Handler<'r> {
    pub fn new(client: &'r RestClient) -> Self {
        Handler { client }
    }

    route!(new_order, NewOrderEndpoint);
    route!(test_new_order, TestNewOrderEndpoint);
    route!(query_order, QueryOrderEndpoint);
    route!(cancel_order, CancelOrderEndpoint);
    route!(cancel_all_open_orders, CancelAllOpenOrdersEndpoint);
}
