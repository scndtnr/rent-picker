use std::sync::Arc;

use crate::{
    model::{RoomHeaderRecord, RoomHeaderSummaryRecord, RoomHeaderSummaryTable},
    progress_bar::{debug_progress, new_progress_bar},
};
use futures::{stream, StreamExt, TryStreamExt};
use sqlx::{QueryBuilder, Sqlite};
use usecase::env::get_usize_of_env_var;

use super::{repository_impl::SqliteRepositoryImpl, sql};
use anyhow::Result;
use domain::{
    model::{AsVec, RoomHeader, RoomHeaders, TableType, TargetArea},
    repository::RoomHeaderRepository,
};

#[async_trait::async_trait]
impl RoomHeaderRepository for SqliteRepositoryImpl<RoomHeader> {
    #[allow(unused_variables)]
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

    /// 指定されたテーブルのサマリを表示する
    #[tracing::instrument(level = "trace", skip_all, fields(table=table.to_string()), err(Debug))]
    async fn display_summary(&self, table: TableType) -> Result<()> {
        let pool = self.reader_pool();
        let table_name = sql::room_header::table_name(&table);
        let sql = sql::room_header_summary::group_by_area_and_station(&table);
        let summary: RoomHeaderSummaryTable = sqlx::query_as::<_, RoomHeaderSummaryRecord>(&sql)
            .fetch_all(&*pool)
            .await?
            .into();
        tracing::info!("{:#?}", summary);
        tracing::info!(
            "[{}] Total records count : {}",
            table_name,
            summary.total_count()
        );
        Ok(())
    }

    /// 作業用ロードテーブルからPKで集約したデータを取り出す
    #[tracing::instrument(level = "debug", skip_all, err(Debug))]
    async fn select_group_by_pk_from_temp_table(&self) -> Result<RoomHeaders> {
        let pool = self.reader_pool();
        let sql = sql::room_header::select_group_by_pk(TableType::Temp);
        let headers_dto = sqlx::query_as::<_, RoomHeaderRecord>(&sql)
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
    #[tracing::instrument(level = "trace", skip_all, fields(url=source.url(), building_name=source.building_name(), table=table.to_string()), err(Debug))]
    async fn insert(&self, source: &RoomHeader, table: TableType) -> Result<()> {
        let pool = self.writer_pool();
        let dto: RoomHeaderRecord = source.clone().into();
        let sql = sql::room_header::insert_all_column(table);
        let _ = sqlx::query(&sql)
            .bind(dto.url)
            .bind(dto.building_name)
            .bind(dto.location)
            .bind(dto.walk_to_station)
            .bind(dto.age_in_years)
            .bind(dto.number_of_floors)
            .bind(dto.transfer_in_search_result)
            .bind(dto.area_of_search_condition)
            .bind(dto.commute_station_of_search_condition)
            .bind(dto.floor)
            .bind(dto.rental_fee)
            .bind(dto.management_fee)
            .bind(dto.security_deposit)
            .bind(dto.key_money)
            .bind(dto.floor_plan)
            .bind(dto.private_area)
            .bind(dto.scraping_date)
            .execute(&*pool)
            .await?;
        Ok(())
    }

    /// 複数のデータを1件ずつinsertする。
    #[tracing::instrument(level = "debug", skip_all, fields(len=source.len(),table=table.to_string()), err(Debug))]
    async fn insert_many_one_by_one(&self, source: &RoomHeaders, table: TableType) -> Result<()> {
        // プログレスバーの準備
        let target_table = sql::room_header::table_name(&table);
        let pb_records = new_progress_bar(source.len() as u64).await;
        pb_records.set_message(format!("Insert to {}...", target_table));

        // insert文の実行
        let buffered_n = get_usize_of_env_var("MAX_CONCURRENCY");
        let result: Result<()> = stream::iter(source.clone().into_inner())
            .map(|header| (header, Arc::clone(&pb_records), table.clone()))
            .map(|(header, pb_records, table)| async move {
                self.insert(&header, table).await?;
                pb_records.inc(1);
                debug_progress(&pb_records, "Insert record...").await;
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
    #[tracing::instrument(level = "debug", skip_all, fields(len=source.len(),table=table.to_string()), err(Debug))]
    async fn insert_many_multi(&self, source: &RoomHeaders, table: TableType) -> Result<()> {
        // プログレスバーの準備
        let target_table = sql::room_header::table_name(&table);
        let pb_records = new_progress_bar(source.len() as u64).await;
        pb_records.set_message(format!("Insert to {}...", target_table));

        // RoomHeadersをDTOのVecに変換する
        let dto_headers: Vec<RoomHeaderRecord> = source
            .clone()
            .into_inner()
            .into_iter()
            .map(|header| {
                let dto: RoomHeaderRecord = header.into();
                dto
            })
            .collect();

        // n件毎にinsertするクエリビルダを生成する
        let n = 500;
        let query_builders: Vec<QueryBuilder<Sqlite>> = dto_headers
            .chunks(n)
            .map(|headers| {
                let mut query_builder: QueryBuilder<Sqlite> =
                    QueryBuilder::new(sql::room_header::insert_all_header(table.clone()));
                query_builder.push_values(headers, |mut b, header| {
                    b.push_bind(header.url.clone())
                        .push_bind(header.building_name.clone())
                        .push_bind(header.location.clone())
                        .push_bind(header.walk_to_station.clone())
                        .push_bind(header.age_in_years.clone())
                        .push_bind(header.number_of_floors.clone())
                        .push_bind(header.transfer_in_search_result.clone())
                        .push_bind(header.area_of_search_condition.clone())
                        .push_bind(header.commute_station_of_search_condition.clone())
                        .push_bind(header.floor.clone())
                        .push_bind(header.rental_fee.clone())
                        .push_bind(header.management_fee.clone())
                        .push_bind(header.security_deposit.clone())
                        .push_bind(header.key_money.clone())
                        .push_bind(header.floor_plan.clone())
                        .push_bind(header.private_area.clone())
                        .push_bind(header.scraping_date);
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
            debug_progress(&pb_records, "Insert record...").await;
        }

        // プログレスバーの後始末
        pb_records.finish_with_message(format!("Finish Insert to {}", target_table));

        // 返り値
        Ok(())
    }

    /// tempテーブルからloadテーブルに全件insertする
    #[tracing::instrument(level = "debug", skip_all, err(Debug))]
    async fn insert_to_load_from_temp_all(&self) -> Result<()> {
        let pool = self.writer_pool();
        let sql = sql::room_header::insert_from_other_table_all(TableType::Load, TableType::Temp);
        let _ = sqlx::query(&sql).execute(&*pool).await?;
        Ok(())
    }

    /// tempテーブルからloadテーブルに全件insertする
    #[tracing::instrument(level = "debug", skip_all, err(Debug))]
    async fn insert_to_main_from_temp_group_by_pk(&self) -> Result<()> {
        let pool = self.writer_pool();
        let sql =
            sql::room_header::insert_from_other_table_group_by_pk(TableType::Main, TableType::Temp);
        let _ = sqlx::query(&sql).execute(&*pool).await?;
        Ok(())
    }

    #[tracing::instrument(level = "debug", skip_all, fields(table=table.to_string()), err(Debug))]
    async fn delete_all(&self, table: TableType) -> Result<()> {
        let pool = self.writer_pool();
        let sql = sql::room_header::delete_all(table);
        let _ = sqlx::query(&sql).execute(&*pool).await?;
        Ok(())
    }

    #[tracing::instrument(level = "trace", skip_all, fields(url=source.url(), building_name=source.building_name(), table=table.to_string()), err(Debug))]
    async fn delete_by_pk(&self, source: &RoomHeader, table: TableType) -> Result<()> {
        let pool = self.writer_pool();
        let dto: RoomHeaderRecord = source.clone().into();
        let sql = sql::room_header::delete_by_pk(table);
        let _ = sqlx::query(&sql).bind(dto.url).execute(&*pool).await?;
        Ok(())
    }

    #[tracing::instrument(level = "debug", skip_all, fields(len=source.len(), table=table.to_string()), err(Debug))]
    async fn delete_many_by_pk(&self, source: &RoomHeaders, table: TableType) -> Result<()> {
        // プログレスバーの準備
        let target_table = sql::room_header::table_name(&table);
        let pb_records = new_progress_bar(source.len() as u64).await;
        pb_records.set_message(format!("Delete target record from {}...", target_table));

        // delete文の実行
        let buffered_n = get_usize_of_env_var("MAX_CONCURRENCY");
        let result: Result<()> = stream::iter(source.clone().into_inner())
            .map(|header| (header, Arc::clone(&pb_records), table.clone()))
            .map(|(header, pb_records, table)| async move {
                self.delete_by_pk(&header, table).await?;
                pb_records.inc(1);
                debug_progress(&pb_records, "Delete target record...").await;
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

    #[tracing::instrument(level = "debug", skip_all, err(Debug))]
    async fn delete_from_main_by_temp_record_pk(&self) -> Result<()> {
        let pool = self.writer_pool();
        let sql = sql::room_header::delete_where_group_by_pk_from_other_table(
            TableType::Main,
            TableType::Temp,
        );
        let _ = sqlx::query(&sql).execute(&*pool).await?;
        Ok(())
    }
}
