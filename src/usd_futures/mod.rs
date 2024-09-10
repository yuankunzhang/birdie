// pub mod account;
pub mod convert;
pub mod market;
pub mod trade;

use crate::rest_api::RestApiClient;

pub struct RestApiCategory<'r> {
    client: &'r RestApiClient,
}

impl<'r> RestApiCategory<'r> {
    pub fn new(client: &'r RestApiClient) -> Self {
        RestApiCategory { client }
    }

    // pub fn account(&self) -> account::RestApiHandler {
    //     account::RestApiHandler::new(self.client)
    // }

    pub fn convert(&self) -> convert::RestApiHandler {
        convert::RestApiHandler::new(self.client)
    }

    pub fn market(&self) -> market::RestApiHandler {
        market::RestApiHandler::new(self.client)
    }

    pub fn trade(&self) -> trade::RestApiHandler {
        trade::RestApiHandler::new(self.client)
    }
}
