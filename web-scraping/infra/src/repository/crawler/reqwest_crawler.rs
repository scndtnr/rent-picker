use reqwest::Client;

#[derive(Debug, Clone)]
pub struct ReqwestCrawler {
    client: Client,
}

impl Default for ReqwestCrawler {
    fn default() -> Self {
        Self::new()
    }
}

impl ReqwestCrawler {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                // [Error: IncompleteMessage: connection closed before message completed · Issue #2136 · hyperium/hyper](https://github.com/hyperium/hyper/issues/2136)
                .pool_max_idle_per_host(0)
                .build()
                .expect("Fail to build reqwest client."),
        }
    }

    pub(super) fn client(&self) -> &Client {
        &self.client
    }
}
