use anyhow::Result;

use crate::model::{RoomHeader, RoomHeaders, TableType, TargetArea};

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait RoomHeaderRepository {
    async fn find_by_area_and_station(
        &self,
        area: TargetArea,
        station: &str,
    ) -> Result<RoomHeaders>;
    async fn find_all(&self) -> Result<RoomHeaders>;
    async fn select_group_by_pk_from_temp_table(&self) -> Result<RoomHeaders>;
    async fn insert(&self, source: &RoomHeader, table: TableType) -> Result<()>;
    async fn insert_many(&self, source: &RoomHeaders, table: TableType) -> Result<()>;
    async fn insert_from_temp_to_load(&self) -> Result<()>;
    async fn delete_all(&self, table: TableType) -> Result<()>;
    async fn delete_by_pk(&self, source: &RoomHeader, table: TableType) -> Result<()>;
    async fn delete_many_by_pk(&self, source: &RoomHeaders, table: TableType) -> Result<()>;
    async fn delete_main_record_by_temp_record_pk(&self) -> Result<()>;
}
