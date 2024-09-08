pub mod borrow_and_repay;
pub mod market;

use crate::rest_api::RestApiClient;

pub struct RestApiCategory<'r> {
    client: &'r RestApiClient,
}

impl<'r> RestApiCategory<'r> {
    pub fn new(client: &'r RestApiClient) -> Self {
        RestApiCategory { client }
    }

    pub fn borrow_and_repay(&self) -> borrow_and_repay::RestApiHandler {
        borrow_and_repay::RestApiHandler::new(self.client)
    }

    pub fn market(&self) -> market::RestApiHandler {
        market::RestApiHandler::new(self.client)
    }
}
