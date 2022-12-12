use reqwest::Client;

pub trait HttpClient {
    fn client(&self) -> &Client;
}
