use crate::env::{get_bool_of_env_var, get_env_var};

use anyhow::{bail, Context, Result};
use chromiumoxide::{
    browser::{Browser, BrowserConfig},
    cdp::browser_protocol::{page::StopLoadingParams, target::CreateTargetParams},
    Handler, Page,
};
use std::time::Duration;

#[async_trait::async_trait]
pub(crate) trait ChromiumoxideCrawler {
    // -------------------- ブラウザの設定、新規作成 --------------------------

    #[tracing::instrument(skip_all)]
    async fn config(timeout_secs: u64) -> (Browser, Handler) {
        // ヘッドレスモード判定
        let is_headless_mode = get_bool_of_env_var("IS_HEADLESS_MODE");

        // ユーザデータディレクトリ設定
        // 毎回新しいキャッシュやプロファイルで実行するため、
        // 一時ディレクトリを tempディレクトリ配下に作成している
        let data_dir = ::tempfile::Builder::new()
            .prefix("chromiumoxide-runner-")
            .tempdir()
            .expect("Fail to create temp dir, for new user data directory.");

        // ブラウザ設定
        let browser_config_builder = if is_headless_mode {
            BrowserConfig::builder()
        } else {
            BrowserConfig::builder().with_head()
        };
        let browser_config = match browser_config_builder
            .request_timeout(Duration::from_secs(timeout_secs))
            // .incognito()
            .user_data_dir(data_dir)
            .build()
        {
            Ok(config) => config,
            Err(e) => {
                tracing::error!("Fail to build browser config: {:#?}", e);
                panic!();
            }
        };

        // ブラウザを生成する
        Browser::launch(browser_config).await.unwrap_or_else(|e| {
            tracing::error!("Unable to launch browser: {:#?}", e);
            panic!();
        })
    }

    // ----------------- 新規タブを開く -------------------------------

    /// 新規タブを開いて画面遷移する
    /// timeoutした場合
    #[tracing::instrument(skip_all, fields(url=url), err(Debug), level = "debug")]
    async fn attempt_navigation(browser: &Browser, url: &str) -> Result<Page> {
        // 最初にブランクページを開き、
        // 本命のエラー処理時に page を操作できるよう準備をする
        let params = match CreateTargetParams::builder().url("about:blank").build() {
            Ok(params) => params,
            Err(e) => bail!("Fail to build CreateTargetParams for new tab. Error: {}", e),
        };
        let page = browser
            .new_page(params)
            .await
            .context("Fail to open blank page.")?;

        // タイムアウト時間の設定
        let timeout_secs = get_env_var("TIMEOUT_SECS_OF_NAVIGATE_PAGE")
            .unwrap_or_else(|_| "5".to_string())
            .parse::<u64>()?;
        let duration = tokio::time::Duration::from_secs(timeout_secs);

        // タイムアウト付きで page.goto(url) を実行する
        // タイムアウトのエラー処理でページの読み込みを中止する
        match tokio::time::timeout(duration, page.goto(url)).await {
            Ok(_page) => {
                tracing::trace!("Success to load page: {}", url);
            }
            Err(e) => {
                tracing::warn!(
                    "Stop page loading, because navigation to new page is timeout ({} secs). Error: {}",
                    timeout_secs,
                    e
                );
                page.execute(StopLoadingParams {}).await?;
            }
        };

        Ok(page)
    }

    /// タイムアウト付きでページ遷移を待つ
    #[tracing::instrument(skip_all, err(Debug), level = "debug")]
    async fn wait_for_navigation_with_timeout(page: &Page) -> Result<()> {
        // タイムアウト時間の設定
        let timeout_secs = get_env_var("TIMEOUT_SECS_OF_NAVIGATE_PAGE")
            .unwrap_or_else(|_| "5".to_string())
            .parse::<u64>()?;
        let duration = tokio::time::Duration::from_secs(timeout_secs);

        // タイムアウト付きで page.wait_for_navigation() を実行する
        // タイムアウトのエラー処理でページの読み込みを中止する
        match tokio::time::timeout(duration, page.wait_for_navigation_response()).await {
            Ok(http_request) => {
                tracing::trace!(
                    "Success to wait for new page. HttpRequest: {:#?}",
                    http_request.context("Fail to wait for new page.")?
                );
            }
            Err(e) => {
                tracing::warn!(
                    "Stop page loading, because navigation to new page is timeout ({} secs). Error: {}",
                    timeout_secs,
                    e
                );
                page.execute(StopLoadingParams {}).await?;
            }
        };

        Ok(())
    }
    // ----------------- タブを閉じる -------------------------------

    /// target_id を表示してからタブを閉じる
    #[tracing::instrument(skip_all, err(Debug))]
    async fn close_page(page: Page) -> Result<()> {
        // 使い終わったタブを閉じる
        let page_id = page.target_id().inner().clone();
        tracing::debug!("close page. ID: {:#?}", &page_id);
        page.close()
            .await
            .with_context(|| format!("Fail to close page. ID: {:#?}", page_id))
    }
}
