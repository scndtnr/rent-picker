use std::sync::Arc;

use crate::{model::RoomHeaderTable, progress_bar::new_progress_bar};
use futures::{stream, StreamExt, TryStreamExt};
use sqlx::{QueryBuilder, Sqlite};
use usecase::env::get_usize_of_env_var;

use super::{repository_impl::SqliteRepositoryImpl, Sql};
use anyhow::Result;
use domain::{
    model::{AsVec, RoomHeader, RoomHeaders, TableType, TargetArea},
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
    async fn select_group_by_pk_from_temp_table(&self) -> Result<RoomHeaders> {
        let pool = self.reader_pool();
        let sql = Sql::new().room_header.select_group_by_pk(TableType::Temp);
        let headers_dto = sqlx::query_as::<_, RoomHeaderTable>(&sql)
            .fetch_all(&*pool)
            .await?;

        let headers: RoomHeaders = headers_dto
            .into_iter()
            .map(|header| header.try_into())
            .collect::<Result<Vec<RoomHeader>>>()?
            .into();
        Ok(headers)
    }

    /// データを1件insertする。
    #[tracing::instrument(level = "trace", skip_all, fields(url=source.url(), title=source.residence_title(), table=table.to_string()), err(Debug))]
    async fn insert(&self, source: &RoomHeader, table: TableType) -> Result<()> {
        let pool = self.writer_pool();
        let dto: RoomHeaderTable = source.clone().into();
        let sql = Sql::new().room_header.insert_all_column(table);
        let _ = sqlx::query(&sql)
            .bind(dto.url)
            .bind(dto.residence_title)
            .bind(dto.residence_address)
            .bind(dto.residence_nearest_station)
            .bind(dto.residence_age)
            .bind(dto.residence_floors)
            .bind(dto.residence_transfer)
            .bind(dto.residence_area)
            .bind(dto.residence_station)
            .bind(dto.room_floor)
            .bind(dto.room_rent_price)
            .bind(dto.room_condo_fee)
            .bind(dto.room_deposit)
            .bind(dto.room_key_money)
            .bind(dto.room_layout)
            .bind(dto.room_exclusive_area)
            .bind(dto.created_at)
            .execute(&*pool)
            .await?;
        Ok(())
    }

    /// 複数のデータを1件ずつinsertする。
    #[tracing::instrument(skip_all, fields(len=source.len(),table=table.to_string()), err(Debug))]
    async fn insert_many_one_by_one(&self, source: &RoomHeaders, table: TableType) -> Result<()> {
        // プログレスバーの準備
        let target_table = Sql::new().room_header.table_name(&table);
        let pb_records = new_progress_bar(source.len() as u64).await;
        pb_records.set_message(format!("Insert to {}...", target_table));

        // insert文の実行
        let buffered_n = get_usize_of_env_var("MAX_CONCURRENCY");
        let result: Result<()> = stream::iter(source.clone().into_inner())
            .map(|header| (header, Arc::clone(&pb_records), table.clone()))
            .map(|(header, pb_records, table)| async move {
                self.insert(&header, table).await?;
                pb_records.inc(1);
                Ok(())
            })
            .buffer_unordered(buffered_n)
            .try_collect()
            .await;
        result?;

        // プログレスバーの後始末
        pb_records.finish_with_message(format!("Finish Insert to {}", target_table));

        // 返り値
        Ok(())
    }

    /// 複数のデータを500件ずつinsertする。
    #[tracing::instrument(skip_all, fields(len=source.len(),table=table.to_string()), err(Debug))]
    async fn insert_many_multi(&self, source: &RoomHeaders, table: TableType) -> Result<()> {
        // プログレスバーの準備
        let target_table = Sql::new().room_header.table_name(&table);
        let pb_records = new_progress_bar(source.len() as u64).await;
        pb_records.set_message(format!("Insert to {}...", target_table));

        // RoomHeadersをDTOのVecに変換する
        let dto_headers: Vec<RoomHeaderTable> = source
            .clone()
            .into_inner()
            .into_iter()
            .map(|header| {
                let dto: RoomHeaderTable = header.into();
                dto
            })
            .collect();

        // n件毎にinsertするクエリビルダを生成する
        let n = 500;
        let query_builders: Vec<QueryBuilder<Sqlite>> = dto_headers
            .chunks(n)
            .map(|headers| {
                let mut query_builder: QueryBuilder<Sqlite> =
                    QueryBuilder::new(Sql::new().room_header.insert_all_header(table.clone()));
                query_builder.push_values(headers, |mut b, header| {
                    b.push_bind(header.url.clone())
                        .push_bind(header.residence_title.clone())
                        .push_bind(header.residence_address.clone())
                        .push_bind(header.residence_nearest_station.clone())
                        .push_bind(header.residence_age.clone())
                        .push_bind(header.residence_floors.clone())
                        .push_bind(header.residence_transfer.clone())
                        .push_bind(header.residence_area.clone())
                        .push_bind(header.residence_station.clone())
                        .push_bind(header.room_floor.clone())
                        .push_bind(header.room_rent_price.clone())
                        .push_bind(header.room_condo_fee.clone())
                        .push_bind(header.room_deposit.clone())
                        .push_bind(header.room_key_money.clone())
                        .push_bind(header.room_layout.clone())
                        .push_bind(header.room_exclusive_area.clone())
                        .push_bind(header.created_at);
                });
                query_builder
            })
            .collect();

        // n 件毎に insertを実行する
        let pool = self.writer_pool();
        for mut builder in query_builders {
            let query = builder.build();
            query.execute(&*pool).await?;
            pb_records.inc(n as u64);
        }

        // プログレスバーの後始末
        pb_records.finish_with_message(format!("Finish Insert to {}", target_table));

        // 返り値
        Ok(())
    }

    /// tempテーブルからloadテーブルに全件insertする
    #[tracing::instrument(level = "trace", skip_all, err(Debug))]
    async fn insert_to_load_from_temp_all(&self) -> Result<()> {
        let pool = self.writer_pool();
        let sql = Sql::new()
            .room_header
            .insert_from_other_table_all(TableType::Load, TableType::Temp);
        let _ = sqlx::query(&sql).execute(&*pool).await?;
        Ok(())
    }

    /// tempテーブルからloadテーブルに全件insertする
    #[tracing::instrument(level = "trace", skip_all, err(Debug))]
    async fn insert_to_main_from_temp_group_by_pk(&self) -> Result<()> {
        let pool = self.writer_pool();
        let sql = Sql::new()
            .room_header
            .insert_from_other_table_group_by_pk(TableType::Main, TableType::Temp);
        let _ = sqlx::query(&sql).execute(&*pool).await?;
        Ok(())
    }

    #[tracing::instrument(level = "trace", skip_all, fields(table=table.to_string()), err(Debug))]
    async fn delete_all(&self, table: TableType) -> Result<()> {
        let pool = self.writer_pool();
        let sql = Sql::new().room_header.delete_all(table);
        let _ = sqlx::query(&sql).execute(&*pool).await?;
        Ok(())
    }

    #[tracing::instrument(level = "trace", skip_all, fields(url=source.url(), title=source.residence_title(), table=table.to_string()), err(Debug))]
    async fn delete_by_pk(&self, source: &RoomHeader, table: TableType) -> Result<()> {
        let pool = self.writer_pool();
        let dto: RoomHeaderTable = source.clone().into();
        let sql = Sql::new().room_header.delete_by_pk(table);
        let _ = sqlx::query(&sql).bind(dto.url).execute(&*pool).await?;
        Ok(())
    }

    #[tracing::instrument(skip_all, fields(len=source.len(), table=table.to_string()), err(Debug))]
    async fn delete_many_by_pk(&self, source: &RoomHeaders, table: TableType) -> Result<()> {
        // プログレスバーの準備
        let target_table = Sql::new().room_header.table_name(&table);
        let pb_records = new_progress_bar(source.len() as u64).await;
        pb_records.set_message(format!("Delete target record from {}...", target_table));

        // delete文の実行
        let buffered_n = get_usize_of_env_var("MAX_CONCURRENCY");
        let result: Result<()> = stream::iter(source.clone().into_inner())
            .map(|header| (header, Arc::clone(&pb_records), table.clone()))
            .map(|(header, pb_records, table)| async move {
                self.delete_by_pk(&header, table).await?;
                pb_records.inc(1);
                Ok(())
            })
            .buffer_unordered(buffered_n)
            .try_collect()
            .await;
        result?;

        // プログレスバーの後始末
        pb_records.finish_with_message(format!(
            "Finished Deleting target record from {}",
            target_table
        ));

        // 返り値
        Ok(())
    }

    #[tracing::instrument(skip_all, err(Debug))]
    async fn delete_from_main_by_temp_record_pk(&self) -> Result<()> {
        let pool = self.writer_pool();
        let sql = Sql::new()
            .room_header
            .delete_where_group_by_pk_from_other_table(TableType::Main, TableType::Temp);
        let _ = sqlx::query(&sql).execute(&*pool).await?;
        Ok(())
    }
}
