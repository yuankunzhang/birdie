mod close_user_data_stream;
mod keepalive_user_data_stream;
mod start_user_data_stream;

pub use close_user_data_stream::*;
pub use keepalive_user_data_stream::*;
pub use start_user_data_stream::*;

use crate::rest_api::{route, RestApiClient};

pub struct RestApiHandler<'r> {
    client: &'r RestApiClient,
}

impl<'r> RestApiHandler<'r> {
    pub fn new(client: &'r RestApiClient) -> Self {
        RestApiHandler { client }
    }

    route!(start_user_data_stream, StartUserDataStreamEndpoint);
    route!(keepalive_user_data_stream, KeepaliveUserDataStreamEndpoint);
    route!(close_user_data_stream, CloseUserDataStreamEndpoint);
}
