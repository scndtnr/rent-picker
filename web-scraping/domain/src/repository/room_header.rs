use anyhow::Result;

use crate::model::{RoomHeader, RoomHeaders, TargetArea};

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait RoomHeaderRepository {
    async fn find_by_area_and_station(
        &self,
        area: TargetArea,
        station: &str,
    ) -> Result<RoomHeaders>;
    async fn find_all(&self) -> Result<RoomHeaders>;
    async fn group_by_pk_from_load_table(&self) -> Result<RoomHeaders>;
    async fn insert(&self, source: &RoomHeader, is_load_table: bool) -> Result<()>;
    async fn insert_many(&self, source: &RoomHeaders, is_load_table: bool) -> Result<()>;
    async fn delete_by_pk(&self, source: &RoomHeader, is_load_table: bool) -> Result<()>;
    async fn delete_many_by_pk(&self, source: &RoomHeaders, is_load_table: bool) -> Result<()>;
}
