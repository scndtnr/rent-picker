mod health_check;
mod scrape_rooms;

use domain::repository::Repositories;
pub use health_check::HealthCheckUsecase;
pub use scrape_rooms::ScrapeRoomsUsecase;

pub trait Usecases {
    type Repositories: Repositories;

    fn health_check_usecase(&self) -> &HealthCheckUsecase<Self::Repositories>;
    fn scrape_rooms_usecase(&self) -> &ScrapeRoomsUsecase<Self::Repositories>;
}
