use crate::{
    model::RawRoomRecord,
    progress_bar::{debug_progress, new_progress_bar},
};

use super::{sql, SqliteRepositoryImpl};
use anyhow::Result;
use domain::{
    model::{AsVec, RawRoom, RawRooms, TableType},
    repository::RawRoomRepository,
};
use sqlx::{QueryBuilder, Sqlite};

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
    async fn insert_many_multi(&self, source: &RawRooms, table: TableType) -> Result<()> {
        // プログレスバーの準備
        let target_table = sql::raw_room::table_name(&table);
        let pb_records = new_progress_bar(source.len() as u64).await;
        pb_records.set_message(format!("Insert to {}...", target_table));

        // RawRoomsをDTOのVecに変換する
        let dto_raw_rooms: Vec<RawRoomRecord> = source
            .clone()
            .into_inner()
            .into_iter()
            .map(|raw_room| raw_room.into())
            .collect();

        // n件毎にinsertするクエリビルダを生成する
        let n = 500;
        let query_builders: Vec<QueryBuilder<Sqlite>> = dto_raw_rooms
            .chunks(n)
            .map(|raw_rooms| {
                let mut query_builder: QueryBuilder<Sqlite> =
                    QueryBuilder::new(sql::raw_room::insert_all_columns(&table));
                query_builder.push_values(raw_rooms, |mut b, raw_room| {
                    b.push_bind(raw_room.url.clone())
                        .push_bind(raw_room.suumo_code.clone())
                        .push_bind(raw_room.building_name.clone())
                        .push_bind(raw_room.rental_fee.clone())
                        .push_bind(raw_room.management_fee.clone())
                        .push_bind(raw_room.security_deposit.clone())
                        .push_bind(raw_room.key_money.clone())
                        .push_bind(raw_room.guarantee_deposit.clone())
                        .push_bind(raw_room.key_money_amortization.clone())
                        .push_bind(raw_room.location.clone())
                        .push_bind(raw_room.walk_to_station.clone())
                        .push_bind(raw_room.floor_plan.clone())
                        .push_bind(raw_room.floor_plan_details.clone())
                        .push_bind(raw_room.private_area.clone())
                        .push_bind(raw_room.age_in_years.clone())
                        .push_bind(raw_room.construction_date_yyyymm.clone())
                        .push_bind(raw_room.floor.clone())
                        .push_bind(raw_room.number_of_floors.clone())
                        .push_bind(raw_room.facing_direction.clone())
                        .push_bind(raw_room.building_type.clone())
                        .push_bind(raw_room.features.clone())
                        .push_bind(raw_room.structure.clone())
                        .push_bind(raw_room.damage_insurance.clone())
                        .push_bind(raw_room.parking.clone())
                        .push_bind(raw_room.move_in.clone())
                        .push_bind(raw_room.transaction_type.clone())
                        .push_bind(raw_room.conditions.clone())
                        .push_bind(raw_room.property_code.clone())
                        .push_bind(raw_room.contract_period.clone())
                        .push_bind(raw_room.notes.clone())
                        .push_bind(raw_room.info_update_date)
                        .push_bind(raw_room.next_update_date)
                        .push_bind(raw_room.scraping_date)
                        .push_bind(raw_room.is_expired as u8);
                });
                query_builder
            })
            .collect();

        // n 件毎に insert を実行する
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
        let sql = sql::raw_room::insert_from_other_table_all(&TableType::Load, &TableType::Temp);
        let _ = sqlx::query(&sql).execute(&*pool).await?;
        Ok(())
    }

    /// tempテーブルのPK集約レコードをmainテーブルにinsertする
    #[tracing::instrument(level = "debug", skip_all, err(Debug))]
    async fn insert_to_main_from_temp_not_expired_group_by_pk(&self) -> Result<()> {
        let pool = self.writer_pool();
        let sql = sql::raw_room::insert_from_other_table_group_by_pk(
            &TableType::Main,
            &TableType::Temp,
            false,
        );
        let _ = sqlx::query(&sql).execute(&*pool).await?;
        Ok(())
    }

    // temp テーブルにのみ存在する掲載終了フラグ = true のレコードを
    // main テーブルに insert する。
    #[tracing::instrument(level = "debug", skip_all, err(Debug))]
    async fn insert_to_main_from_temp_only_expired_record(&self) -> Result<()> {
        let pool = self.writer_pool();
        let sql = sql::raw_room::insert_from_other_table_by_is_expired(
            &TableType::Main,
            &TableType::Temp,
            true,
        );
        let _ = sqlx::query(&sql).execute(&*pool).await?;
        Ok(())
    }

    /// tempテーブルの掲載終了フラグ = true を
    /// mainテーブルの同PKのレコードに適用する。
    #[tracing::instrument(level = "debug", skip_all, err(Debug))]
    async fn update_is_expired_of_main_by_temp(&self) -> Result<()> {
        let pool = self.writer_pool();
        let sql = sql::raw_room::update_is_expired_column_by_other_table(
            &TableType::Main,
            &TableType::Temp,
            true,
        );
        let _ = sqlx::query(&sql).execute(&*pool).await?;
        Ok(())
    }

    /// 対象テーブルのレコードを全件deleteする
    #[tracing::instrument(level = "debug", skip_all, fields(table=table.to_string()), err(Debug))]
    async fn delete_all(&self, table: TableType) -> Result<()> {
        let pool = self.writer_pool();
        let sql = sql::raw_room::delete_all(&table);
        let _ = sqlx::query(&sql).execute(&*pool).await?;
        Ok(())
    }

    /// tempテーブルのPK集約レコードと合致するレコードをmainテーブルから削除する
    #[tracing::instrument(level = "debug", skip_all, err(Debug))]
    async fn delete_from_main_by_temp_not_expired_group_by_pk(&self) -> Result<()> {
        let pool = self.writer_pool();
        let sql = sql::raw_room::delete_where_group_by_pk_from_other_table(
            &TableType::Main,
            &TableType::Temp,
            false,
        );
        let _ = sqlx::query(&sql).execute(&*pool).await?;
        Ok(())
    }
}
