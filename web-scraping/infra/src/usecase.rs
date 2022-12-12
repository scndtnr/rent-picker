use domain::repository::Repositories;
use usecase::{usecase::SearchRentUsecase, HealthCheckUsecase, Usecases};

use crate::RepositoryImpls;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UsecaseImpls {
    health_check_usecase: HealthCheckUsecase<RepositoryImpls>,
    search_rent_usecase: SearchRentUsecase<RepositoryImpls>,
}

impl Usecases for UsecaseImpls {
    type Repositories = RepositoryImpls;

    fn health_check_usecase(&self) -> &HealthCheckUsecase<Self::Repositories> {
        &self.health_check_usecase
    }
    fn search_rent_usecase(&self) -> &SearchRentUsecase<Self::Repositories> {
        &self.search_rent_usecase
    }
}

impl UsecaseImpls {
    pub fn new(repositories: RepositoryImpls) -> Self {
        let health_check_usecase =
            HealthCheckUsecase::new(repositories.suumo_repository().to_owned());
        let search_rent_usecase =
            SearchRentUsecase::new(repositories.suumo_repository().to_owned());

        Self {
            health_check_usecase,
            search_rent_usecase,
        }
    }
}
