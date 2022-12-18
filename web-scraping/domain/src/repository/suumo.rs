use std::sync::Arc;

use anyhow::Result;
use url::Url;

use crate::model::{RoomHeaders, Rooms, TargetArea};

#[cfg_attr(feature = "mock", mockall::automock(type Crawler=();))]
#[async_trait::async_trait]
pub trait SuumoRepository {
    type Crawler;

    /// クローラ生成（Browser生成タイミングを制御するため）
    async fn new_crawler(&self) -> Arc<Self::Crawler>;

    /// Suumoのヘルスチェック。トップページにログインできるかどうか。
    async fn health_check(&self, crawler: &Self::Crawler) -> Result<()>;

    /// 住居の属する地域や、通勤先の駅を指定して、賃貸一覧ページのURLを取得する
    async fn urls_of_list_page(
        &self,
        crawler: &Self::Crawler,
        area: &TargetArea,
        station: &str,
    ) -> Result<Vec<Url>>;

    /// 住居の属する地域や、通勤先の駅を指定して、賃貸の概要とURLを取得する
    async fn room_headers(
        &self,
        crawler: &Self::Crawler,
        urls: Vec<Url>,
        area: &TargetArea,
        station: &str,
    ) -> Result<RoomHeaders>;

    /// RoomHeadersから、賃貸の詳細情報を取得する
    async fn rooms_by_room_headers(
        &self,
        crawler: &Self::Crawler,
        headers: &RoomHeaders,
    ) -> Result<Rooms>;
}
