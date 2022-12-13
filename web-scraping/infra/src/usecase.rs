use domain::repository::Repositories;
use usecase::{usecase::ScrapeRoomsUsecase, HealthCheckUsecase, Usecases};

use crate::RepositoryImpls;

#[derive(Debug, Clone)]
pub struct UsecaseImpls {
    health_check_usecase: HealthCheckUsecase<RepositoryImpls>,
    scrape_rooms_usecase: ScrapeRoomsUsecase<RepositoryImpls>,
}

impl Usecases for UsecaseImpls {
    type Repositories = RepositoryImpls;

    fn health_check_usecase(&self) -> &HealthCheckUsecase<Self::Repositories> {
        &self.health_check_usecase
    }
    fn scrape_rooms_usecase(&self) -> &ScrapeRoomsUsecase<Self::Repositories> {
        &self.scrape_rooms_usecase
    }
}

impl UsecaseImpls {
    pub fn new(repositories: RepositoryImpls) -> Self {
        let health_check_usecase =
            HealthCheckUsecase::new(repositories.suumo_repository().to_owned());
        let scrape_rooms_usecase =
            ScrapeRoomsUsecase::new(repositories.suumo_repository().to_owned());

        Self {
            health_check_usecase,
            scrape_rooms_usecase,
        }
    }
}
