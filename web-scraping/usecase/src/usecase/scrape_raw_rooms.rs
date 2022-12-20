use anyhow::{Context, Result};
use domain::{
    model::{AsVec, RawRooms, TableType, TargetArea},
    repository::{RawRoomRepository, Repositories, RoomHeaderRepository, SuumoRepository},
};
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScrapeRawRoomsUsecase<R: Repositories> {
    suumo_repo: R::SuumoRepo,
    room_header_repo: R::RoomHeaderRepo,
    raw_room_repo: R::RawRoomRepo,
}

impl<R: Repositories> ScrapeRawRoomsUsecase<R> {
    pub fn new(
        suumo_repo: R::SuumoRepo,
        room_header_repo: R::RoomHeaderRepo,
        raw_room_repo: R::RawRoomRepo,
    ) -> Self {
        Self {
            suumo_repo,
            room_header_repo,
            raw_room_repo,
        }
    }

    #[tracing::instrument(skip_all, err(Debug))]
    pub async fn scrape_raw_rooms_from_suumo(
        &self,
        area: TargetArea,
        save: bool,
        dry_run: bool,
    ) -> Result<RawRooms> {
        // 前準備
        let crawler = self.suumo_repo.new_crawler().await;

        // 住居の属する地域を指定して、データベースから詳細ページのURLを取り出す
        let urls = self.urls_from_database_by_area(area).await?;
        tracing::info!("Urls length: {}", urls.len());

        // 仮実行の場合、ここで処理を終了する
        if dry_run {
            let dummy_raw_rooms: RawRooms = Vec::new().into();
            return Ok(dummy_raw_rooms);
        }

        // 詳細ページのURLから賃貸の詳細情報を取得する
        let raw_rooms = self.suumo_repo.raw_rooms(&crawler, urls).await?;

        // 賃貸詳細をデータベースに保存する
        if save {
            self.save_raw_rooms_to_temp_table(&raw_rooms).await?;
            self.save_raw_rooms_to_load_table().await?;
            self.save_raw_rooms_to_main_table().await?;
        }

        tracing::info!("Scraping Page Count: {:#?}", raw_rooms.len());

        Ok(raw_rooms)
    }

    #[tracing::instrument(skip_all, err(Debug))]
    pub async fn urls_from_database_by_area(&self, area: TargetArea) -> Result<Vec<Url>> {
        // 住居の属する地域を指定して、データベースから賃貸情報ヘッダを取得する
        let room_headers = self
            .room_header_repo
            .find_unscraped_urls_with_area(area)
            .await?;

        // 賃貸情報ヘッダから詳細ページのURLを取り出す
        room_headers
            .into_inner()
            .into_iter()
            .map(|header| Url::parse(header.url()).context("Fail to parse room page url."))
            .collect()
    }

    #[tracing::instrument(skip_all, err(Debug))]
    async fn save_raw_rooms_to_temp_table(&self, raw_rooms: &RawRooms) -> Result<()> {
        // 作業用一時テーブルのデータを全削除する
        self.raw_room_repo.delete_all(TableType::Temp).await?;

        // 作業用一時テーブルにスクレイピングデータを入れ込む
        self.raw_room_repo
            .insert_many_multi(raw_rooms, TableType::Temp)
            .await?;

        Ok(())
    }

    #[tracing::instrument(skip(self), err(Debug))]
    async fn save_raw_rooms_to_load_table(&self) -> Result<()> {
        // 作業用一時テーブルから累積テーブルにデータを入れ込む
        self.raw_room_repo.insert_to_load_from_temp_all().await
    }

    #[tracing::instrument(skip_all, err(Debug))]
    async fn save_raw_rooms_to_main_table(&self) -> Result<()> {
        // 集約データとPKが一致するレコードを本テーブルから削除する
        self.raw_room_repo
            .delete_from_main_by_temp_record_pk()
            .await?;

        // 集約データを本テーブルに入れ込む
        self.raw_room_repo
            .insert_to_main_from_temp_group_by_pk()
            .await
    }
}
