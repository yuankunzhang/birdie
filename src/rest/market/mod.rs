mod ping;

pub use ping::*;

use super::RestClient;

pub struct Handler<'r> {
    client: &'r RestClient,
}

impl<'r> Handler<'r> {
    pub fn new(client: &'r RestClient) -> Self {
        Handler { client }
    }

    pub fn ping(&self) -> PingEndpoint {
        PingEndpoint::new(self.client)
    }
}
