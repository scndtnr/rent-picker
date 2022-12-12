mod health_check;
mod search_rent;

use domain::repository::Repositories;
pub use health_check::HealthCheckUsecase;
pub use search_rent::SearchRentUsecase;

pub trait Usecases {
    type Repositories: Repositories;

    fn health_check_usecase(&self) -> &HealthCheckUsecase<Self::Repositories>;
    fn search_rent_usecase(&self) -> &SearchRentUsecase<Self::Repositories>;
}
