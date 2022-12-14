use anyhow::Result;
use domain::{
    model::{AsVec, RoomHeaders, Rooms, TargetArea},
    repository::{Repositories, RoomHeaderRepository, SuumoRepository},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScrapeRoomsUsecase<R: Repositories> {
    suumo_repo: R::SuumoRepo,
    room_header_repo: R::RoomHeaderRepo,
}

impl<R: Repositories> ScrapeRoomsUsecase<R> {
    pub fn new(suumo_repo: R::SuumoRepo, room_header_repo: R::RoomHeaderRepo) -> Self {
        Self {
            suumo_repo,
            room_header_repo,
        }
    }

    #[tracing::instrument(skip_all, err(Debug))]
    pub async fn scrape_rooms_from_suumo(
        &self,
        area: TargetArea,
        station: &str,
        save: bool,
        headers_from_database: bool,
    ) -> Result<Rooms> {
        // 前準備
        let crawler = self.suumo_repo.new_crawler().await;

        // 指定した地域・通勤先の駅を元に賃貸概要を取得する
        let room_headers = if headers_from_database {
            // データベースから取得する
            self.room_header_repo
                .find_by_area_and_station(area, station)
                .await?
        } else {
            // Webサイトから取得する
            self.suumo_repo
                .room_headers_by_area_and_station(&crawler, &area, station)
                .await?
        };
        tracing::info!("{:#?}", room_headers.len());

        // 賃貸概要をデータベースに保存する
        if save {
            self.save_room_headers_to_database(&room_headers).await?
        }

        // 各詳細ページから賃貸の詳細情報を取得する
        let rooms = self
            .suumo_repo
            .rooms_by_room_headers(&crawler, &room_headers)
            .await?;

        // 賃貸詳細をデータベースに保存する
        if save {
            self.save_rooms_to_database(&rooms).await?
        }

        Ok(rooms)
    }

    #[tracing::instrument(skip_all, err(Debug))]
    async fn save_room_headers_to_database(&self, room_headers: &RoomHeaders) -> Result<()> {
        todo!()
    }

    #[tracing::instrument(skip_all, err(Debug))]
    async fn save_rooms_to_database(&self, rooms: &Rooms) -> Result<()> {
        todo!()
    }
}
