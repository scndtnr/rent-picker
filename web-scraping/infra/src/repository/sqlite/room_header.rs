use crate::{env::get_env_var, model::RoomHeaderTable};

use super::repository_impl::SqliteRepositoryImpl;
use anyhow::Result;
use domain::{
    model::{RoomHeader, TargetArea},
    repository::RoomHeaderRepository,
};

#[async_trait::async_trait]
impl RoomHeaderRepository for SqliteRepositoryImpl<RoomHeader> {
    async fn find_by_url(&self, url: &str) -> Result<Vec<RoomHeader>> {
        todo!()
    }
    async fn find_by_area_and_station(
        &self,
        area: TargetArea,
        station: &str,
    ) -> Result<Vec<RoomHeader>> {
        todo!()
    }
    async fn find_all(&self) -> Result<Vec<RoomHeader>> {
        todo!()
    }
    async fn insert(&self, source: RoomHeader) -> Result<()> {
        let pool = self.pool_clone();
        let dto: RoomHeaderTable = source.into();
        let sql = get_env_var("SQL_INSERT_ROOM_HEADER").unwrap();
        let _ = sqlx::query(&sql)
            .bind(dto.url)
            .bind(dto.residence_title)
            .bind(dto.residence_transfer)
            .bind(dto.residence_area)
            .bind(dto.residence_station)
            .bind(dto.created_at)
            .execute(&*pool)
            .await?;
        Ok(())
    }
}
