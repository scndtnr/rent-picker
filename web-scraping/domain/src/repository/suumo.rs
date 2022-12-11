use anyhow::Result;

#[cfg_attr(feature = "mock", mockall::automock(type Crawler=();))]
#[async_trait::async_trait]
pub trait SuumoRepository {
    type Crawler;
    type Selector;

    /// クローラ生成（Browser生成タイミングを制御するため）
    async fn new_crawler(&self) -> Self::Crawler;

    /// セレクタ生成
    async fn new_selector(&self) -> Self::Selector;

    /// Suumoのヘルスチェック。トップページにログインできるかどうか。
    async fn health_check(&self, crawler: &Self::Crawler, selector: &Self::Selector) -> Result<()>;
}
