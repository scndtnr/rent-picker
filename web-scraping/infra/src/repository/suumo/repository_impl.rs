use anyhow::{bail, Result};
use domain::repository::SuumoRepository;
use futures::{stream, StreamExt, TryStreamExt};

use crate::{
    env::get_env_var,
    repository::{ChromiumoxideCrawler, Crawler},
};

use super::SuumoSelector;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SuumoRepositoryImpl;

impl SuumoRepositoryImpl {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SuumoRepositoryImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl SuumoRepository for SuumoRepositoryImpl {
    type Crawler = Crawler;
    type Selector = SuumoSelector;

    /// クローラ生成（Browser生成タイミングを制御するため）
    async fn new_crawler(&self) -> Self::Crawler {
        Self::Crawler::new().await
    }

    /// セレクタ生成
    async fn new_selector(&self) -> Self::Selector {
        Self::Selector::default()
    }

    /// Suumoのヘルスチェック。トップページにログインできるかどうか。
    async fn health_check(&self, crawler: &Self::Crawler, selector: &Self::Selector) -> Result<()> {
        let url = get_env_var("URL_SUUMO_KANTO_DOMAIN").unwrap();
        let browser = crawler.browser();

        // suumo関東版のトップページに遷移する
        let page =
            <Self::Crawler as ChromiumoxideCrawler>::attempt_navigation(browser, &url).await?;

        // トップページのh1テキストを読む
        let text = match page
            .find_element(&selector.kanto_title)
            .await?
            .inner_text()
            .await?
        {
            Some(text) => text,
            None => bail!("Text is not found."),
        };

        // テキスト内容のチェック
        if text == "関東の住宅・不動産情報探し" {
            Ok(())
        } else {
            bail!("Unknown text: {}", text)
        }
    }
}
