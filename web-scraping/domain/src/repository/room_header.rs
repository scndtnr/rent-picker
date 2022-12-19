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
    async fn display_summary(&self, table: TableType) -> Result<()>;
    async fn select_group_by_pk_from_temp_table(&self) -> Result<RoomHeaders>;
    async fn insert(&self, source: &RoomHeader, table: TableType) -> Result<()>;
    async fn insert_many_one_by_one(&self, source: &RoomHeaders, table: TableType) -> Result<()>;
    async fn insert_many_multi(&self, source: &RoomHeaders, table: TableType) -> Result<()>;
    async fn insert_to_load_from_temp_all(&self) -> Result<()>;
    async fn insert_to_main_from_temp_group_by_pk(&self) -> Result<()>;
    async fn delete_all(&self, table: TableType) -> Result<()>;
    async fn delete_by_pk(&self, source: &RoomHeader, table: TableType) -> Result<()>;
    async fn delete_many_by_pk(&self, source: &RoomHeaders, table: TableType) -> Result<()>;
    async fn delete_from_main_by_temp_record_pk(&self) -> Result<()>;
}
