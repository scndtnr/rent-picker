use super::SqliteRepositoryImpl;
use anyhow::Result;
use domain::{
    model::{AsVec, RawRoom, RawRooms, TableType},
    repository::RawRoomRepository,
};

#[async_trait::async_trait]
impl RawRoomRepository for SqliteRepositoryImpl<RawRoom> {
    async fn find_all(&self) -> Result<RawRooms> {
        todo!();
    }

    /// 指定されたテーブルのサマリを表示する
    #[tracing::instrument(level = "trace", skip_all, fields(table=table.to_string()), err(Debug))]
    #[allow(unused_variables)]
    async fn display_summary(&self, table: TableType) -> Result<()> {
        todo!();
    }

    /// 複数のデータを500件ずつinsertする。
    #[tracing::instrument(level = "debug", skip_all, fields(len=source.len(),table=table.to_string()), err(Debug))]
    #[allow(unused_variables)]
    async fn insert_many_multi(&self, source: &RawRooms, table: TableType) -> Result<()> {
        todo!();
    }

    /// tempテーブルからloadテーブルに全件insertする
    #[tracing::instrument(level = "debug", skip_all, err(Debug))]
    async fn insert_to_load_from_temp_all(&self) -> Result<()> {
        todo!();
    }

    /// tempテーブルのPK集約レコードをmainテーブルにinsertする
    #[tracing::instrument(level = "debug", skip_all, err(Debug))]
    async fn insert_to_main_from_temp_group_by_pk(&self) -> Result<()> {
        todo!();
    }

    /// 対象テーブルのレコードを全件deleteする
    #[tracing::instrument(level = "debug", skip_all, fields(table=table.to_string()), err(Debug))]
    #[allow(unused_variables)]
    async fn delete_all(&self, table: TableType) -> Result<()> {
        todo!();
    }

    /// tempテーブルのPK集約レコードと合致するレコードをmainテーブルから削除する
    #[tracing::instrument(level = "debug", skip_all, err(Debug))]
    async fn delete_from_main_by_temp_record_pk(&self) -> Result<()> {
        todo!();
    }
}
