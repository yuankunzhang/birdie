pub mod market;

use crate::rest_api::RestApiClient;

pub struct RestApiCategory<'r> {
    client: &'r RestApiClient,
}

impl<'r> RestApiCategory<'r> {
    pub fn new(client: &'r RestApiClient) -> Self {
        RestApiCategory { client }
    }

    pub fn market(&self) -> market::RestApiHandler {
        market::RestApiHandler::new(self.client)
    }
}
