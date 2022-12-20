use crate::{
    model::{RoomHeaderRecord, RoomHeaderSummaryRecord, RoomHeaderSummaryTable},
    progress_bar::{debug_progress, new_progress_bar},
};
use sqlx::{QueryBuilder, Sqlite};

use super::{repository_impl::SqliteRepositoryImpl, sql};
use anyhow::Result;
use domain::{
    model::{AsVec, RoomHeader, RoomHeaders, TableType, TargetArea},
    repository::RoomHeaderRepository,
};

#[async_trait::async_trait]
impl RoomHeaderRepository for SqliteRepositoryImpl<RoomHeader> {
    #[allow(unused_variables)]
    async fn find_by_area(&self, area: TargetArea) -> Result<RoomHeaders> {
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

    /// tempテーブルのPK集約レコードをmainテーブルにinsertする
    #[tracing::instrument(level = "debug", skip_all, err(Debug))]
    async fn insert_to_main_from_temp_group_by_pk(&self) -> Result<()> {
        let pool = self.writer_pool();
        let sql =
            sql::room_header::insert_from_other_table_group_by_pk(TableType::Main, TableType::Temp);
        let _ = sqlx::query(&sql).execute(&*pool).await?;
        Ok(())
    }

    /// 対象テーブルのレコードを全件deleteする
    #[tracing::instrument(level = "debug", skip_all, fields(table=table.to_string()), err(Debug))]
    async fn delete_all(&self, table: TableType) -> Result<()> {
        let pool = self.writer_pool();
        let sql = sql::room_header::delete_all(table);
        let _ = sqlx::query(&sql).execute(&*pool).await?;
        Ok(())
    }

    /// tempテーブルのPK集約レコードと合致するレコードをmainテーブルから削除する
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
