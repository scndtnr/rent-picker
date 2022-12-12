use std::fmt::Debug;

use anyhow::{bail, Context, Result};
use reqwest::{Client, Response};
use scraper::{ElementRef, Html};

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

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn innter_text_of_element(html: &str, css_selector: &str, sep: &str) -> String {
        let html = Self::parse_html(html);
        let element = Self::find_element(&html, css_selector).unwrap();
        Self::inner_text(&element, sep)
    }

    /// 返り値のHtmlはthreads safeではないため、
    /// async内で利用する場合は{}で囲ってスコープを変える必要がある
    pub fn parse_html(html: &str) -> Html {
        Html::parse_document(html)
    }

    /// 返り値のElementRefはthreads safeではないため、
    /// async内で利用する場合は{}で囲ってスコープを変える必要がある
    pub fn find_elements<'a, 'b>(html: &'a Html, css_selector: &'b str) -> Vec<ElementRef<'a>> {
        let selector = scraper::Selector::parse(css_selector).unwrap();
        html.select(&selector).collect::<Vec<ElementRef>>()
    }

    /// 返り値のElementRefはthreads safeではないため、
    /// async内で利用する場合は{}で囲ってスコープを変える必要がある
    pub fn find_element<'a, 'b>(html: &'a Html, css_selector: &'b str) -> Result<ElementRef<'a>> {
        let elems = Self::find_elements(html, css_selector);
        if elems.is_empty() {
            bail!("Element is not found. selector: {}", css_selector);
        } else if elems.len() == 1 {
            Ok(elems[0])
        } else {
            bail!("Multiple elements are found. selector: {}", css_selector);
        }
    }

    pub fn inner_text(element: &ElementRef, sep: &str) -> String {
        element.text().collect::<Vec<&str>>().join(sep)
    }
}
