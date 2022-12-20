use anyhow::Result;

use crate::model::{RawRooms, TableType};

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait RawRoomRepository {
    async fn find_all(&self) -> Result<RawRooms>;

    /// 指定されたテーブルのサマリを表示する
    async fn display_summary(&self, table: TableType) -> Result<()>;

    /// 複数のデータを500件ずつinsertする。
    async fn insert_many_multi(&self, source: &RawRooms, table: TableType) -> Result<()>;

    /// tempテーブルからloadテーブルに全件insertする
    async fn insert_to_load_from_temp_all(&self) -> Result<()>;

    /// tempテーブルのPK集約レコードをmainテーブルにinsertする
    async fn insert_to_main_from_temp_group_by_pk(&self) -> Result<()>;

    /// 対象テーブルのレコードを全件deleteする
    async fn delete_all(&self, table: TableType) -> Result<()>;

    /// tempテーブルのPK集約レコードと合致するレコードをmainテーブルから削除する
    async fn delete_from_main_by_temp_record_pk(&self) -> Result<()>;
}
