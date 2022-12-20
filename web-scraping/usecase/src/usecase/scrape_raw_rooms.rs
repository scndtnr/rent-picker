use anyhow::Result;
use domain::{
    model::{AsVec, RawRooms, RoomHeaders, TargetArea},
    repository::{Repositories, RoomHeaderRepository, SuumoRepository},
};

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
        todo!()
    }
}
