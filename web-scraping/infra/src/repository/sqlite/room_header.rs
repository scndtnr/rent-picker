use std::sync::Arc;

use crate::{model::RoomHeaderTable, progress_bar::new_progress_bar};
use futures::{stream, StreamExt, TryStreamExt};
use usecase::env::get_usize_of_env_var;

use super::{repository_impl::SqliteRepositoryImpl, Sql};
use anyhow::Result;
use domain::{
    model::{AsVec, RoomHeader, RoomHeaders, TargetArea},
    repository::RoomHeaderRepository,
};

#[async_trait::async_trait]
impl RoomHeaderRepository for SqliteRepositoryImpl<RoomHeader> {
    async fn find_by_area_and_station(
        &self,
        area: TargetArea,
        station: &str,
    ) -> Result<RoomHeaders> {
        todo!()
    }
    async fn find_all(&self) -> Result<RoomHeaders> {
        todo!()
    }

    /// 作業用ロードテーブルからPKで集約したデータを取り出す
    #[tracing::instrument(skip_all, err(Debug))]
    async fn group_by_pk_from_load_table(&self) -> Result<RoomHeaders> {
        let pool = self.reader_pool();
        let sql = Sql::new().room_header.group_by_pk_from_load_table();
        let headers_dto = sqlx::query_as::<_, RoomHeaderTable>(&sql)
            .fetch_all(pool)
            .await?;

        let headers: RoomHeaders = headers_dto
            .into_iter()
            .map(|header| header.try_into())
            .collect::<Result<Vec<RoomHeader>>>()?
            .into();
        Ok(headers)
    }

    /// データを1件ずつinsertする。
    /// is_load_table = true の場合、作業用ロードテーブルにinsertする
    #[tracing::instrument(level = "trace", skip_all, fields(url=source.url(), title=source.residence_title(), is_load_table=is_load_table), err(Debug))]
    async fn insert(&self, source: &RoomHeader, is_load_table: bool) -> Result<()> {
        let pool = self.writer_pool();
        let dto: RoomHeaderTable = source.clone().into();
        let table = if is_load_table {
            "load_room_header"
        } else {
            "room_header"
        };
        let sql = Sql::new().room_header.insert_all_column(table);
        let _ = sqlx::query(&sql)
            .bind(dto.url)
            .bind(dto.residence_title)
            .bind(dto.residence_transfer)
            .bind(dto.residence_area)
            .bind(dto.residence_station)
            .bind(dto.created_at)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// 複数のデータを1件ずつinsertする。
    /// is_load_table = true の場合、作業用ロードテーブルにinsertする
    #[tracing::instrument(skip_all, fields(len=source.len(),is_load_table=is_load_table), err(Debug))]
    async fn insert_many(&self, source: &RoomHeaders, is_load_table: bool) -> Result<()> {
        // プログレスバーの準備
        let pb_records = new_progress_bar(source.len() as u64).await;
        let target_table = if is_load_table {
            "Load Table"
        } else {
            "Main Table"
        };
        pb_records.set_message(format!("Insert to {}...", target_table));
        let buffered_n = get_usize_of_env_var("MAX_CONCURRENCY");
        let result: Result<()> = stream::iter(source.clone().into_inner())
            .map(|header| (header, Arc::clone(&pb_records)))
            .map(|(header, pb_records)| async move {
                self.insert(&header, is_load_table).await?;
                pb_records.inc(1);
                Ok(())
            })
            .buffer_unordered(buffered_n)
            .try_collect()
            .await;
        result?;
        pb_records.finish_with_message(format!("Finish Insert to {}", target_table));
        Ok(())
    }

    #[tracing::instrument(level = "trace", skip_all, fields(url=source.url(), title=source.residence_title(), is_load_table=is_load_table), err(Debug))]
    async fn delete_by_pk(&self, source: &RoomHeader, is_load_table: bool) -> Result<()> {
        let pool = self.writer_pool();
        let dto: RoomHeaderTable = source.clone().into();
        let table = if is_load_table {
            "load_room_header"
        } else {
            "room_header"
        };
        let sql = format!(
            "
            DELETE FROM {}
            WHERE
                url = ?
        ",
            table
        );
        let _ = sqlx::query(&sql).bind(dto.url).execute(pool).await?;
        Ok(())
    }

    #[tracing::instrument(skip_all, fields(len=source.len(), is_load_table=is_load_table), err(Debug))]
    async fn delete_many_by_pk(&self, source: &RoomHeaders, is_load_table: bool) -> Result<()> {
        // プログレスバーの準備
        let pb_records = new_progress_bar(source.len() as u64).await;
        let target_table = if is_load_table {
            "Load Table"
        } else {
            "Main Table"
        };
        pb_records.set_message(format!("Delete target record from {}...", target_table));
        let buffered_n = get_usize_of_env_var("MAX_CONCURRENCY");
        let result: Result<()> = stream::iter(source.clone().into_inner())
            .map(|header| (header, Arc::clone(&pb_records)))
            .map(|(header, pb_records)| async move {
                self.delete_by_pk(&header, is_load_table).await?;
                pb_records.inc(1);
                Ok(())
            })
            .buffer_unordered(buffered_n)
            .try_collect()
            .await;
        result?;
        pb_records.finish_with_message(format!(
            "Finished Deleting target record from {}",
            target_table
        ));
        Ok(())
    }
}
