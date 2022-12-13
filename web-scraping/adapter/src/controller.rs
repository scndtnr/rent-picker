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

    pub async fn search_rent_suumo(&self, dto: SuumoRequestDto) -> Rooms {
        self.usecases
            .search_rent_usecase()
            .search_rent_suumo(
                dto.area.try_into().expect("Fail to convert target area."),
                &dto.station,
            )
            .await
            .expect("Fail to search rent.")
    }
}
