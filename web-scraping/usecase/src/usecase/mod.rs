mod health_check;

use domain::repository::Repositories;
pub use health_check::HealthCheckUsecase;

pub trait Usecases {
    type Repositories: Repositories;

    fn health_check_usecase(&self) -> &HealthCheckUsecase<Self::Repositories>;
}
