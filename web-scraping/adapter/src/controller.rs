use domain::model::{RawRooms, RoomHeaders};
use usecase::Usecases;

use crate::dto::{ReadDbRequestDto, ScrapeSuumoRawRoomParamsDto, ScrapeSuumoRoomHeaderParamsDto};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Controller<U> {
    usecases: U,
}

impl<U: Usecases> Controller<U> {
    pub fn new(usecases: U) -> Self {
        Self { usecases }
    }

    pub async fn health_check_suumo(&self) {
        self.usecases
            .health_check_usecase()
            .health_check_suumo()
            .await
            .expect("Fail to health check of suumo.");
    }

    pub async fn scrape_raw_rooms_from_suumo(&self, dto: ScrapeSuumoRawRoomParamsDto) -> RawRooms {
        self.usecases
            .scrape_raw_rooms_usecase()
            .scrape_raw_rooms_from_suumo(
                dto.area.try_into().expect("Fail to convert target area."),
                dto.max_page,
                dto.save,
                dto.dry_run,
            )
            .await
            .expect("Fail to scrape rooms from Suumo.")
    }

    pub async fn scrape_room_headers_from_suumo(
        &self,
        dto: ScrapeSuumoRoomHeaderParamsDto,
    ) -> RoomHeaders {
        self.usecases
            .scrape_room_headers_usecase()
            .scrape_room_headers_from_suumo(
                dto.area.try_into().expect("Fail to convert target area."),
                &dto.station,
                dto.max_page,
                dto.save,
                dto.dry_run,
            )
            .await
            .expect("Fail to scrape room headers from Suumo.")
    }

    pub async fn read_db_for_summary(&self, dto: ReadDbRequestDto) {
        self.usecases
            .read_db_for_summary_usecase()
            .read_room_headers_summary(
                dto.table_name
                    .try_into()
                    .expect("Fail to convert table_name"),
                dto.table_type
                    .try_into()
                    .expect("Fail to convert table_name"),
            )
            .await
            .expect("Fail to read db for summary.")
    }
}
