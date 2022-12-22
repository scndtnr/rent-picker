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
    async fn insert_to_main_from_temp_not_expired_group_by_pk(&self) -> Result<()>;

    // temp テーブルにのみ存在する掲載終了フラグ = true のレコードを
    // main テーブルに insert する。
    async fn insert_to_main_from_temp_only_expired_record(&self) -> Result<()>;

    /// tempテーブルの掲載終了フラグ = true を
    /// mainテーブルの同PKのレコードに適用する。
    async fn update_is_expired_of_main_by_temp(&self) -> Result<()>;

    /// 対象テーブルのレコードを全件deleteする
    async fn delete_all(&self, table: TableType) -> Result<()>;

    /// tempテーブルのPK集約レコードと合致するレコードをmainテーブルから削除する
    async fn delete_from_main_by_temp_not_expired_group_by_pk(&self) -> Result<()>;
}
