use anyhow::Result;

use crate::model::{Rooms, TargetArea};

#[cfg_attr(feature = "mock", mockall::automock(type Crawler=();))]
#[async_trait::async_trait]
pub trait SuumoRepository {
    type Crawler;

    /// クローラ生成（Browser生成タイミングを制御するため）
    async fn new_crawler(&self) -> Self::Crawler;

    /// Suumoのヘルスチェック。トップページにログインできるかどうか。
    async fn health_check(&self, crawler: &Self::Crawler) -> Result<()>;

    /// 住居の属する地域や、通勤先の駅を指定して、賃貸情報を取得する
    async fn rooms_by_area_and_station(
        &self,
        crawler: &Self::Crawler,
        area: TargetArea,
        station: &str,
    ) -> Result<Rooms>;
}
