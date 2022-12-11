use domain::repository::Repositories;
use usecase::{HealthCheckUsecase, Usecases};

use crate::RepositoryImpls;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UsecaseImpls {
    health_check_usecase: HealthCheckUsecase<RepositoryImpls>,
}

impl Usecases for UsecaseImpls {
    type Repositories = RepositoryImpls;

    fn health_check_usecase(&self) -> &HealthCheckUsecase<Self::Repositories> {
        &self.health_check_usecase
    }
}

impl UsecaseImpls {
    pub fn new(repositories: RepositoryImpls) -> Self {
        let health_check_usecase =
            HealthCheckUsecase::new(repositories.suumo_repository().to_owned());

        Self {
            health_check_usecase,
        }
    }
}
