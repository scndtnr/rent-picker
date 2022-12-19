use anyhow::Result;
use domain::{
    model::{TableName, TableType},
    repository::{Repositories, RoomHeaderRepository},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReadDbForSummaryUsecase<R: Repositories> {
    room_header_repo: R::RoomHeaderRepo,
}

impl<R: Repositories> ReadDbForSummaryUsecase<R> {
    pub fn new(room_header_repo: R::RoomHeaderRepo) -> Self {
        Self { room_header_repo }
    }

    #[tracing::instrument(skip_all, fields(table_name=table_name.to_string(), table_type=table_type.to_string()), err(Debug))]
    pub async fn read_room_headers_summary(
        &self,
        table_name: TableName,
        table_type: TableType,
    ) -> Result<()> {
        match table_name {
            TableName::RoomHeader => self.room_header_repo.display_summary(table_type).await?,
            TableName::Room => todo!(),
        }

        Ok(())
    }
}
