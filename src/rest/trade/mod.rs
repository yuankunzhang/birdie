use super::{route, RestClient};

mod new_order;
mod test_new_order;

pub use new_order::*;
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
}
