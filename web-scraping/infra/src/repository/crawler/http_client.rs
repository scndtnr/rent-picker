use reqwest::Client;

#[async_trait::async_trait]
pub trait HttpClient {
    fn client(&self) -> &Client;
    async fn sleep_by_secs(&self, secs: u64) {
        tokio::time::sleep(tokio::time::Duration::from_secs(secs)).await
    }
}
