use anyhow::Result;
use url::Url;

use crate::model::{RoomHeaders, TableType, TargetArea};

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait::async_trait]
pub trait RoomHeaderRepository {
    async fn find_all(&self) -> Result<RoomHeaders>;

    /// まだ賃貸詳細データをスクレイピングしていない、
    /// あるいは最終更新が古いURLを返す
    async fn find_unscraped_raw_room_urls_with_area(&self, area: TargetArea) -> Result<Vec<Url>>;

    /// 指定されたテーブルのサマリを表示する
    async fn display_summary(&self, table: TableType) -> Result<()>;

    /// 複数のデータを500件ずつinsertする。
    async fn insert_many_multi(&self, source: &RoomHeaders, table: TableType) -> Result<()>;

    /// tempテーブルからloadテーブルに全件insertする
    async fn insert_to_load_from_temp_all(&self) -> Result<()>;

    /// tempテーブルのPK集約レコードをmainテーブルにinsertする
    async fn insert_to_main_from_temp_group_by_pk(&self) -> Result<()>;

    /// 対象テーブルのレコードを全件deleteする
    async fn delete_all(&self, table: TableType) -> Result<()>;

    /// tempテーブルのPK集約レコードと合致するレコードをmainテーブルから削除する
    async fn delete_from_main_by_temp_group_by_pk(&self) -> Result<()>;
}
