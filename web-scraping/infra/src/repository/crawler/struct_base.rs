use crate::env::get_env_var;
use chromiumoxide::Browser;
use futures::StreamExt;
use std::sync::Arc;

use super::ChromiumoxideCrawler;

#[derive(Debug, Clone)]
pub struct Crawler {
    browser: Arc<Browser>,
}

impl ChromiumoxideCrawler for Crawler {}

impl Crawler {
    // ----------------- ブラウザの設定、新規作成 ----------

    /// Crawlerの新しいインスタンスを返す
    /// 同時にブラウザを立ち上げている
    #[tracing::instrument(skip_all)]
    pub async fn new() -> Self {
        let timeout_secs: u64 = get_env_var("TIMEOUT_SECS_OF_BROWSER")
            .unwrap()
            .parse()
            .unwrap();
        let (browser, mut handler) = Self::config(timeout_secs).await;

        // chromeバイナリとコネクションを維持するためのハンドル
        let _handle = tokio::task::spawn(async move {
            loop {
                let _ = handler.next().await.unwrap();
            }
        });

        tracing::debug!("{:#?}", browser.version().await);

        Self {
            browser: Arc::new(browser),
        }
    }

    // ----------------- プロパティ アクセサ ----------

    /// 普通の参照を返す
    pub fn browser(&self) -> &Browser {
        &self.browser
    }

    /// Arc::clone()して返す
    pub fn browser_clone(&self) -> Arc<Browser> {
        Arc::clone(&self.browser)
    }
}
