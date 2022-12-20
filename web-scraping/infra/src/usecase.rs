use domain::repository::Repositories;
use usecase::{
    HealthCheckUsecase, ReadDbForSummaryUsecase, ScrapeRawRoomsUsecase, ScrapeRoomHeadersUsecase,
    Usecases,
};

use crate::RepositoryImpls;

#[derive(Debug, Clone)]
pub struct UsecaseImpls {
    health_check_usecase: HealthCheckUsecase<RepositoryImpls>,
    scrape_raw_rooms_usecase: ScrapeRawRoomsUsecase<RepositoryImpls>,
    scrape_room_headers_usecase: ScrapeRoomHeadersUsecase<RepositoryImpls>,
    read_db_for_summary_usecase: ReadDbForSummaryUsecase<RepositoryImpls>,
}

impl Usecases for UsecaseImpls {
    type Repositories = RepositoryImpls;

    fn health_check_usecase(&self) -> &HealthCheckUsecase<Self::Repositories> {
        &self.health_check_usecase
    }
    fn scrape_raw_rooms_usecase(&self) -> &ScrapeRawRoomsUsecase<Self::Repositories> {
        &self.scrape_raw_rooms_usecase
    }
    fn scrape_room_headers_usecase(&self) -> &ScrapeRoomHeadersUsecase<Self::Repositories> {
        &self.scrape_room_headers_usecase
    }
    fn read_db_for_summary_usecase(&self) -> &ReadDbForSummaryUsecase<Self::Repositories> {
        &self.read_db_for_summary_usecase
    }
}

impl UsecaseImpls {
    pub fn new(repositories: RepositoryImpls) -> Self {
        let health_check_usecase =
            HealthCheckUsecase::new(repositories.suumo_repository().to_owned());
        let scrape_raw_rooms_usecase = ScrapeRawRoomsUsecase::new(
            repositories.suumo_repository().to_owned(),
            repositories.room_header_repository().to_owned(),
        );
        let scrape_room_headers_usecase = ScrapeRoomHeadersUsecase::new(
            repositories.suumo_repository().to_owned(),
            repositories.room_header_repository().to_owned(),
        );
        let read_db_for_summary_usecase =
            ReadDbForSummaryUsecase::new(repositories.room_header_repository().to_owned());

        Self {
            health_check_usecase,
            scrape_raw_rooms_usecase,
            scrape_room_headers_usecase,
            read_db_for_summary_usecase,
        }
    }
}
