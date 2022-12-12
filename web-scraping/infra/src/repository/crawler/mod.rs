mod html_parser;
mod http_client;
mod reqwest_crawler;

pub use html_parser::HtmlParser;
pub use http_client::HttpClient;
pub use reqwest_crawler::ReqwestCrawler;

impl HtmlParser for ReqwestCrawler {}
impl HttpClient for ReqwestCrawler {
    fn client(&self) -> &reqwest::Client {
        self.client()
    }
}
