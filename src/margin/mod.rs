//! # Binance's Margin Trading REST API.
//!
//! - [`account`] Account endpoints.
//! - [`borrow_and_repay`] Borrow and repay endpoints.
//! - [`market`] Market endpoints.
//! - [`risk_data_stream`] Risk data stream endpoints.
//! - [`trade`] Trade endpoints.
//! - [`trade_data_stream`] Trade data stream endpoints.
//! - [`transfer`] Transfer endpoints.
//!
pub mod account;
pub mod borrow_and_repay;
pub mod market;
pub mod risk_data_stream;
pub mod trade;
pub mod trade_data_stream;
pub mod transfer;

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

    pub fn borrow_and_repay(&self) -> borrow_and_repay::RestApiHandler {
        borrow_and_repay::RestApiHandler::new(self.client)
    }

    pub fn market(&self) -> market::RestApiHandler {
        market::RestApiHandler::new(self.client)
    }

    pub fn risk_data_stream(&self) -> risk_data_stream::RestApiHandler {
        risk_data_stream::RestApiHandler::new(self.client)
    }

    pub fn trade(&self) -> trade::RestApiHandler {
        trade::RestApiHandler::new(self.client)
    }

    pub fn trade_data_stream(&self) -> trade_data_stream::RestApiHandler {
        trade_data_stream::RestApiHandler::new(self.client)
    }

    pub fn transfer(&self) -> transfer::RestApiHandler {
        transfer::RestApiHandler::new(self.client)
    }
}
