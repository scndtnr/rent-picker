use super::SqliteRepositoryImpl;
use anyhow::Result;
use domain::{
    model::{RawRoom, RawRooms, TableType},
    repository::RawRoomRepository,
};

#[async_trait::async_trait]
impl RawRoomRepository for SqliteRepositoryImpl<RawRoom> {
    async fn find_all(&self) -> Result<RawRooms> {
        todo!();
    }
    async fn display_summary(&self, table: TableType) -> Result<()> {
        todo!();
    }
    async fn insert_many_multi(&self, source: &RawRooms, table: TableType) -> Result<()> {
        todo!();
    }
    async fn insert_to_load_from_temp_all(&self) -> Result<()> {
        todo!();
    }
    async fn insert_to_main_from_temp_group_by_pk(&self) -> Result<()> {
        todo!();
    }
    async fn delete_all(&self, table: TableType) -> Result<()> {
        todo!();
    }
    async fn delete_from_main_by_temp_record_pk(&self) -> Result<()> {
        todo!();
    }
}
