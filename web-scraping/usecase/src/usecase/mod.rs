pub mod health_check;
pub mod read_db_for_summary;
pub mod scrape_room_headers;
pub mod scrape_rooms;

use domain::repository::Repositories;
use health_check::HealthCheckUsecase;
use read_db_for_summary::ReadDbForSummaryUsecase;
use scrape_room_headers::ScrapeRoomHeadersUsecase;
use scrape_rooms::ScrapeRoomsUsecase;

pub trait Usecases {
    type Repositories: Repositories;

    fn health_check_usecase(&self) -> &HealthCheckUsecase<Self::Repositories>;
    fn scrape_rooms_usecase(&self) -> &ScrapeRoomsUsecase<Self::Repositories>;
    fn scrape_room_headers_usecase(&self) -> &ScrapeRoomHeadersUsecase<Self::Repositories>;
    fn read_db_for_summary_usecase(&self) -> &ReadDbForSummaryUsecase<Self::Repositories>;
}
