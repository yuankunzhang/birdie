pub mod account;
pub mod general;
pub mod market;
pub mod trade;
pub mod user_data_stream;
pub mod web_socket_streams;

use crate::rest_api::RestApiClient;

pub struct RestApiCategory<'r> {
    client: &'r RestApiClient,
}

impl<'r> RestApiCategory<'r> {
    pub fn new(client: &'r RestApiClient) -> Self {
        RestApiCategory { client }
    }

    pub fn account(&self) -> account::RestApiHandler {
        account::RestApiHandler::new(self.client)
    }

    pub fn general(&self) -> general::RestApiHandler {
        general::RestApiHandler::new(self.client)
    }

    pub fn market(&self) -> market::RestApiHandler {
        market::RestApiHandler::new(self.client)
    }

    pub fn trade(&self) -> trade::RestApiHandler {
        trade::RestApiHandler::new(self.client)
    }

    pub fn user_data_stream(&self) -> user_data_stream::RestApiHandler {
        user_data_stream::RestApiHandler::new(self.client)
    }
}
