use anyhow::Result;
use url::Url;

use crate::model::{Residences, TargetArea};

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

    /// 検索条件を指定して賃貸一覧ページのURLを取得する
    async fn url_of_room_list(
        &self,
        crawler: &Self::Crawler,
        area: TargetArea,
        station: &str,
    ) -> Result<Url>;

    /// 賃貸一覧ページから賃貸情報や詳細ページのURLを取得する
    async fn residences_in_list_page(
        &self,
        crawler: &Self::Crawler,
        selector: &Self::Selector,
        url: Url,
    ) -> Result<Residences>;
}
