mod accept_quote;
mod list_all_convert_pairs;
mod order_status;
mod send_quote_requets;

pub use accept_quote::*;
pub use list_all_convert_pairs::*;
pub use order_status::*;
pub use send_quote_requets::*;

use crate::rest_api::{route, RestApiClient};

pub struct RestApiHandler<'r> {
    client: &'r RestApiClient,
}

impl<'r> RestApiHandler<'r> {
    pub fn new(client: &'r RestApiClient) -> Self {
        RestApiHandler { client }
    }

    route!(list_all_convert_pairs, ListAllConvertPairsEndpoint);
    route!(send_quote_request, SendQuoteRequestEndpoint);
    route!(accept_quote, AcceptQuoteEndpoint);
    route!(order_status, OrderStatusEndpoint);
}
