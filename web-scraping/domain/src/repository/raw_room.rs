use anyhow::Result;

use crate::model::{RawRooms, TableType};

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait RawRoomRepository {
    async fn find_all(&self) -> Result<RawRooms>;
    async fn display_summary(&self, table: TableType) -> Result<()>;
    async fn insert_many_multi(&self, source: &RawRooms, table: TableType) -> Result<()>;
    async fn insert_to_load_from_temp_all(&self) -> Result<()>;
    async fn insert_to_main_from_temp_group_by_pk(&self) -> Result<()>;
    async fn delete_all(&self, table: TableType) -> Result<()>;
    async fn delete_from_main_by_temp_record_pk(&self) -> Result<()>;
}
