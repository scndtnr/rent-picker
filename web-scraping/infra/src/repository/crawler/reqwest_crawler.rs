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
            client: Client::new(),
        }
    }

    pub(super) fn client(&self) -> &Client {
        &self.client
    }
}
