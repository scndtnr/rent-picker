use crate::model::RoomHeaderTable;
use futures::{stream, StreamExt, TryStreamExt};
use usecase::env::get_usize_of_env_var;

use super::repository_impl::SqliteRepositoryImpl;
use anyhow::Result;
use domain::{
    model::{AsVec, RoomHeader, RoomHeaders, TargetArea},
    repository::RoomHeaderRepository,
};

#[async_trait::async_trait]
impl RoomHeaderRepository for SqliteRepositoryImpl<RoomHeader> {
    async fn find_by_url(&self, url: &str) -> Result<RoomHeaders> {
        todo!()
    }
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
        let sql = "
            SELECT 
                url,
                max(residence_title) as residence_title,
                max(residence_transfer) as residence_transfer ,
                max(residence_area) as residence_area ,
                max(residence_station) as residence_station ,
                max(created_at) as created_at 
            FROM 
                load_room_header lrh 
            GROUP BY
                url
        ";
        let headers_dto = sqlx::query_as::<_, RoomHeaderTable>(sql)
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
    #[tracing::instrument(skip_all, fields(url=source.url(), title=source.residence_title(), is_load_table=is_load_table), err(Debug))]
    async fn insert(&self, source: &RoomHeader, is_load_table: bool) -> Result<()> {
        let pool = self.writer_pool();
        let dto: RoomHeaderTable = source.clone().into();
        let table = if is_load_table {
            "load_room_header"
        } else {
            "room_header"
        };
        let sql = format!("
            INSERT INTO
                {}
                    (url, residence_title, residence_transfer, residence_area, residence_station, created_at)
                VALUES
                    (?, ?, ?, ?, ?, ?)
        ", table);
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
    async fn insert_many(&self, source: &RoomHeaders, is_load_table: bool) -> Result<()> {
        let buffered_n = get_usize_of_env_var("MAX_CONCURRENCY");
        let result: Result<()> = stream::iter(source.clone().into_inner())
            .map(|header| async move { self.insert(&header, is_load_table).await })
            .buffer_unordered(buffered_n)
            .try_collect()
            .await;
        result?;
        Ok(())
    }

    #[tracing::instrument(skip_all, fields(url=source.url(), title=source.residence_title(), is_load_table=is_load_table), err(Debug))]
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
    async fn delete_many_by_pk(&self, source: &RoomHeaders, is_load_table: bool) -> Result<()> {
        let buffered_n = get_usize_of_env_var("MAX_CONCURRENCY");
        let result: Result<()> = stream::iter(source.clone().into_inner())
            .map(|header| async move { self.delete_by_pk(&header, is_load_table).await })
            .buffer_unordered(buffered_n)
            .try_collect()
            .await;
        result?;
        Ok(())
    }
}
