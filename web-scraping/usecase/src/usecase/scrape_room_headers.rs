use anyhow::Result;
use domain::{
    model::{AsVec, RoomHeaders, TableType, TargetArea},
    repository::{Repositories, RoomHeaderRepository, SuumoRepository},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScrapeRoomHeadersUsecase<R: Repositories> {
    suumo_repo: R::SuumoRepo,
    room_header_repo: R::RoomHeaderRepo,
}

impl<R: Repositories> ScrapeRoomHeadersUsecase<R> {
    pub fn new(suumo_repo: R::SuumoRepo, room_header_repo: R::RoomHeaderRepo) -> Self {
        Self {
            suumo_repo,
            room_header_repo,
        }
    }

    #[tracing::instrument(skip(self), err(Debug))]
    pub async fn scrape_room_headers_from_suumo(
        &self,
        area: TargetArea,
        station: &str,
        save: bool,
    ) -> Result<RoomHeaders> {
        // 前準備
        let crawler = self.suumo_repo.new_crawler().await;

        // 指定した地域・通勤先の駅を元に賃貸情報を取得する
        let room_headers = self
            .suumo_repo
            .room_headers_by_area_and_station(&crawler, &area, station)
            .await?;

        // 賃貸概要をデータベースに保存する
        if save {
            self.save_room_headers_to_temp_table(&room_headers).await?;
            self.save_room_headers_to_load_table().await?;
            self.save_room_headers_to_main_table().await?;
        }

        tracing::info!("{:#?}", room_headers.len());

        Ok(room_headers)
    }

    #[tracing::instrument(skip(self, room_headers), err(Debug))]
    async fn save_room_headers_to_temp_table(&self, room_headers: &RoomHeaders) -> Result<()> {
        // 作業用一時テーブルのデータを全削除する
        self.room_header_repo.delete_all(TableType::Temp).await?;

        // 作業用一時テーブルにスクレイピングデータを入れ込む
        self.room_header_repo
            .insert_many(room_headers, TableType::Temp)
            .await?;

        Ok(())
    }

    #[tracing::instrument(skip(self), err(Debug))]
    async fn save_room_headers_to_load_table(&self) -> Result<()> {
        // 作業用一時テーブルから累積テーブルにデータを入れ込む
        self.room_header_repo.insert_to_load_from_temp_all().await
    }

    #[tracing::instrument(skip_all, err(Debug))]
    async fn save_room_headers_to_main_table(&self) -> Result<()> {
        // 集約データとPKが一致するレコードを本テーブルから削除する
        self.room_header_repo
            .delete_from_main_by_temp_record_pk()
            .await?;

        // 集約データを本テーブルに入れ込む
        self.room_header_repo
            .insert_to_main_from_temp_group_by_pk()
            .await
    }
}
