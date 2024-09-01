mod ping;
mod time;

pub use ping::*;
pub use time::*;

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

    pub fn time(&self) -> TimeEndpoint {
        TimeEndpoint::new(self.client)
    }
}
