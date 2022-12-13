use anyhow::Result;

use crate::model::{RoomHeader, TargetArea};

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait RoomHeaderRepository {
    async fn find_by_url(&self, url: &str) -> Result<Vec<RoomHeader>>;
    async fn find_by_area_and_station(
        &self,
        area: TargetArea,
        station: &str,
    ) -> Result<Vec<RoomHeader>>;
    async fn find_all(&self) -> Result<Vec<RoomHeader>>;
    async fn insert(&self, source: RoomHeader) -> Result<()>;
}
