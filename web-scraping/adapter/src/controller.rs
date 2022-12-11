use usecase::usecase::Usecases;

use crate::dto::RequestDto;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Controller<U> {
    usecases: U,
}

impl<U: Usecases> Controller<U> {
    pub fn new(usecases: U) -> Self {
        Self { usecases }
    }

    pub async fn health_check_suumo(&self, dto: RequestDto) {
        tracing::debug!("dto: {:#?}", dto);
        self.usecases
            .health_check_usecase()
            .health_check_suumo()
            .await
            .expect("Fail to health check of suumo.");
    }
}
