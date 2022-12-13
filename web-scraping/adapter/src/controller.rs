use domain::model::Rooms;
use usecase::usecase::Usecases;

use crate::dto::SuumoRequestDto;

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

    pub async fn scrape_rooms_from_suumo(&self, dto: SuumoRequestDto) -> Rooms {
        self.usecases
            .scrape_rooms_usecase()
            .scrape_rooms_from_suumo(
                dto.area.try_into().expect("Fail to convert target area."),
                &dto.station,
            )
            .await
            .expect("Fail to scrape rooms from Suumo.")
    }
}
